use clap::{Args, Parser, Subcommand};
use printing::ipp::{client::IPPClient, errors::IPPClientError};

#[derive(Parser)]
#[command(name = "ipp_client")]
#[command(version = "1.0")]
#[command(about="IPP (Internet Printing Protocol) client, communicates with IPP server", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, short = 'H')]
    host: String,
}

fn main() -> Result<(), IPPClientError> {
    let cli = Cli::parse();
    let ipp_host = cli.host;
    let mut ipp_client = IPPClient::try_new(&ipp_host)?;
    //ipp_client.send_print_job()?;
    ipp_client.get_printer_attributes()?;
    Ok(())
}
