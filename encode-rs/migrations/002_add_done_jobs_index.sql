-- Add index for efficient cleanup of old done jobs
CREATE INDEX idx_jobs_status_completed ON jobs(status, completed_at) 
WHERE status = 'done';
