pub enum DaemonCommand {
    //+----+-------+----+
    //| 01 | Queue | LF |
    //+----+-------+----+
    //Operand - Printer queue name
    PrintRemainingJobs = 0x01,
    //+----+-------+----+
    //| 02 | Queue | LF |
    //+----+-------+----+
    //Operand - Printer queue name
    ReceivePrinterJob = 0x02,
    //+----+-------+----+------+----+
    //| 03 | Queue | SP | List | LF |
    //+----+-------+----+------+----+
    //Operand 1 - Printer queue name
    //Other operands - User names or job numbers
    SendQueueStateJobShort = 0x03,
    //+----+-------+----+------+----+
    //| 04 | Queue | SP | List | LF |
    //+----+-------+----+------+----+
    //Operand 1 - Printer queue name
    //Other operands - User names or job numbers
    SendQueueStateJobLong = 0x04,
    //+----+-------+----+-------+----+------+----+
    //| 05 | Queue | SP | Agent | SP | List | LF |
    //+----+-------+----+-------+----+------+----+
    //Operand 1 - Printer queue name
    //Operand 2 - User name making request (the agent)
    //Other operands - User names or job numbers
    RemoveJobs = 0x05,
}

pub enum ReceiveJobSubCommand {
    //+----+----+
    //| 01 | LF |
    //+----+----+
    Abort = 0x01,
    //+----+-------+----+------+----+
    //| 02 | Count | SP | Name | LF |
    //+----+-------+----+------+----+
    //Command code - 2
    //Operand 1 - Number of bytes in control file
    //Operand 2 - Name of control file
    ReceiveControlFile = 0x02,
    //+----+-------+----+------+----+
    //| 03 | Count | SP | Name | LF |
    //+----+-------+----+------+----+
    //Command code - 3
    //Operand 1 - Number of bytes in data file
    //Operand 2 - Name of data file
    ReceiveDataFile = 0x03,
}
