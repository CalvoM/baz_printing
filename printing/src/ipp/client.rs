use reqwest::{
    blocking::Client,
    header::{self, InvalidHeaderValue, CONTENT_TYPE},
    Url,
};

use crate::ipp::errors::IPPClientError;
#[repr(C)]
pub struct SendPrintJobPayload {
    version: u16,
}

pub struct IPPClient {
    server_host: Url,
    transport: Client,
}

impl IPPClient {
    pub fn try_new(server_host: &str, queue_name: &str) -> Result<Self, IPPClientError> {
        let server_host = Self::parse_ipp_url(server_host)?;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/ipp"
                .parse()
                .map_err(|e: InvalidHeaderValue| IPPClientError::SetupError(e.to_string()))?,
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        Ok(Self {
            server_host,
            transport: client,
        })
    }
    pub fn send_print_job(&mut self) {}
    pub fn send_print_uri(&self) {}
    pub fn validate_job(&self) {}
    pub fn create_job(&self) {}
    pub fn get_printer_attributes(&self) {}
    pub fn get_jobs(&self) {}
    pub fn pause_printer(&self) {}
    pub fn resume_printer(&self) {}
    pub fn purge_jobs(&self) {}
    fn parse_ipp_url(raw_url: &str) -> Result<Url, IPPClientError> {
        if raw_url.len() > 1023 {
            return Err(IPPClientError::SetupError(
                "IPP URL exceeds 1023 octet limit".into(),
            ));
        }
        let mut parsed =
            Url::parse(raw_url).map_err(|e| IPPClientError::SetupError(e.to_string()))?;
        if !matches!(parsed.scheme(), "ipp" | "ipps") {
            return Err(IPPClientError::SetupError(format!(
                "invalid scheme '{}': expected ipp or ipps",
                parsed.scheme()
            )));
        }
        if parsed.host().is_none() {
            return Err(IPPClientError::SetupError(
                "IPP URL must specify a host".into(),
            ));
        }
        if parsed.port().is_none() {
            parsed
                .set_port(Some(631))
                .map_err(|_| IPPClientError::SetupError("failed to set default port".into()))?;
        }
        Ok(parsed)
    }

    fn ipp_port(url: &Url) -> u16 {
        url.port().unwrap_or(631)
    }

    fn ipp_path(url: &Url) -> &str {
        let path = url.path();
        if path.is_empty() {
            "/"
        } else {
            path
        }
    }
}
