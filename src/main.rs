use cardamum::{cli::Cli, config::TomlConfig};
use clap::Parser;
use color_eyre::Result;
use pimalaya_tui::terminal::{
    cli::{
        printer::{Printer, StdoutPrinter},
        tracing,
    },
    config::TomlConfig as _,
};

fn main() -> Result<()> {
    let tracing = tracing::install()?;

    let cli = Cli::parse();
    let mut printer = StdoutPrinter::new(cli.output);
    let res = match cli.command {
        Some(cmd) => cmd.execute(&mut printer, cli.config_paths.as_ref()),
        None => {
            let config = TomlConfig::from_paths_or_default(cli.config_paths.as_ref())?;
            printer.out(config)
        }
    };

    tracing.with_debug_and_trace_notes(res)
}
