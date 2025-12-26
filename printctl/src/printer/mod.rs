pub mod state;

use crate::prelude::*;
use std::{collections::VecDeque, sync::Arc};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};
use tokio_serial::SerialPortBuilderExt;

use crate::agent::models;
use state::PrinterState;

#[derive(Debug)]
pub enum PrinterCommand {
    Write(Vec<u8>, oneshot::Sender<Result<()>>),
    ReadLine(oneshot::Sender<Result<String>>),
    QueueJob(models::Job),
    StartNextJob,
}

#[derive(Debug)]
pub struct Printer {
    pub tag: Option<String>,
    pub port_path: String,

    // printer internal state
    pub state: Arc<Mutex<PrinterState>>,

    // queued print jobs
    pub job_queue: Arc<Mutex<VecDeque<models::Job>>>,

    // command channel TO worker
    cmd_tx: mpsc::Sender<PrinterCommand>,

    // raw serial lines FROM worker
    pub serial_rx: broadcast::Sender<String>,

    // serial connection (owned only by worker)
    connection: Arc<Mutex<Option<tokio_serial::SerialStream>>>,
}

impl Printer {
    pub async fn new(path: &str, baud: u32, tag: Option<String>) -> Result<Self> {
        let serial = tokio_serial::new(path, baud).open_native_async()?;

        // mpsc command channel to worker
        let (cmd_tx, cmd_rx) = mpsc::channel(32);

        // broadcast for serial lines (observers subscribe)
        let (serial_tx, _) = broadcast::channel(256);

        let printer = Self {
            tag,
            port_path: path.to_string(),

            state: Arc::new(Mutex::new(PrinterState::default())),
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            connection: Arc::new(Mutex::new(Some(serial))),

            cmd_tx,
            serial_rx: serial_tx.clone(),
        };

        printer.spawn_worker(cmd_rx, serial_tx);

        Ok(printer)
    }

    fn spawn_worker(
        &self,
        mut cmd_rx: mpsc::Receiver<PrinterCommand>,
        serial_tx: broadcast::Sender<String>,
    ) {
        let connection = self.connection.clone();
        let state = self.state.clone();
        let job_queue = self.job_queue.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let mut line_buf = Vec::new();

            loop {
                let mut guard = connection.lock().await;
                let serial = match guard.as_mut() {
                    Some(s) => s,
                    None => {
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                        continue;
                    }
                };

                tokio::select! {
                    // SERIAL READ
                    result = serial.read(&mut buf) => {
                        let Ok(n) = result else { continue };
                        if n == 0 { continue };

                        for byte in &buf[..n] {
                            if *byte == b'\n' {
                                let line = String::from_utf8_lossy(&line_buf).trim().to_string();
                                let _ = serial_tx.send(line.clone());

                                // update printer state machine
                                {
                                    let mut st = state.lock().await;
                                    st.update_from_line(&line);
                                }

                                line_buf.clear();
                            } else {
                                line_buf.push(*byte);
                            }
                        }
                    }

                    // COMMAND HANDLING
                    Some(cmd) = cmd_rx.recv() => {
                        match cmd {
                            PrinterCommand::Write(data, respond) => {
                                let res = async {
                                    serial.write_all(&data).await?;
                                    serial.flush().await?;
                                    Ok::<(), std::io::Error>(())
                                }.await;

                                let _ = respond.send(res.map_err(|e| e.into()));

                            }

                            PrinterCommand::ReadLine(respond) => {
                                let mut sub = serial_tx.subscribe();
                                match sub.recv().await {
                                    Ok(line) => {
                                        let _ = respond.send(Ok(line));
                                    }
                                    Err(e) => {
                                        let _ = respond.send(Err(Error::IO(std::io::Error::new(
                                            std::io::ErrorKind::Other,
                                            e.to_string(),
                                        ))));
                                    }
                                }
                            }

                            PrinterCommand::QueueJob(job) => {
                                job_queue.lock().await.push_back(job);
                            }

                            PrinterCommand::StartNextJob => {
                                if let Some(job) = job_queue.lock().await.pop_front() {
                                    println!("Starting job: {:?}", job);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

impl Printer {
    pub async fn write(&self, data: Vec<u8>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx.send(PrinterCommand::Write(data, tx)).await?;
        rx.await?
    }

    pub async fn read_line(&self) -> Result<String> {
        let (tx, rx) = oneshot::channel();
        self.cmd_tx.send(PrinterCommand::ReadLine(tx)).await?;
        rx.await?
    }

    pub async fn queue_job(&self, job: models::Job) -> Result<()> {
        self.cmd_tx
            .send(PrinterCommand::QueueJob(job))
            .await
            .map_err(|e| Error::IO(std::io::Error::new(std::io::ErrorKind::BrokenPipe, e)))
    }

    pub async fn start_next_job(&self) -> Result<()> {
        self.cmd_tx
            .send(PrinterCommand::StartNextJob)
            .await
            .map_err(|e| Error::IO(std::io::Error::new(std::io::ErrorKind::BrokenPipe, e)))
    }

    /// Get a live stream of raw lines from the printer
    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.serial_rx.subscribe()
    }
}
