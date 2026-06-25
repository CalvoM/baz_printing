use thiserror::Error;

#[derive(Error, Debug)]
pub enum IPPClientError {
    #[error("{0}")]
    SetupError(String),
}

impl serde::Serialize for IPPClientError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
