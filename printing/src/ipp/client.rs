use reqwest::{
    blocking::Client,
    header::{self, InvalidHeaderValue, CONTENT_TYPE},
    Url,
};

use crate::ipp::{
    errors::IPPClientError,
    utils::{
        pack_attribute_with_one_value, pack_byte_ipp, pack_u16_to_u32, pack_u8_to_u16,
        AttributeGroupTags, IPPOperationRequestBase, NetworkPackable, OperationID, RequestID,
        ResponseStatusCode, ValueTags, IPP_CONTENT_TYPE, SUPPORTED_VERSION,
    },
};

#[repr(C)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct SendPrintJobRequest {
    base: IPPOperationRequestBase,
}

impl NetworkPackable for SendPrintJobRequest {}

#[repr(C)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct GetPrinterAttributesRequest {
    base: IPPOperationRequestBase,
}

impl NetworkPackable for GetPrinterAttributesRequest {}

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
            IPP_CONTENT_TYPE
                .parse()
                .map_err(|e: InvalidHeaderValue| IPPClientError::SetupError(e.to_string()))?,
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        Ok(Self {
            server_host,
            transport: client,
        })
    }
    pub fn send_print_job(&mut self) -> Result<(), IPPClientError> {
        let operation_id = OperationID::PrintJob as u16;
        let request_id = RequestID::PrintJob as u32;
        let base_data = IPPOperationRequestBase {
            version: SUPPORTED_VERSION,
            operation_id,
            request_id,
        };
        let data = SendPrintJobRequest { base: base_data };
        let mut expected_data = data.to_bytes()?;
        self.send_bytes_to_printer(expected_data)?;
        Ok(())
    }
    pub fn send_print_uri(&self) {}
    pub fn validate_job(&self) {}
    pub fn create_job(&self) {}

    /// Request the values of the attributes of a Printer
    /// [`Reference`]: https://www.rfc-editor.org/info/rfc8011/#section-4.2.5
    /// This is a `required` operation.
    pub fn get_printer_attributes(&mut self) -> Result<(), IPPClientError> {
        let operation_id = OperationID::GetPrinterAttributes as u16;
        let request_id = RequestID::GetPrinterAttributes as u32;
        let base_data = IPPOperationRequestBase {
            version: SUPPORTED_VERSION,
            operation_id,
            request_id,
        };
        let data = GetPrinterAttributesRequest { base: base_data };
        let mut expected_data = data.to_bytes()?;
        let ops_attr_tag = AttributeGroupTags::OperationAttributesTag as u8;
        expected_data.push(ops_attr_tag);
        let attrs_bytes =
            pack_attribute_with_one_value(ValueTags::Charset, "attributes-charset", "utf-8");
        expected_data.extend_from_slice(&attrs_bytes);
        let attrs_bytes = pack_attribute_with_one_value(
            ValueTags::NaturalLanguage,
            "attributes-natural-language",
            "en",
        );
        expected_data.extend_from_slice(&attrs_bytes);
        let attrs_bytes =
            pack_attribute_with_one_value(ValueTags::URI, "printer-uri", self.server_host.as_str());
        expected_data.extend_from_slice(&attrs_bytes);
        let attrs_bytes =
            pack_attribute_with_one_value(ValueTags::NameWithoutLanguage, "jobname", "test");
        expected_data.extend_from_slice(&attrs_bytes);
        // last_byte
        let end_tag = AttributeGroupTags::EndOfAttributesTag as u8;
        expected_data.push(end_tag);
        let response = self.send_bytes_to_from_printer(expected_data)?;
        let printer_attributes_bytes = self.parse_response(response, SUPPORTED_VERSION, request_id);
        println!("{:#?}", printer_attributes_bytes);

        Ok(())
    }
    pub fn get_jobs(&self) {}
    pub fn pause_printer(&self) {}
    pub fn resume_printer(&self) {}
    pub fn purge_jobs(&self) {}
    fn send_bytes_to_printer(&mut self, data: Vec<u8>) -> Result<(), IPPClientError> {
        self.transport
            .post(self.server_host.clone())
            .body(data)
            .send()
            .unwrap();
        Ok(())
    }
    fn send_bytes_to_from_printer(&mut self, data: Vec<u8>) -> Result<Vec<u8>, IPPClientError> {
        let res = self
            .transport
            .post(self.server_host.clone())
            .body(data)
            .send()
            .map_err(|e| IPPClientError::TransportError(e.to_string()))?;
        let res_bytes = res
            .bytes()
            .map_err(|e| IPPClientError::TransportError(e.to_string()))?
            .to_vec();
        Ok(res_bytes)
    }

    fn parse_response(
        &mut self,
        response: Vec<u8>,
        expected_version_number: u16,
        expected_request_id: u32,
    ) -> Vec<u8> {
        let version_number = pack_u8_to_u16(response[0], response[1]);
        if version_number != expected_version_number {
            panic!(
                "Versions numbers do not match: Received {:X}, Expected {:X}",
                version_number, expected_version_number
            );
        }
        let status_code = pack_u8_to_u16(response[2], response[3]);
        let ok_code = ResponseStatusCode::SuccessfulOk as u16;
        if status_code != ok_code {
            match ResponseStatusCode::try_from(status_code) {
                Ok(status) => panic!("We did not receive Ok StatusCode, we received: {}", status),
                Err(code) => panic!(
                    "We did not receive Ok StatusCode, we received unknown code: {:X}",
                    code
                ),
            }
        }
        let upper_request_id = pack_u8_to_u16(response[4], response[5]);
        let lower_request_id = pack_u8_to_u16(response[6], response[7]);
        let request_id = pack_u16_to_u32(upper_request_id, lower_request_id);
        if request_id != expected_request_id {
            panic!(
                "We did not receive the expected request ID, we received: {:X}",
                request_id
            );
        }
        response[8..response.len()].to_vec()
    }
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
