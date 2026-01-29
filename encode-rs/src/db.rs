use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct Job {
    pub id: Uuid,
    pub status: String,
    pub input_path: String,
    pub output_path: String,
    pub video_codec: String,
    pub preset: String,
    pub crf: i32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Run migrations - suppress NOTICE messages to avoid "relation already exists" spam
        let mut tx = pool.begin().await?;
        sqlx::query("SET LOCAL client_min_messages = WARNING")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn add_job(
        &self,
        input_path: &Path,
        output_path: &Path,
        codec: Option<&str>,
        preset: Option<&str>,
        crf: Option<i32>,
    ) -> Result<Uuid> {
        let input_str = input_path.to_string_lossy();
        let output_str = output_path.to_string_lossy();

        let row = sqlx::query(
            r#"
            INSERT INTO jobs (input_path, output_path, video_codec, preset, crf)
            VALUES ($1, $2, COALESCE($3, 'libx264'), COALESCE($4, 'medium'), COALESCE($5, 23))
            RETURNING id
            "#,
        )
        .bind(&*input_str)
        .bind(&*output_str)
        .bind(codec)
        .bind(preset)
        .bind(crf)
        .fetch_one(&self.pool)
        .await?;

        let id: Uuid = row.try_get("id")?;
        Ok(id)
    }

    pub async fn claim_next_job(&self) -> Result<Option<Job>> {
        let mut tx = self.pool.begin().await?;

        // Use SELECT FOR UPDATE to prevent race conditions
        let row = sqlx::query(
            r#"
            SELECT id, status, input_path, output_path, video_codec, preset, crf,
                   error_message, created_at, started_at, completed_at
            FROM jobs
            WHERE status = 'pending'
            ORDER BY created_at ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut *tx)
        .await?;

        let job = match row {
            Some(row) => {
                let id: Uuid = row.try_get("id")?;

                // Mark as processing
                sqlx::query(
                    r#"
                    UPDATE jobs
                    SET status = 'processing', started_at = NOW()
                    WHERE id = $1
                    "#,
                )
                .bind(id)
                .execute(&mut *tx)
                .await?;

                Some(Job {
                    id,
                    status: "processing".to_string(),
                    input_path: row.try_get("input_path")?,
                    output_path: row.try_get("output_path")?,
                    video_codec: row.try_get("video_codec")?,
                    preset: row.try_get("preset")?,
                    crf: row.try_get("crf")?,
                    error_message: row.try_get("error_message")?,
                    created_at: row.try_get("created_at")?,
                    started_at: row.try_get("started_at")?,
                    completed_at: row.try_get("completed_at")?,
                })
            }
            None => None,
        };

        tx.commit().await?;
        Ok(job)
    }

    pub async fn mark_done(&self, job_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE jobs
            SET status = 'done', completed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn mark_failed(&self, job_id: Uuid, error_message: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE jobs
            SET status = 'failed', error_message = $2, completed_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(job_id)
        .bind(error_message)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn has_pending_jobs(&self) -> Result<bool> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM jobs WHERE status = 'pending'
            ) as has_jobs
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let has_jobs: bool = row.try_get("has_jobs")?;
        Ok(has_jobs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_job() -> Job {
        Job {
            id: Uuid::new_v4(),
            status: "pending".to_string(),
            input_path: "/input.mp4".to_string(),
            output_path: "/output.mp4".to_string(),
            video_codec: "libx264".to_string(),
            preset: "medium".to_string(),
            crf: 23,
            error_message: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }

    #[test]
    fn test_job_creation() {
        let job = create_test_job();

        assert_eq!(job.input_path, "/input.mp4");
        assert_eq!(job.output_path, "/output.mp4");
        assert_eq!(job.status, "pending");
        assert_eq!(job.video_codec, "libx264");
        assert_eq!(job.preset, "medium");
        assert_eq!(job.crf, 23);
        assert!(job.error_message.is_none());
        assert!(job.started_at.is_none());
        assert!(job.completed_at.is_none());
    }

    #[test]
    fn test_job_with_error() {
        let mut job = create_test_job();
        job.status = "failed".to_string();
        job.error_message = Some("FFmpeg error".to_string());

        assert_eq!(job.status, "failed");
        assert_eq!(job.error_message, Some("FFmpeg error".to_string()));
    }

    #[test]
    fn test_job_processing_state() {
        let mut job = create_test_job();
        job.status = "processing".to_string();
        job.started_at = Some(Utc::now());

        assert_eq!(job.status, "processing");
        assert!(job.started_at.is_some());
    }

    #[test]
    fn test_job_completed_state() {
        let mut job = create_test_job();
        job.status = "done".to_string();
        job.started_at = Some(Utc::now());
        job.completed_at = Some(Utc::now());

        assert_eq!(job.status, "done");
        assert!(job.started_at.is_some());
        assert!(job.completed_at.is_some());
    }
}
