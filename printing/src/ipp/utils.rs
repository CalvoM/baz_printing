//An operation request or response is encoded as follows:

//-----------------------------------------------
//|                  version-number             |   2 bytes  - required
//-----------------------------------------------
//|               operation-id (request)        |
//|                      or                     |   2 bytes  - required
//|               status-code (response)        |
//-----------------------------------------------
//|                   request-id                |   4 bytes  - required
//-----------------------------------------------
//|                 attribute-group             |   n bytes - 0 or more
//-----------------------------------------------
//|              end-of-attributes-tag          |   1 byte   - required
//-----------------------------------------------
//|                     data                    |   q bytes  - optional
//-----------------------------------------------
//
//From the standpoint of a parser that performs an action based on a
//"tag" value, the encoding consists of:

//-----------------------------------------------
//|                  version-number             |   2 bytes  - required
//-----------------------------------------------
//|               operation-id (request)        |
//|                      or                     |   2 bytes  - required
//|               status-code (response)        |
//-----------------------------------------------
//|                   request-id                |   4 bytes  - required
//-----------------------------------------------------------
//|        tag (delimiter-tag or value-tag)     |   1 byte  |
//-----------------------------------------------           |-0 or more
//|           empty or rest of attribute        |   x bytes |
//-----------------------------------------------------------
//|              end-of-attributes-tag          |   1 byte   - required
//-----------------------------------------------
//|                     data                    |   y bytes  - optional
//-----------------------------------------------

use std::u8;

use crate::ipp::errors::IPPClientError;

pub const SUPPORTED_VERSION: u16 = 0x0101; // 1.1
pub const IPP_CONTENT_TYPE: &str = "application/ipp";

#[repr(C)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct IPPOperationRequestBase {
    pub version: u16,
    pub operation_id: u16,
    pub request_id: u32,
}

#[repr(C)]
#[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive)]
#[rkyv(derive(Debug))]
pub struct IPPOperationResponseBase {
    pub version: u16,
    pub status_code: u16,
    pub request_id: u32,
}

pub trait NetworkPackable
where
    Self: for<'a> rkyv::Serialize<
            rkyv::rancor::Strategy<
                rkyv::ser::Serializer<
                    rkyv::util::AlignedVec,
                    rkyv::ser::allocator::ArenaHandle<'a>,
                    rkyv::ser::sharing::Share,
                >,
                rkyv::rancor::Error,
            >,
        >,
{
    fn to_bytes(&self) -> Result<Vec<u8>, IPPClientError>
    where
        Self: Sized,
    {
        Ok(rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|e| IPPClientError::ByteParsingError(e.to_string()))?
            .into_vec())
    }
}

pub fn pack_attribute_with_one_value(value_tag: ValueTags, name: &str, value: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let parsed_tag = value_tag as u8;
    buf.push(parsed_tag);
    let name_length = name.len() as u16;
    buf.extend_from_slice(&name_length.to_be_bytes());
    buf.extend_from_slice(name.as_bytes());
    let value_length = value.len() as u16;
    buf.extend_from_slice(&value_length.to_be_bytes());
    buf.extend_from_slice(value.as_bytes());
    buf
}
pub fn pack_byte_ipp<S>(data: S) -> Result<Vec<u8>, IPPClientError>
where
    S: for<'a> rkyv::Serialize<
            rkyv::rancor::Strategy<
                rkyv::ser::Serializer<
                    rkyv::util::AlignedVec,
                    rkyv::ser::allocator::ArenaHandle<'a>,
                    rkyv::ser::sharing::Share,
                >,
                rkyv::rancor::Error,
            >,
        >,
{
    Ok(rkyv::to_bytes::<rkyv::rancor::Error>(&data)
        .map_err(|e| IPPClientError::SendPrintJobError(e.to_string()))?
        .into_vec())
}

//Each "attribute-group" field is encoded as follows:

//-----------------------------------------------
//|           begin-attribute-group-tag         |  1 byte
//----------------------------------------------------------
//|                   attribute                 |  p bytes |- 0 or more
//----------------------------------------------------------

//An "attribute-group" field contains zero or more "attribute" fields.

//Note that the values of the "begin-attribute-group-tag" field and the
//"end-of-attributes-tag" field are called "delimiter-tags".

