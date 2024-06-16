use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long, default_value = "src")]
    dir: String,
    /// specify the output file
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Debug)]
struct SummaryItem {
    name: String,
    path: PathBuf,
    introduction: Option<String>,
    chapters: Vec<SummaryItem>,
}
impl SummaryItem {
    pub fn new(dir: &str, ignore: &Ignore) -> anyhow::Result<Self> {
        info!("try to create SummaryItem from {}", dir);
        let mut chapters = Vec::new();
        let dir = Path::new(dir).canonicalize()?;
        // check if the dir is a file
        let dir = dir
            .to_str()
            .ok_or(anyhow!("[{}{}]cannot get str of  dir", file!(), line!()))?;
        let meta = fs::metadata(dir)?;
        if meta.is_file() {
            let name = dir
                .split("/")
                .last()
                .ok_or(anyhow!("[{}{}]invalid name", file!(), line!()))?
                .split(".")
                .next()
                .unwrap()
                .to_string();
            return Ok(Self {
                name,
                path: PathBuf::from(dir),
                introduction: None,
                chapters,
            });
        }

        // check if the dir is a directory
        if !meta.is_dir() {
            return Err(anyhow!("{} is neither a file nor a directory", dir));
        }
        info!("{dir}");
        // check introduction
        let mut introduction: Option<String> = None;
        let names = vec!["README.md", "readme.md", "README", "readme"];
        // check if the introduction file exists
        for name in names {
            let path = format!("{}/{}", dir, name);
            if Path::new(&path).exists() && !ignore.is_ignore(&path) {
                introduction = Some(path);
                break;
            }
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_str =
                path.to_str()
                    .ok_or(anyhow!("[{}{}]cannot get str of  path", file!(), line!()))?;
            info!("{}", path_str);
            if ignore.is_ignore(path_str) {
                info!("ignore {}", path_str);
                continue;
            }
            if path_str.ends_with("readme.md") || path_str.ends_with("README.md") {
                info!("path_str {} is readme.md,skip", path_str);
                continue;
            }
            if path.is_dir() {
                chapters.push(Self::new(path_str, ignore)?);
            } else {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                chapters.push(Self {
                    name,
                    path,
                    introduction: None,
                    chapters: Vec::new(),
                });
            }
        }

        let name = dir.split("/").last().unwrap().to_string();
        let path = PathBuf::from(dir);
        Ok(Self {
            name,
            path,
            introduction,
            chapters,
        })
    }
    pub fn gen_summary(&self) -> Result<String> {
        let mut summary = String::new();
        summary.push_str("# Summary\n\n");
        if self.path.is_dir() {
            for chapter in &self.chapters {
                summary.push('\n');
                summary.push_str(&chapter.item(0)?);
            }
        } else {
            let path_str = self.path.to_str().ok_or(anyhow!(line!()))?;
            summary.push_str(&format!("- [{}]({})", self.name, path_str));
        }
        Ok(summary)
    }
    pub fn item(&self, depth: usize) -> Result<String> {
        let mut item = String::new();
        for _ in 0..depth {
            item.push('\t');
        }
        let path_str = self.path.to_str().ok_or(anyhow!(line!()))?;
        if self.path.is_dir() {
            if let Some(introduction) = &self.introduction {
                item.push_str(format!("- [{}]({})", &self.name, introduction).as_str());
            } else {
                item.push_str(format!("- [{}]()", &self.name).as_str());
            }
            for chapter in self.chapters.iter() {
                item.push('\n');
                item.push_str(&chapter.item(depth + 1)?);
            }
        } else {
            item.push_str(&format!("- [{}]({})", &self.name, path_str));
        }
        info!("{item}");
        Ok(item)
    }
}

#[derive(Debug)]
pub struct Ignore {
    unignored: HashSet<String>,
}
impl Ignore {
    pub fn new(dir: &str) -> Result<Self> {
        use ignore::WalkBuilder;
        let mut unignored = HashSet::new();
        for result in WalkBuilder::new(dir)
            .add_custom_ignore_filename("mdbook.ignore")
            .build()
        {
            let result = result?;
            let path = result.path().canonicalize()?;
            let path = path.to_str().ok_or(anyhow!("[{}{}]", file!(), line!()))?;
            unignored.insert(path.to_string());
        }
        Ok(Self { unignored })
    }

    pub fn is_ignore(&self, path: &str) -> bool {
        !self.unignored.contains(path)
    }
}

fn main() {
    // set env_logger
    env_logger::init();
    let e = Cli::parse();
    let ignore = Ignore::new(&e.dir).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    info!("{:?}", &ignore);
    let summary = SummaryItem::new(&e.dir, &ignore).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    info!("{:?}", &summary);
    match summary.gen_summary() {
        Ok(summary) => {
            if let Some(output) = e.output {
                info!("output SUMMARY.md to {}", output);
                if let Err(e) = fs::write(output, summary) {
                    panic!("{}", e);
                }
            } else {
                println!("{}", summary);
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
