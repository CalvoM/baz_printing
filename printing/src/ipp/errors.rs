use thiserror::Error;

#[derive(Error, Debug)]
pub enum IPPClientError {
    #[error("{0}")]
    SetupError(String),
    #[error("{0}")]
    SendPrintJobError(String),
    #[error("{0}")]
    ByteParsingError(String),
    #[error("{0}")]
    TransportError(String),
    #[error("{0}")]
    GetPrinterAttributesError(String),
}

impl serde::Serialize for IPPClientError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