pub enum AttributeGroupTags {
    OperationAttributesTag = 0x01,
    JobAttributesTag = 0x02,
    EndOfAttributesTag = 0x03, // Occurs exactly once in an operation and must be last attribute
    // group tab
    PrinterAttributesTag = 0x04,
    UnsupportedAttributesTag = 0x05,
    FutureGroupTags = 0x06,
}
impl std::fmt::Display for AttributeGroupTags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AttributeGroupTags::OperationAttributesTag => {
                write!(f, "operation-attributes-tag")
            }
            AttributeGroupTags::JobAttributesTag => {
                write!(f, "job-attributes-tag")
            }
            AttributeGroupTags::EndOfAttributesTag => {
                write!(f, "end-of-attributes-tag")
            }
            AttributeGroupTags::PrinterAttributesTag => {
                write!(f, "printer-attributes-tag")
            }
            AttributeGroupTags::UnsupportedAttributesTag => {
                write!(f, "unsupported-attributes-tag")
            }
            AttributeGroupTags::FutureGroupTags => {
                write!(f, "future-group-tag")
            }
        }
    }
}
impl std::convert::TryFrom<u8> for AttributeGroupTags {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(AttributeGroupTags::OperationAttributesTag),
            0x02 => Ok(AttributeGroupTags::JobAttributesTag),
            0x03 => Ok(AttributeGroupTags::EndOfAttributesTag),
            0x04 => Ok(AttributeGroupTags::PrinterAttributesTag),
            0x05 => Ok(AttributeGroupTags::UnsupportedAttributesTag),
            0x06 => Ok(AttributeGroupTags::FutureGroupTags),
            0_u8 | 7_u8..=u8::MAX => Err("Unsupported tag."),
        }
    }
}

pub enum ModelDocumentGroupAttributeTag {
    OperationAttributes,
    JobTemplateAttributes,
    JobObjectAttributes,
    UnsupportedAttributes,
}

impl std::fmt::Display for ModelDocumentGroupAttributeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModelDocumentGroupAttributeTag::OperationAttributes => {
                write!(f, "operations-attributes-tag")
            }
            ModelDocumentGroupAttributeTag::JobTemplateAttributes => {
                write!(f, "job-attributes-tag")
            }
            ModelDocumentGroupAttributeTag::JobObjectAttributes => write!(f, "job-attributes-tag"),
            ModelDocumentGroupAttributeTag::UnsupportedAttributes => {
                write!(f, "unsupported-attributes-tag")
            }
        }
    }
}

//An "attribute" field is encoded as follows:

//-----------------------------------------------
//|          attribute-with-one-value           |  q bytes
//----------------------------------------------------------
//|             additional-value                |  r bytes |- 0 or more
//----------------------------------------------------------

//When an attribute is single valued (e.g., "copies" with a value of
//10) or multi-valued with one value (e.g., "sides-supported" with just
//the value 'one-sided'), it is encoded with just an "attribute-with-
//one-value" field.  When an attribute is multi-valued with n values
//(e.g., "sides-supported" with the values 'one-sided' and 'two-sided-
//long-edge'), it is encoded with an "attribute-with-one-value" field
//followed by n-1 "additional-value" fields.

//Each "attribute-with-one-value" field is encoded as follows:

//-----------------------------------------------
//|                   value-tag                 |   1 byte
//-----------------------------------------------
//|               name-length  (value is u)     |   2 bytes
//-----------------------------------------------
//|                     name                    |   u bytes
//-----------------------------------------------
//|              value-length  (value is v)     |   2 bytes
//-----------------------------------------------
//|                     value                   |   v bytes
//-----------------------------------------------

//An "attribute-with-one-value" field is encoded with five subfields:

//o  The "value-tag" field specifies the attribute syntax, e.g., 0x44
//for the attribute syntax 'keyword'.

//o  The "name-length" field specifies the length of the "name" field
//in bytes, e.g., u in the above diagram or 15 for the name "sides-
//supported".

//o  The "name" field contains the textual name of the attribute, e.g.,
//"sides-supported".

//o  The "value-length" field specifies the length of the "value" field
//in bytes, e.g., v in the above diagram or 9 for the (keyword)
//value 'one-sided'.

