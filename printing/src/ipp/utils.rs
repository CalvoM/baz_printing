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

pub const SUPPORTED_VERSION: u16 = 0x0101; // 1.1

#[repr(C)]
#[derive(serde::Serialize)]
pub struct IPPOperationRequest {
    pub version: u16,
    pub operation_id: u16,
    pub request_id: u32,
}

#[repr(C)]
#[derive(serde::Serialize)]
pub struct IPPOperationResponse {
    pub version: u16,
    pub status_code: u16,
    pub request_id: u32,
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
