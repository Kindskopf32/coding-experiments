CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    input_path TEXT NOT NULL,
    output_path TEXT NOT NULL,
    video_codec TEXT NOT NULL DEFAULT 'libx264',
    preset TEXT NOT NULL DEFAULT 'medium',
    crf INTEGER NOT NULL DEFAULT 23,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_jobs_status_created ON jobs(status, created_at) 
WHERE status = 'pending';
