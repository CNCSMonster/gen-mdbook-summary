use std::process::exit;

use clap::Args;
use clap::Parser;
use gen_mdbook_summary::Ignore;
use gen_mdbook_summary::SummaryItem;
use log::error;
use log::info;

use indoc::indoc;

pub const IGNORE_FILE: &str = ".gmsignore";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli2 {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
    #[command(flatten)]
    pub gen_args: GenArgs,
}

#[derive(Clone, clap::Subcommand)]
pub enum Cmd {
    /// initialize the .gmsignore file
    /// This file is the default ignore file for the summary generator,
    /// it will be used to ignore files that should not be included in the summary.
    /// You can modify it to suit your needs.
    /// If you want to use a different ignore file, you can specify it with the --ignore option.
    #[command(about = "initialize the .gmsignore file")]
    Init,
    /// generate the summary file
    Gen(GenArgs),
}

#[derive(Args, Clone, Debug)]
pub struct GenArgs {
    /// Optional name to operate on
    #[arg(short, long, default_value = "src")]
    pub dir: String,
    /// specify the output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// if organize the items in order
    #[arg(short, long, default_value = "true")]
    pub sort: bool,
    #[arg(
        short,
        long,
        default_value = IGNORE_FILE,
        help = "specify the ignore file ,using .gitignore grammar,
    matched files will be ignored."
    )]
    pub ignore: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(short, long, default_value = "src")]
    pub dir: String,
    /// specify the output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// if organize the items in order
    #[arg(short, long, default_value = "true")]
    pub sort: bool,
    #[arg(
        short,
        long,
        default_value = IGNORE_FILE,
        help = "specify the ignore file ,using .gitignore grammar,
    matched files will be ignored."
    )]
    pub ignore: String,
}

pub fn handle_gen(gen_args: &GenArgs) -> anyhow::Result<()> {
    let ignore = Ignore::new(&gen_args.dir, &gen_args.ignore).unwrap_or_else(|e| {
        error!("{}", e);
        exit(-1);
    });
    info!("{:?}", &ignore);
    let mut summary = SummaryItem::new(&gen_args.dir, &ignore).unwrap_or_else(|e| {
        error!("{}", e);
        exit(-1);
    });
    info!("{:?}", &summary);
    if gen_args.sort {
        info!("sort the summary");
        summary.sort();
    }
    match summary.gen_summary() {
        Ok(summary) => {
            if let Some(output) = &gen_args.output {
                info!("output SUMMARY.md to {}", output);
                if let Err(e) = std::fs::write(output, summary) {
                    error!("{}", e);
                    exit(-1);
                }
            } else {
                println!("{}", summary);
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
    Ok(())
}

pub fn handle_init() -> anyhow::Result<()> {
    std::fs::write(
        IGNORE_FILE,
        indoc! {r##"
            book.toml
            .gitignore
            book
            *.doc
            *.pdf
            *.png
            *.xlsx
            *.pptx
            *.jpg
        "##},
    )?;
    Ok(())
}
