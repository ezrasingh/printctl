pub mod models;

use crate::prelude::*;
use std::collections::{HashMap, VecDeque};

use chrono::Utc;
use tokio_serial::SerialPortInfo;
use uuid::Uuid;

use crate::printer::Printer;

#[derive(Default)]
pub struct PrintAgent {
    name: String,
    printers: HashMap<String, Printer>,
    gcode_files: HashMap<Uuid, models::GcodeFile>,
    jobs: HashMap<Uuid, models::Job>,
    job_queue: VecDeque<Uuid>,
    job_logs: HashMap<Uuid, Vec<models::JobLogEntry>>,
}

impl PrintAgent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            printers: HashMap::new(),
            gcode_files: HashMap::new(),
            jobs: HashMap::new(),
            job_queue: VecDeque::new(),
            job_logs: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn available_devices(&self) -> Result<Vec<SerialPortInfo>> {
        let ports = tokio_serial::available_ports()?;
        Ok(ports)
    }

    pub async fn start_printer(
        &mut self,
        name: &str,
        port: SerialPortInfo,
        baud: u32,
    ) -> Result<()> {
        let port_path = &port.port_name;
        let printer = Printer::new(port_path, baud, Some(name.to_string())).await?;
        self.printers.insert(name.into(), printer);
        Ok(())
    }

    pub fn upload_gcode(&mut self, name: &std::ffi::OsStr, content: Vec<u8>) -> Uuid {
        let g = models::GcodeFile {
            id: Uuid::new_v4(),
            name: name.to_os_string(),
            content,
        };
        let id = g.id;
        self.gcode_files.insert(id, g);
        id
    }

    pub fn create_job(&mut self, printer_id: Uuid, gcode_file_id: Uuid) -> Uuid {
        let job = models::Job {
            id: Uuid::new_v4(),
            printer_id,
            gcode_file_id,
            status: models::JobStatus::Queued,
            created_at: Utc::now(),
            started_at: None,
            finished_at: None,
        };
        let id = job.id;
        self.jobs.insert(id, job);
        self.job_queue.push_back(id);
        id
    }

    pub fn list_jobs(&self) -> std::collections::hash_map::Values<'_, Uuid, models::Job> {
        self.jobs.values()
    }

    pub fn get_job(&self, job_id: Uuid) -> Option<&models::Job> {
        self.jobs.get(&job_id)
    }

    pub fn get_job_logs(&self, job_id: Uuid) -> Option<&Vec<models::JobLogEntry>> {
        self.job_logs.get(&job_id)
    }
}
