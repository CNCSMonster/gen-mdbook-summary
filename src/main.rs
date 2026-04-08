mod cli;

use cli::*;

fn main() -> anyhow::Result<()> {
    // set env_logger
    env_logger::init();
    let args: Args = argh::from_env();
    match args.cmd {
        Some(Cmd::Init(_)) => handle_init(),
        Some(Cmd::Gen(gen_args)) => handle_gen(&gen_args),
        None => handle_gen(&GenArgs {
            dir: args.dir,
            output: args.output,
            sort: args.sort,
            ignore: args.ignore,
        }),
    }
}
