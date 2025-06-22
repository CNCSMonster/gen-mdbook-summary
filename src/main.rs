mod cli;

use clap::Parser;
use cli::*;

fn main() -> anyhow::Result<()> {
    // set env_logger
    env_logger::init();
    let cli = Cli2::parse();
    match cli.cmd {
        Some(Cmd::Init) => handle_init(),
        Some(Cmd::Gen(gen_args)) => handle_gen(&gen_args),
        None => handle_gen(&cli.gen_args),
    }
}
