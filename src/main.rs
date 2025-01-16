use cardamum::cli::Cli;
use clap::Parser;
use color_eyre::Result;
use pimalaya_tui::terminal::cli::{printer::StdoutPrinter, tracing};

fn main() -> Result<()> {
    let tracing = tracing::install()?;

    let cli = Cli::parse();
    let mut printer = StdoutPrinter::new(cli.output);
    let res = cli.command.execute(&mut printer, cli.config_paths.as_ref());

    tracing.with_debug_and_trace_notes(res)
}
