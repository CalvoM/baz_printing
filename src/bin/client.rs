use baz_printing_lpdp::client::LPDPClient;
use baz_printing_lpdp::errors::LPDPClientError;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //path to the file to printed
    #[arg(short, long)]
    file_path: String,
}

#[dotenvy::load]
fn main() -> Result<(), LPDPClientError> {
    let args = Args::parse();
    let lpdp_server =
        std::env::var("LPDP_DAEMON").unwrap_or_else(|_| "192.168.100.200".to_string());
    let queue_name = std::env::var("PRINTER_QUEUE_NAME")
        .unwrap_or_else(|_| "HP_Color_LaserJet_MFP_M283fdw".to_string());

    let mut lpdp_client = LPDPClient::try_new(&queue_name, &lpdp_server)?;
    lpdp_client.send_printer_job(std::path::Path::new(&args.file_path))?;
    Ok(())
}
