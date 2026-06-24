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
