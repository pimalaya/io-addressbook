use std::process::{self, Command};

use clap::Parser;
use color_eyre::{eyre::bail, Result};
use pimalaya_tui::terminal::{cli::printer::Printer, config::TomlConfig as _, prompt};

use crate::{
    account::{arg::name::AccountNameFlag, config::Backend},
    carddav::sans_io::DeleteAddressbookFlow,
    config::TomlConfig,
    contact::{Authentication, Encryption},
    tcp::{sans_io::Io as TcpIo, std::Connector},
    tls::std::RustlsConnector,
};

/// Delete all folders.
///
/// This command allows you to delete all exsting folders.
#[derive(Debug, Parser)]
pub struct DeleteAddressbookCommand {
    #[command(flatten)]
    pub account: AccountNameFlag,

    #[arg(name = "ADDRESSBOOK-ID")]
    pub id: String,

    #[arg(long, short)]
    pub yes: bool,
}

impl DeleteAddressbookCommand {
    pub fn execute(self, printer: &mut impl Printer, config: TomlConfig) -> Result<()> {
        if !self.yes {
            let confirm = "Do you really want to delete this addressbook";
            let confirm = format!("{confirm}? All contacts will be definitely deleted.");

            if !prompt::bool(confirm, false)? {
                process::exit(0);
            };
        };

        let (_, toml_account_config) =
            config.to_toml_account_config(self.account.name.as_deref())?;
        let (_, backend) = toml_account_config.into();

        match backend {
            Backend::None => bail!("cannot delete addressbook: backend is not defined"),
            Backend::CardDav(config) => match config.authentication {
                Authentication::None => unimplemented!(),
                Authentication::Basic(auth) => {
                    let mut args = auth.password.split_whitespace();
                    let program = args.next().unwrap();
                    let password = Command::new(program).args(args).output().unwrap().stdout;
                    let password = String::from_utf8_lossy(password.trim_ascii());
                    let mut flow = DeleteAddressbookFlow::new(
                        &self.id,
                        &config.http_version,
                        &auth.username,
                        &password,
                    );

                    match config.encryption {
                        Encryption::None => {
                            let mut tcp = Connector::connect(&config.hostname, config.port)?;

                            while let Some(io) = flow.next() {
                                match io {
                                    TcpIo::Read => {
                                        tcp.read(&mut flow)?;
                                    }
                                    TcpIo::Write => {
                                        tcp.write(&mut flow)?;
                                    }
                                }
                            }
                        }
                        Encryption::Rustls(_) => {
                            let mut tls = RustlsConnector::connect(&config.hostname, config.port)?;

                            while let Some(io) = flow.next() {
                                match io {
                                    TcpIo::Read => {
                                        tls.read(&mut flow)?;
                                    }
                                    TcpIo::Write => {
                                        tls.write(&mut flow)?;
                                    }
                                }
                            }
                        }
                    }
                }
            },
        };

        printer.out("Addressbook successfully deleted")
    }
}
