mod list;

use clap::Subcommand;
use color_eyre::Result;
use pimalaya_tui::terminal::cli::printer::Printer;

use crate::config::TomlConfig;

use self::list::ListCardsCommand;

#[derive(Debug, Subcommand)]
pub enum CardSubcommand {
    #[command(alias = "lst")]
    List(ListCardsCommand),
}

impl CardSubcommand {
    pub fn execute(self, printer: &mut impl Printer, config: TomlConfig) -> Result<()> {
        match self {
            Self::List(cmd) => cmd.execute(printer, config),
        }
    }
}
