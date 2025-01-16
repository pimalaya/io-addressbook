use clap::{value_parser, CommandFactory, Parser};
use clap_complete::Shell;
use color_eyre::Result;
use pimalaya_tui::terminal::cli::printer::Printer;
use tracing::info;

use crate::cli::Cli;

/// Print completion script for the given shell to stdout.
///
/// This command allows you to generate completion script for a given
/// shell. The script is printed to the standard output. If you want
/// to write it to a file, just use unix redirection.
#[derive(Debug, Parser)]
pub struct GenerateCompletionsCommand {
    /// Shell for which completion script should be generated for.
    #[arg(value_parser = value_parser!(Shell))]
    pub shell: Shell,
}

impl GenerateCompletionsCommand {
    pub fn execute(self, printer: &mut impl Printer) -> Result<()> {
        info!(shell = ?self.shell, "generate completion script");

        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();

        let mut buf = Vec::new();
        clap_complete::generate(self.shell, &mut cmd, name, &mut buf);
        let script = String::from_utf8(buf)?;

        printer.out(script)
    }
}
