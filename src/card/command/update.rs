use std::{
    env::{self, temp_dir},
    fs,
    process::{Command, Stdio},
};

use clap::Parser;
use color_eyre::{eyre::bail, Result};
use pimalaya_tui::terminal::{cli::printer::Printer, config::TomlConfig as _};

use crate::{
    account::{arg::name::AccountNameFlag, config::Backend},
    carddav::sans_io::{ReadCardFlow, UpdateCardFlow},
    config::TomlConfig,
    contact::{Authentication, Encryption},
    tcp::{sans_io::Io as TcpIo, std::Connector},
    tls::std::RustlsConnector,
};

/// Update all folders.
///
/// This command allows you to update all exsting folders.
#[derive(Debug, Parser)]
pub struct UpdateCardCommand {
    #[command(flatten)]
    pub account: AccountNameFlag,

    /// The identifier of the vCard to update.
    #[arg(name = "CARD-ID")]
    pub id: String,
}

impl UpdateCardCommand {
    pub fn execute(self, printer: &mut impl Printer, config: TomlConfig) -> Result<()> {
        let (_, toml_account_config) =
            config.to_toml_account_config(self.account.name.as_deref())?;
        let (_, backend) = toml_account_config.into();

        match backend {
            Backend::None => bail!("cannot update card: backend is not defined"),
            Backend::CardDav(config) => {
                match config.authentication {
                    Authentication::None => unimplemented!(),
                    Authentication::Basic(auth) => {
                        let mut args = auth.password.split_whitespace();
                        let program = args.next().unwrap();
                        let password = Command::new(program).args(args).output()?.stdout;
                        let password = String::from_utf8_lossy(password.trim_ascii());

                        let mut flow = ReadCardFlow::new(
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
                                let mut tls =
                                    RustlsConnector::connect(&config.hostname, config.port)?;

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

                        let (_, uid) = self.id.rsplit_once('/').unwrap();
                        let path = temp_dir().join(uid);
                        fs::write(&path, flow.output()?)?;

                        let args = env::var("EDITOR")?;
                        let mut args = args.split_whitespace();
                        let editor = args.next().unwrap();
                        let edition = Command::new(editor)
                            .args(args)
                            .arg(&path)
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .status()?;

                        if !edition.success() {
                            let code = edition.code();
                            bail!("error while editing vCard: error code {code:?}");
                        }

                        let mut flow = UpdateCardFlow::new(
                            &self.id,
                            &config.http_version,
                            &auth.username,
                            &password,
                            &fs::read_to_string(&path)?,
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
                                let mut tls =
                                    RustlsConnector::connect(&config.hostname, config.port)?;

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
                };
            }
        };

        printer.out("Card successfully updated")
    }
}