//o  The "value" field contains the value of the attribute, e.g., the
//textual value 'one-sided'.

//Each "additional-value" field is encoded as follows:

//-----------------------------------------------
//|                   value-tag                 |   1 byte
//-----------------------------------------------
//|            name-length  (value is 0x0000)   |   2 bytes
//-----------------------------------------------
//|              value-length (value is w)      |   2 bytes
//-----------------------------------------------
//|                     value                   |   w bytes
//-----------------------------------------------

//Figure 5: Additional Attribute Value Encoding

//An "additional-value" is encoded with four subfields:

//o  The "value-tag" field specifies the attribute syntax, e.g., 0x44
//for the attribute syntax 'keyword'.

//o  The "name-length" field has the value of 0 in order to signify
//that it is an "additional-value".  The value of the "name-length"
//field distinguishes an "additional-value" field ("name-length" is
//0) from an "attribute-with-one-value" field ("name-length" is not
//0).

//o  The "value-length" field specifies the length of the "value" field
//in bytes, e.g., w in the above diagram or 19 for the (keyword)
//value 'two-sided-long-edge'.

//o  The "value" field contains the value of the attribute, e.g., the
//textual value 'two-sided-long-edge'.

pub enum ValueTags {
    //out-of-band values
    Unsupported = 0x10,
    Unknown = 0x12,
    NoValue = 0x13,
    //integer values
    UnassignedInteger = 0x20,
    Integer = 0x21,
    Boolean = 0x22,
    Enum = 0x23,
    MoreUnassignedInteger = 0x24,
    MoreUnassignedIntegerLimit = 0x2f,
    //Octet String
    OctetWithUnspecifiedFormat = 0x30,
    Datetime = 0x31,
    Resolution = 0x32,
    RangeOfInteger = 0x33,
    BeginCollection = 0x34,
    TextWithLanguage = 0x35,
    NameWithLanguage = 0x36,
    EndCollection = 0x37,
    UnassignedOctetString = 0x38,
    UnassignedOctetStringLimit = 0x3f,
    //Character String
    UnassignedCharacterString = 0x40,
    TextWithoutLanguage = 0x41,
    NameWithoutLanguage = 0x42,
    UnassignedCharacterString2 = 0x43,
    Keyword = 0x44,
    URI = 0x45,
    URIScheme = 0x46,
    Charset = 0x47,
    NaturalLanguage = 0x48,
    MimeMediaType = 0x49,
    MemberAttrName = 0x4a,
    MoreUnassignedCharacterString = 0x4b,
    MoreUnassignedCharacterStringLimit = 0x4f,
}

impl std::convert::TryFrom<u8> for ValueTags {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            //out-of-band values
            0x10 => Ok(ValueTags::Unsupported),
            0x12 => Ok(ValueTags::Unknown),
            0x13 => Ok(ValueTags::NoValue),
            //integer values
            0x20 => Ok(ValueTags::UnassignedInteger),
            0x21 => Ok(ValueTags::Integer),
            0x22 => Ok(ValueTags::Boolean),
            0x23 => Ok(ValueTags::Enum),
            0x24..=0x2f => Ok(ValueTags::MoreUnassignedInteger),
            //Octet String
            0x30 => Ok(ValueTags::OctetWithUnspecifiedFormat),
            0x31 => Ok(ValueTags::Datetime),
            0x32 => Ok(ValueTags::Resolution),
            0x33 => Ok(ValueTags::RangeOfInteger),
            0x34 => Ok(ValueTags::BeginCollection),
            0x35 => Ok(ValueTags::TextWithLanguage),
            0x36 => Ok(ValueTags::NameWithLanguage),
            0x37 => Ok(ValueTags::EndCollection),
            0x38..=0x3f => Ok(ValueTags::UnassignedOctetString),
            //Character String
            0x40 => Ok(ValueTags::UnassignedCharacterString),
            0x41 => Ok(ValueTags::TextWithoutLanguage),
            0x42 => Ok(ValueTags::NameWithoutLanguage),
            0x43 => Ok(ValueTags::UnassignedCharacterString2),
            0x44 => Ok(ValueTags::Keyword),
            0x45 => Ok(ValueTags::URI),
            0x46 => Ok(ValueTags::URIScheme),
            0x47 => Ok(ValueTags::Charset),
            0x48 => Ok(ValueTags::NaturalLanguage),
            0x49 => Ok(ValueTags::MimeMediaType),
            0x4a => Ok(ValueTags::MemberAttrName),
            0x4b..=0x4f => Ok(ValueTags::MoreUnassignedCharacterString),
            _ => Err("Unsupported value tag."),
        }
    }
}

