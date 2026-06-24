use clap::{Args, Parser, Subcommand};
use printing::lpd::client::LPDPClient;
use printing::lpd::errors::LPDPClientError;

#[derive(Parser)]
#[command(name = "lpd_client")]
#[command(version = "1.0")]
#[command(about="LPD (Line Printer Daemon) client, communicates with LPD server", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: LPDCommands,
    #[arg(long, short = 'H')]
    host: String,
    #[arg(long, short = 'P')]
    queue_name: String,
}

#[derive(Subcommand)]
enum LPDCommands {
    PrintAll,
    Print(PrintArgs),
    Query(QueryArgs),
    AbortAll(AbortAllArgs),
    Abort,
}

#[derive(Args)]
struct PrintArgs {
    file_path: String,
}

#[derive(Args)]
struct QueryArgs {
    username: Option<String>,
    job_number: Option<String>,
}

#[derive(Args)]
struct AbortAllArgs {
    username: Option<String>,
    job_number: Option<String>,
}

#[dotenvy::load]
fn main() -> Result<(), LPDPClientError> {
    let cli = Cli::parse();
    let lpd_host = cli.host;
    let queue_name = cli.queue_name;
    match &cli.command {
        LPDCommands::PrintAll => {
            let mut lpd_client = LPDPClient::try_new(&queue_name, &lpd_host)?;
            lpd_client.print_remaining_jobs()?;
        }
        LPDCommands::Print(args) => {
            let mut lpd_client = LPDPClient::try_new(&queue_name, &lpd_host)?;
            lpd_client.send_printer_job(std::path::Path::new(&args.file_path))?;
        }
        LPDCommands::Query(args) => {
            let mut lpd_client = LPDPClient::try_new(&queue_name, &lpd_host)?;
            let query = lpd_client
                .request_queue_start_long(args.username.clone(), args.job_number.clone())?;
            println!("{query}");
        }
        LPDCommands::AbortAll(args) => {
            let mut lpd_client = LPDPClient::try_new(&queue_name, &lpd_host)?;
            lpd_client.request_job_removal(args.username.clone(), args.job_number.clone())?;
        }
        LPDCommands::Abort => {
            let mut lpd_client = LPDPClient::try_new(&queue_name, &lpd_host)?;
            lpd_client.abort_printer_job()?;
        }
    }
    Ok(())
}
