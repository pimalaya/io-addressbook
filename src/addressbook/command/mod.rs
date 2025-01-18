mod create;
mod list;
mod update;

use clap::Subcommand;
use color_eyre::Result;
use pimalaya_tui::terminal::cli::printer::Printer;

use crate::config::TomlConfig;

use self::{
    create::CreateAddressbookCommand, list::ListAddressbooksCommand,
    update::UpdateAddressbookCommand,
};

#[derive(Debug, Subcommand)]
pub enum AddressbookSubcommand {
    #[command(alias = "lst")]
    List(ListAddressbooksCommand),
    #[command(alias = "new", alias = "add")]
    Create(CreateAddressbookCommand),
    #[command(alias = "set", alias = "change")]
    Update(UpdateAddressbookCommand),
}

impl AddressbookSubcommand {
    pub fn execute(self, printer: &mut impl Printer, config: TomlConfig) -> Result<()> {
        match self {
            Self::List(cmd) => cmd.execute(printer, config),
            Self::Create(cmd) => cmd.execute(printer, config),
            Self::Update(cmd) => cmd.execute(printer, config),
        }
    }
}
