use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use crate::errors::LPDPClientError::{self};
use crate::utils::{DaemonCommand, ReceiveJobSubCommand};

pub struct LPDPClient {
    pub queue_name: String,
    pub server_host: String,
    stream: TcpStream,
}

impl LPDPClient {
    pub fn try_new(queue_name: &str, server_host: &str) -> Result<Self, LPDPClientError> {
        let stream = TcpStream::connect(format!("{server_host}:515"))
            .map_err(|e| LPDPClientError::UnreachableServer(e.to_string()))?;
        Ok(LPDPClient {
            queue_name: queue_name.to_string(),
            server_host: server_host.to_string(),
            stream,
        })
    }
    pub fn print_remaining_jobs(&mut self) -> Result<(), LPDPClientError> {
        let job_cmd = [
            &[DaemonCommand::PrintRemainingJobs as u8][..],
            self.queue_name.as_bytes(),
            b"\n",
        ]
        .concat();
        self.stream
            .write_all(&job_cmd)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        Ok(())
    }
    pub fn send_printer_job(&mut self, file_path: &Path) -> Result<(), LPDPClientError> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("document");
        let data =
            std::fs::read(file_path).map_err(|e| LPDPClientError::FileReadError(e.to_string()))?;

        let job_cmd: Vec<u8> = [
            &[DaemonCommand::ReceivePrinterJob as u8][..],
            self.queue_name.as_bytes(),
            b"\n",
        ]
        .concat();
        self.stream
            .write_all(&job_cmd)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        self.send_printer_control_file(file_name)?;
        self.send_printer_data_file(&data)?;
        Ok(())
    }
    pub fn abort_printer_job(&mut self) -> Result<(), LPDPClientError> {
        let job_cmd: Vec<u8> = [
            &[DaemonCommand::ReceivePrinterJob as u8][..],
            self.queue_name.as_bytes(),
            b"\n",
        ]
        .concat();
        self.stream
            .write_all(&job_cmd)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        self.send_printer_abort_job()?;
        Ok(())
    }
    pub fn request_queue_start_short() {}
    pub fn request_queue_start_long() {}
    pub fn request_job_removal() {}
    fn send_printer_abort_job(&mut self) -> Result<(), LPDPClientError> {
        let job_cmd: Vec<u8> = [&[ReceiveJobSubCommand::Abort as u8][..], b"\n"].concat();
        self.stream
            .write_all(&job_cmd)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        Ok(())
    }
    fn send_printer_control_file(&mut self, file_name: &str) -> Result<(), LPDPClientError> {
        let binding = self.build_control_file_content(file_name)?;
        let control_content = binding.as_bytes();
        let binding = self.build_control_file_name()?;
        let cf_name = binding.as_bytes();
        let cf_count = control_content.len().to_string();
        let job_cmd = [
            &[ReceiveJobSubCommand::ReceiveControlFile as u8][..],
            cf_count.as_bytes(),
            b" ",
            cf_name,
            b"\n",
        ]
        .concat();
        self.stream
            .write_all(&job_cmd)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        self.stream
            .write_all(control_content)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.stream
            .write_all(&[0x00u8])
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        Ok(())
    }

    fn build_control_file_content(&self, file_name: &str) -> Result<String, LPDPClientError> {
        let hostname =
            whoami::hostname().map_err(|e| LPDPClientError::SystemDetailsError(e.to_string()))?;
        let username =
            whoami::username().map_err(|e| LPDPClientError::SystemDetailsError(e.to_string()))?;
        let job_number = "001";
        Ok(format!(
            "H{hostname}\nP{username}\nJ{file_name}\nldfA{job_number}{hostname}\n"
        ))
    }

    fn build_control_file_name(&self) -> Result<String, LPDPClientError> {
        let hostname =
            whoami::hostname().map_err(|e| LPDPClientError::SystemDetailsError(e.to_string()))?;
        let job_number = "001";
        Ok(format!("cfA{job_number}{hostname}"))
    }

    fn build_data_file_name(&self) -> Result<String, LPDPClientError> {
        let hostname =
            whoami::hostname().map_err(|e| LPDPClientError::SystemDetailsError(e.to_string()))?;
        let job_number = "001";
        Ok(format!("dfA{job_number}{hostname}"))
    }
    fn send_printer_data_file(&mut self, data_content: &[u8]) -> Result<(), LPDPClientError> {
        let binding = self.build_data_file_name()?;
        let df_name = binding.as_bytes();
        let df_count = data_content.len().to_string();
        let job_cmd = [
            &[ReceiveJobSubCommand::ReceiveDataFile as u8][..],
            df_count.as_bytes(),
            b" ",
            df_name,
            b"\n",
        ]
        .concat();
        self.stream
            .write_all(&job_cmd)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        self.stream
            .write_all(data_content)
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.stream
            .write_all(&[0x00u8])
            .map_err(|e| LPDPClientError::FailedWrite(e.to_string()))?;
        self.read_ack()?;
        Ok(())
    }
    fn read_ack(&mut self) -> Result<(), LPDPClientError> {
        let mut buf = [0u8; 1];
        let n = self
            .stream
            .read(&mut buf)
            .map_err(|e| LPDPClientError::FailedRead(e.to_string()))?;
        if n != 1 {
            return Err(LPDPClientError::NotAcknowledged(format!(
                "Expected a single ACK byte found {n}"
            )));
        }
        if buf[0] != 0x00 {
            return Err(LPDPClientError::NotAcknowledged(format!(
                "expected ACK 0x00, got 0x{:02X}",
                buf[0]
            )));
        }
        Ok(())
    }
}