impl std::fmt::Display for ValueTags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            //out-of-band values
            ValueTags::Unsupported => "unsupported",
            ValueTags::Unknown => "unknown",
            ValueTags::NoValue => "no-value",
            //integer values
            ValueTags::UnassignedInteger => "unassigned-integer",
            ValueTags::Integer => "integer",
            ValueTags::Boolean => "boolean",
            ValueTags::Enum => "enum",
            ValueTags::MoreUnassignedInteger => "unassigned-integer",
            ValueTags::MoreUnassignedIntegerLimit => "unassigned-integer",
            //Octet String
            ValueTags::OctetWithUnspecifiedFormat => "octetString",
            ValueTags::Datetime => "dateTime",
            ValueTags::Resolution => "resolution",
            ValueTags::RangeOfInteger => "rangeOfInteger",
            ValueTags::BeginCollection => "begCollection",
            ValueTags::TextWithLanguage => "textWithLanguage",
            ValueTags::NameWithLanguage => "nameWithLanguage",
            ValueTags::EndCollection => "endCollection",
            ValueTags::UnassignedOctetString => "unassigned-octetString",
            ValueTags::UnassignedOctetStringLimit => "unassigned-octetString",
            //Character String
            ValueTags::UnassignedCharacterString => "unassigned-character-string",
            ValueTags::TextWithoutLanguage => "textWithoutLanguage",
            ValueTags::NameWithoutLanguage => "nameWithoutLanguage",
            ValueTags::UnassignedCharacterString2 => "unassigned-character-string",
            ValueTags::Keyword => "keyword",
            ValueTags::URI => "uri",
            ValueTags::URIScheme => "uriScheme",
            ValueTags::Charset => "charset",
            ValueTags::NaturalLanguage => "naturalLanguage",
            ValueTags::MimeMediaType => "mimeMediaType",
            ValueTags::MemberAttrName => "memberAttrName",
            ValueTags::MoreUnassignedCharacterString => "unassigned-character-string",
            ValueTags::MoreUnassignedCharacterStringLimit => "unassigned-character-string",
        };
        write!(f, "{name}")
    }
}

pub enum PrinterOperations {
    PrintJob,
    PrintURI,
    ValidateJob,
    CreateJob,
    GetPrinterAttributes,
    GetJobs,
    PausePrinter,
    ResumePrinter,
    PurgeJobs,
}

pub enum JobOperations {
    SendDocument,
    SendURI,
    CancelJob,
    GetJobAttributes,
    HoldJob,
    ReleaseJob,
    RestartJob,
}

#[repr(u16)]
pub enum OperationID {
    PrintJob = 0x0002,
    PrintURI = 0x0003,
    ValidateJob = 0x0004,
    CreateJob = 0x0005,
    SendDocument = 0x0006,
    SendURI = 0x0007,
    CancelJob = 0x0008,
    GetJobAttributes = 0x0009,
    GetJobs = 0x000a,
    GetPrinterAttributes = 0x000b,
    HoldJob = 0x000c,
    ReleaseJob = 0x000d,
    RestartJob = 0x000e,
    PausePrinter = 0x0010,
    ResumePrinter = 0x0011,
    PurgeJobs = 0x0012,
}

