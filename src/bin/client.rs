use baz_printing_lpdp::utils::{DaemonCommand, RecieveJobSubCommand};
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn read_ack(stream: &mut TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1];
    let n = stream.read(&mut buf)?;
    if n != 1 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "expected ACK byte",
        ));
    }
    if buf[0] != 0x00 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("expected ACK 0x00, got 0x{:02X}", buf[0]),
        ));
    }
    Ok(())
}

#[dotenvy::load]
fn main() -> io::Result<()> {
    let control_content = b"Hcookingpot\nPd1r3ct0r\nNdocument.txt\nldfA001cookingpot\n";
    let data_content = b"Hello from Python LPD client!";
    let lpdp_server =
        std::env::var("LDPD_DAEMON").unwrap_or_else(|_| "192.168.100.200".to_string());
    let queue_name = std::env::var("PRINTER_QUEUE_NAME")
        .unwrap_or_else(|_| "HP_Color_LaserJet_MFP_M283fdw".to_string());
    let mut stream = TcpStream::connect(format!("{lpdp_server}:515"))?;

    // Receive job
    let recv_job_command: Vec<u8> = [
        &[DaemonCommand::ReceivePrinterJob as u8][..],
        queue_name.as_bytes(),
        b"\n",
    ]
    .concat();
    stream.write_all(&recv_job_command)?;
    read_ack(&mut stream)?;

    // Control file: header → ACK → content → NUL → ACK
    let cf_name = b"cfA001cookingpot";
    let cf_count = control_content.len().to_string();
    let control_file_cmd = [
        &[RecieveJobSubCommand::ReceiveControlFile as u8][..],
        cf_count.as_bytes(),
        b" ",
        cf_name,
        b"\n",
    ]
    .concat();
    stream.write_all(&control_file_cmd)?;
    read_ack(&mut stream)?;
    stream.write_all(control_content)?;
    stream.write_all(&[0x00u8])?;
    read_ack(&mut stream)?;

    // Data file: header → ACK → content → NUL → ACK
    let df_name = b"dfA001cookingpot";
    let df_count = data_content.len().to_string();
    let data_file_cmd = [
        &[RecieveJobSubCommand::ReceiveDataFile as u8][..],
        df_count.as_bytes(),
        b" ",
        df_name,
        b"\n",
    ]
    .concat();
    stream.write_all(&data_file_cmd)?;
    read_ack(&mut stream)?;
    stream.write_all(data_content)?;
    stream.write_all(&[0x00u8])?;
    read_ack(&mut stream)?;

    Ok(())
}
