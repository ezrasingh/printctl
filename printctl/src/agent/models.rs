use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GcodeFile {
    pub id: Uuid,
    pub name: std::ffi::OsString,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub printer_id: Uuid,
    pub gcode_file_id: Uuid,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct JobLogEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

// Helper to split G-code file into lines for printer task
impl Job {
    pub fn gcode_lines(&self) -> Vec<String> {
        // For simplicity, you could fetch G-code content from the agent's gcode_files map
        vec!["G28 ; home all axes".to_string()] // placeholder
    }
}