// Status-code values range from 0x0000 to 0x7fff, divided into classes:
//   successful     0x0000 - 0x00ff
//   informational  0x0100 - 0x01ff (none defined by RFC 8011)
//   redirection    0x0300 - 0x03ff (none defined by RFC 8011)
//   client-error   0x0400 - 0x04ff
//   server-error   0x0500 - 0x05ff
// The top half of each class's range (0x0n80-0x0nff, n = 0..5) is reserved
// for vendor use. Values 0x0600-0x7fff are reserved for future Standards
// Track documents and MUST NOT be used.
#[repr(u16)]
pub enum ResponseStatusCode {
    //The request has succeeded, and no request attributes were substituted
    //or ignored.
    SuccessfulOk = 0x0000,
    //The request has succeeded, but some supplied (1) attributes were
    //ignored or (2) unsupported values were substituted with supported
    //values or were ignored in order to perform the operation without
    //rejecting it.
    SuccessfulOkIgnoredOrSubstituted = 0x0001,
    //The request has succeeded, but some supplied attribute values
    //conflicted with the values of other supplied attributes.  Either
    //(1) these conflicting values were substituted with (supported) values
    //or (2) the attributes were removed in order to process the Job
    //without rejecting it.
    SuccessfulOkConflictedAttributes = 0x0002,
    //The request could not be understood by the IPP object due to
    //malformed syntax (such as the value of a fixed-length attribute whose
    //length does not match the prescribed length for that attribute
    ClientErrorBadRequest = 0x0400,
    //The IPP object understood the request but is refusing to fulfill it.
    //Additional authentication information or authorization credentials
    //will not help, and the request SHOULD NOT be repeated.
    ClientErrorForbidden = 0x0401,
    //The request requires user authentication.  The IPP Client can repeat
    //the request with suitable authentication information.  If the request
    //already included authentication information, then this status-code
    //indicates that authorization has been refused for those credentials.
    ClientErrorNotAuthenticated = 0x0402,
    //The requester is not authorized to perform the request.  Additional
    //authentication information or authorization credentials will not
    //help, and the request SHOULD NOT be repeated.
    ClientErrorNotAuthorized = 0x0403,
    //The request is for something that cannot happen (e.g. canceling a
    //Job that has already been canceled or aborted).
    ClientErrorNotPossible = 0x0404,
    //The Client did not produce a request within the time that the IPP
    //object was prepared to wait.
    ClientErrorTimeout = 0x0405,
    //The IPP object has not found anything matching the request URI.  No
    //indication is given of whether the condition is temporary or
    //permanent.
    ClientErrorNotFound = 0x0406,
    //The requested object is no longer available, and no forwarding
    //address is known.  This condition is considered permanent.
    ClientErrorGone = 0x0407,
    //The IPP object is refusing to process a request because the request
    //entity is larger than the IPP object is willing or able to process.
    ClientErrorRequestEntityTooLarge = 0x0408,
    //The IPP object is refusing to service the request because one or
    //more Client-supplied attributes have a variable-length value that is
    //longer than the maximum length specified for that attribute.
    ClientErrorRequestValueTooLong = 0x0409,
    //The Document data is in a format, as specified in the
    //"document-format" operation attribute, that is not supported by the
    //Printer.
    ClientErrorDocumentFormatNotSupported = 0x040a,
    //In a Job Creation request with "ipp-attribute-fidelity" set to
    //'true', the Printer does not support one or more attributes,
    //attribute syntaxes, or attribute values supplied in the request.
    ClientErrorAttributesOrValuesNotSupported = 0x040b,
    //The scheme of the Client-supplied URI in a Print-URI or a Send-URI
    //operation is not supported.
    ClientErrorUriSchemeNotSupported = 0x040c,
    //The IPP Printer does not support the charset supplied by the Client
    //in the "attributes-charset" operation attribute.
    ClientErrorCharsetNotSupported = 0x040d,
    //The request is rejected because some attribute values conflicted
    //with the values of other attributes that this document does not
    //permit to be substituted or ignored.
    ClientErrorConflictingAttributes = 0x040e,
    //The Document data, as specified in the "compression" operation
    //attribute, is compressed in a way that is not supported by the
    //Printer.
    ClientErrorCompressionNotSupported = 0x040f,
    //The Document data cannot be decompressed using the algorithm
    //specified by the "compression" operation attribute.
    ClientErrorCompressionError = 0x0410,
    //The Printer encountered an error in the Document data while
    //interpreting it.
    ClientErrorDocumentFormatError = 0x0411,
    //The Printer encountered an access error while attempting to validate
    //the accessibility of, or access to, the Document data specified in
    //the "document-uri" operation attribute of a Print-URI or Send-URI
    //request.
    ClientErrorDocumentAccessError = 0x0412,
    //The IPP object encountered an unexpected condition that prevented it
    //from fulfilling the request.  Indicates that intervention by a
    //knowledgeable human is probably required.
    ServerErrorInternalError = 0x0500,
    //The IPP object does not support the functionality required to
    //fulfill the request, e.g. it does not recognize the operation.
    ServerErrorOperationNotSupported = 0x0501,
    //The IPP object is currently unable to handle the request due to
    //temporary overloading or maintenance of the IPP object.
    ServerErrorServiceUnavailable = 0x0502,
    //The IPP object does not support or refuses to support the IPP
    //version supplied as the "version-number" operation parameter.
    ServerErrorVersionNotSupported = 0x0503,
    //A Printer error, such as a paper jam, occurred while the IPP object
    //processed a Print or Send operation.
    ServerErrorDeviceError = 0x0504,
    //A temporary error such as a buffer-full write error, a memory
    //overflow, or a disk-full condition occurred while the IPP Printer
    //processed an operation.
    ServerErrorTemporaryError = 0x0505,
    //The Printer is not currently accepting Jobs because the
    //Administrator has set "printer-is-accepting-jobs" to 'false'.
    ServerErrorNotAcceptingJobs = 0x0506,
    //The Printer is too busy processing Jobs and/or other requests.
    ServerErrorBusy = 0x0507,
    //The Job has been canceled by an Operator or the system while the
    //Client was transmitting the data to the IPP Printer.
    ServerErrorJobCanceled = 0x0508,
    //The IPP object does not support multiple Documents per Job, and a
    //Client attempted to supply Document data with a second
    //Send-Document or Send-URI operation.
    ServerErrorMultipleDocumentJobsNotSupported = 0x0509,
}

