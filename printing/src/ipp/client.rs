use reqwest;

pub struct IPPClient {
    pub server_host: String,
    pub queue_name: String,
}

impl IPPClient {
    pub fn send_print_job(&self) {}
    pub fn send_print_uri(&self) {}
    pub fn validate_job(&self) {}
    pub fn create_job(&self) {}
    pub fn get_printer_attributes(&self) {}
    pub fn get_jobs(&self) {}
    pub fn pause_printer(&self) {}
    pub fn resume_printer(&self) {}
    pub fn purge_jobs(&self) {}
}
