use std::process::exit;

use argh::FromArgs;
use gen_mdbook_summary::Ignore;
use gen_mdbook_summary::SummaryItem;
use log::error;
use log::info;

pub const IGNORE_FILE: &str = ".gmsignore";

#[derive(FromArgs)]
/// A tool to generate SUMMARY.md for mdbook project
pub struct Args {
    #[argh(subcommand)]
    pub cmd: Option<Cmd>,

    /// optional name to operate on
    #[argh(option, short = 'd', default = "String::from(\"src\")")]
    pub dir: String,
    /// specify the output file
    #[argh(option, short = 'o')]
    pub output: Option<String>,
    /// if organize the items in order
    #[argh(option, short = 's', default = "true")]
    pub sort: bool,
    /// specify the ignore file
    #[argh(option, short = 'i', default = "String::from(\".gmsignore\")")]
    pub ignore: String,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Cmd {
    Init(InitArgs),
    Gen(GenArgs),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "init")]
/// initialize the .gmsignore file
pub struct InitArgs {}

#[derive(FromArgs, Debug, Clone)]
#[argh(subcommand, name = "gen")]
/// generate the summary file
pub struct GenArgs {
    /// optional name to operate on
    #[argh(option, short = 'd', default = "String::from(\"src\")")]
    pub dir: String,
    /// specify the output file
    #[argh(option, short = 'o')]
    pub output: Option<String>,
    /// if organize the items in order
    #[argh(option, short = 's', default = "true")]
    pub sort: bool,
    /// specify the ignore file
    #[argh(option, short = 'i', default = "String::from(\".gmsignore\")")]
    pub ignore: String,
}

pub fn handle_gen(gen_args: &GenArgs) -> anyhow::Result<()> {
    let ignore = Ignore::new(&gen_args.dir, &gen_args.ignore).unwrap_or_else(|e| {
        error!("{}", e);
        exit(-1);
    });
    info!("{:?}", &ignore);

    // 使用绝对路径作为 base_dir
    let base_dir = std::path::Path::new(&gen_args.dir).canonicalize()?;

    // 解析输出文件路径
    let output_path = gen_args.output.as_ref().map(|o| {
        let path = std::path::Path::new(o);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap().join(path)
        }
    });

    let mut summary = SummaryItem::new(&gen_args.dir, &ignore, &base_dir, output_path.as_deref())
        .unwrap_or_else(|e| {
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
        "book.toml
.gitignore
book
*.doc
*.pdf
*.png
*.xlsx
*.pptx
*.jpg
",
    )?;
    Ok(())
}