impl std::convert::TryFrom<u16> for ResponseStatusCode {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0000 => Ok(ResponseStatusCode::SuccessfulOk),
            0x0001 => Ok(ResponseStatusCode::SuccessfulOkIgnoredOrSubstituted),
            0x0002 => Ok(ResponseStatusCode::SuccessfulOkConflictedAttributes),
            0x0400 => Ok(ResponseStatusCode::ClientErrorBadRequest),
            0x0401 => Ok(ResponseStatusCode::ClientErrorForbidden),
            0x0402 => Ok(ResponseStatusCode::ClientErrorNotAuthenticated),
            0x0403 => Ok(ResponseStatusCode::ClientErrorNotAuthorized),
            0x0404 => Ok(ResponseStatusCode::ClientErrorNotPossible),
            0x0405 => Ok(ResponseStatusCode::ClientErrorTimeout),
            0x0406 => Ok(ResponseStatusCode::ClientErrorNotFound),
            0x0407 => Ok(ResponseStatusCode::ClientErrorGone),
            0x0408 => Ok(ResponseStatusCode::ClientErrorRequestEntityTooLarge),
            0x0409 => Ok(ResponseStatusCode::ClientErrorRequestValueTooLong),
            0x040a => Ok(ResponseStatusCode::ClientErrorDocumentFormatNotSupported),
            0x040b => Ok(ResponseStatusCode::ClientErrorAttributesOrValuesNotSupported),
            0x040c => Ok(ResponseStatusCode::ClientErrorUriSchemeNotSupported),
            0x040d => Ok(ResponseStatusCode::ClientErrorCharsetNotSupported),
            0x040e => Ok(ResponseStatusCode::ClientErrorConflictingAttributes),
            0x040f => Ok(ResponseStatusCode::ClientErrorCompressionNotSupported),
            0x0410 => Ok(ResponseStatusCode::ClientErrorCompressionError),
            0x0411 => Ok(ResponseStatusCode::ClientErrorDocumentFormatError),
            0x0412 => Ok(ResponseStatusCode::ClientErrorDocumentAccessError),
            0x0500 => Ok(ResponseStatusCode::ServerErrorInternalError),
            0x0501 => Ok(ResponseStatusCode::ServerErrorOperationNotSupported),
            0x0502 => Ok(ResponseStatusCode::ServerErrorServiceUnavailable),
            0x0503 => Ok(ResponseStatusCode::ServerErrorVersionNotSupported),
            0x0504 => Ok(ResponseStatusCode::ServerErrorDeviceError),
            0x0505 => Ok(ResponseStatusCode::ServerErrorTemporaryError),
            0x0506 => Ok(ResponseStatusCode::ServerErrorNotAcceptingJobs),
            0x0507 => Ok(ResponseStatusCode::ServerErrorBusy),
            0x0508 => Ok(ResponseStatusCode::ServerErrorJobCanceled),
            0x0509 => Ok(ResponseStatusCode::ServerErrorMultipleDocumentJobsNotSupported),
            other => Err(other),
        }
    }
}

