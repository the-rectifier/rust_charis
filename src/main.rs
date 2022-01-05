use anyhow::Result;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use structopt::StructOpt;

mod asm;
use crate::asm::*;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "CHARISA Tools",
    author = "Stavrou Odysseas (canopus)",
    version = "0.1.0"
)]
struct Opts {
    /// Command to run
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Lists Available Serial Ports
    #[structopt(name = "asm")]
    Asm(Asm),

    #[structopt(name = "disasm")]
    Disasm(Disasm),
}

fn main() -> Result<()> {
    TermLogger::init(
        log::LevelFilter::Info,
        ConfigBuilder::new().set_time_to_local(true).build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Failed to init logger");

    let opts = Opts::from_args();

    match opts.cmd {
        Command::Asm(args) => do_asm(args)?,
        Command::Disasm(args) => do_disasm(args)?,
    }

    Ok(())
}
