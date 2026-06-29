use reqwest::{
    blocking::Client,
    header::{self, InvalidHeaderValue, CONTENT_TYPE},
    Url,
};

use crate::ipp::{
    errors::IPPClientError,
    utils::{IPPOperationRequest, OperationID},
};
#[repr(C)]
pub struct SendPrintJobPayload {
    version: u16,
}

pub struct IPPClient {
    server_host: Url,
    transport: Client,
}

impl IPPClient {
    pub fn try_new(server_host: &str) -> Result<Self, IPPClientError> {
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
    pub fn send_print_job(&mut self) {
        let operation_id = OperationID::PrintJob as u16;
        let data = IPPOperationRequest {
            version: 0x0101,
            operation_id,
            request_id: 0x00000001,
        };
        let mut expected_data = Vec::new();
        expected_data.extend_from_slice(&data.version.to_be_bytes());
        expected_data.extend_from_slice(&data.operation_id.to_be_bytes());
        expected_data.extend_from_slice(&data.request_id.to_be_bytes());
        self.transport
            .post(self.server_host.clone())
            .body(expected_data)
            .send()
            .unwrap();
    }
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
        // Rewrite ipp:// -> http:// before parsing; url crate can't switch between
        // non-special (ipp) and special (http) schemes via set_scheme.
        let (http_url, default_port) = if let Some(rest) = raw_url.strip_prefix("ipps://") {
            (format!("https://{rest}"), 631u16)
        } else if let Some(rest) = raw_url.strip_prefix("ipp://") {
            (format!("http://{rest}"), 631u16)
        } else {
            return Err(IPPClientError::SetupError(format!(
                "invalid scheme in '{}': expected ipp or ipps",
                raw_url
            )));
        };
        let mut parsed =
            Url::parse(&http_url).map_err(|e| IPPClientError::SetupError(e.to_string()))?;
        if parsed.host().is_none() {
            return Err(IPPClientError::SetupError(
                "IPP URL must specify a host".into(),
            ));
        }
        if parsed.port().is_none() {
            parsed
                .set_port(Some(default_port))
                .map_err(|_| IPPClientError::SetupError("failed to set default port".into()))?;
        }
        Ok(parsed)
    }
}