impl std::fmt::Display for ResponseStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            ResponseStatusCode::SuccessfulOk => "successful-ok",
            ResponseStatusCode::SuccessfulOkIgnoredOrSubstituted => {
                "successful-ok-ignored-or-substituted-attributes"
            }
            ResponseStatusCode::SuccessfulOkConflictedAttributes => {
                "successful-ok-conflicting-attributes"
            }
            ResponseStatusCode::ClientErrorBadRequest => "client-error-bad-request",
            ResponseStatusCode::ClientErrorForbidden => "client-error-forbidden",
            ResponseStatusCode::ClientErrorNotAuthenticated => "client-error-not-authenticated",
            ResponseStatusCode::ClientErrorNotAuthorized => "client-error-not-authorized",
            ResponseStatusCode::ClientErrorNotPossible => "client-error-not-possible",
            ResponseStatusCode::ClientErrorTimeout => "client-error-timeout",
            ResponseStatusCode::ClientErrorNotFound => "client-error-not-found",
            ResponseStatusCode::ClientErrorGone => "client-error-gone",
            ResponseStatusCode::ClientErrorRequestEntityTooLarge => {
                "client-error-request-entity-too-large"
            }
            ResponseStatusCode::ClientErrorRequestValueTooLong => {
                "client-error-request-value-too-long"
            }
            ResponseStatusCode::ClientErrorDocumentFormatNotSupported => {
                "client-error-document-format-not-supported"
            }
            ResponseStatusCode::ClientErrorAttributesOrValuesNotSupported => {
                "client-error-attributes-or-values-not-supported"
            }
            ResponseStatusCode::ClientErrorUriSchemeNotSupported => {
                "client-error-uri-scheme-not-supported"
            }
            ResponseStatusCode::ClientErrorCharsetNotSupported => {
                "client-error-charset-not-supported"
            }
            ResponseStatusCode::ClientErrorConflictingAttributes => {
                "client-error-conflicting-attributes"
            }
            ResponseStatusCode::ClientErrorCompressionNotSupported => {
                "client-error-compression-not-supported"
            }
            ResponseStatusCode::ClientErrorCompressionError => "client-error-compression-error",
            ResponseStatusCode::ClientErrorDocumentFormatError => {
                "client-error-document-format-error"
            }
            ResponseStatusCode::ClientErrorDocumentAccessError => {
                "client-error-document-access-error"
            }
            ResponseStatusCode::ServerErrorInternalError => "server-error-internal-error",
            ResponseStatusCode::ServerErrorOperationNotSupported => {
                "server-error-operation-not-supported"
            }
            ResponseStatusCode::ServerErrorServiceUnavailable => "server-error-service-unavailable",
            ResponseStatusCode::ServerErrorVersionNotSupported => {
                "server-error-version-not-supported"
            }
            ResponseStatusCode::ServerErrorDeviceError => "server-error-device-error",
            ResponseStatusCode::ServerErrorTemporaryError => "server-error-temporary-error",
            ResponseStatusCode::ServerErrorNotAcceptingJobs => "server-error-not-accepting-jobs",
            ResponseStatusCode::ServerErrorBusy => "server-error-busy",
            ResponseStatusCode::ServerErrorJobCanceled => "server-error-job-canceled",
            ResponseStatusCode::ServerErrorMultipleDocumentJobsNotSupported => {
                "server-error-multiple-document-jobs-not-supported"
            }
        };
        write!(f, "{}", name)
    }
}

#[repr(u32)]
pub enum RequestID {
    PrintJob = 0x00000001,
    GetPrinterAttributes = 0x00000002,
}

pub fn pack_u16_to_u32(high: u16, low: u16) -> u32 {
    ((high as u32) << 16) | (low as u32)
}

pub fn pack_u8_to_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}
