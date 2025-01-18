mod create;
mod list;
mod read;

use clap::Subcommand;
use color_eyre::Result;
use pimalaya_tui::terminal::cli::printer::Printer;

use crate::config::TomlConfig;

use self::{create::CreateCardCommand, list::ListCardsCommand, read::ReadCardCommand};

#[derive(Debug, Subcommand)]
pub enum CardSubcommand {
    #[command(alias = "new", alias = "add")]
    Create(CreateCardCommand),

    #[command(alias = "get")]
    Read(ReadCardCommand),

    #[command(alias = "lst")]
    List(ListCardsCommand),
}

impl CardSubcommand {
    pub fn execute(self, printer: &mut impl Printer, config: TomlConfig) -> Result<()> {
        match self {
            Self::Create(cmd) => cmd.execute(printer, config),
            Self::Read(cmd) => cmd.execute(printer, config),
            Self::List(cmd) => cmd.execute(printer, config),
        }
    }
}
