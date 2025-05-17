use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{anyhow, Result};

use clap::Parser;
use log::{error, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long, default_value = "src")]
    dir: String,
    /// specify the output file
    #[arg(short, long)]
    output: Option<String>,
    /// if organize the items in order
    #[arg(short, long, default_value = "true")]
    sort: bool,
    #[arg(
        short,
        long,
        default_value = "mdbook.ignore",
        help = "specify the ignore file ,using .gitignore grammar,
    matched files will be ignored."
    )]
    ignore: String,
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
        let name = Self::item_name_from_path_str(dir)?.to_string();

        if meta.is_file() {
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
        for readme_name in names {
            let path = format!("{}/{}", dir, readme_name);
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
            chapters.push(Self::new(path_str, ignore)?);
        }
        let path = PathBuf::from(dir);
        Ok(Self {
            name,
            path,
            introduction,
            chapters,
        })
    }

    pub fn sort(&mut self) {
        self.chapters.sort_by(|a, b| a.name.cmp(&b.name));
        for chapter in self.chapters.iter_mut() {
            chapter.sort();
        }
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

    fn item_name_from_path_str(path_name: &str) -> Result<String> {
        let name = path_name
            .split("/")
            .last()
            .ok_or(anyhow!(
                "[{}:{}:{}]invalid name",
                file!(),
                line!(),
                column!()
            ))?
            .trim();

        // remove name's extension
        let name = if Path::new(path_name).is_dir() {
            name.to_string()
        } else {
            let name: Vec<&str> = name.split(".").collect();
            name[..name.len() - 1].join(".")
        };

        Ok(name)
    }
}

#[derive(Debug)]
pub struct Ignore {
    unignored: HashSet<String>,
}

impl Ignore {
    pub fn new(dir: &str, ignore_file: &str) -> Result<Self> {
        use ignore::WalkBuilder;
        let mut unignored = HashSet::new();
        for result in WalkBuilder::new(dir)
            .add_custom_ignore_filename(ignore_file)
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
    let cli = Cli::parse();
    let ignore = Ignore::new(&cli.dir, &cli.ignore).unwrap_or_else(|e| {
        error!("{}", e);
        exit(-1);
    });
    info!("{:?}", &ignore);
    let mut summary = SummaryItem::new(&cli.dir, &ignore).unwrap_or_else(|e| {
        error!("{}", e);
        exit(-1);
    });
    info!("{:?}", &summary);
    if cli.sort {
        info!("sort the summary");
        summary.sort();
    }
    match summary.gen_summary() {
        Ok(summary) => {
            if let Some(output) = cli.output {
                info!("output SUMMARY.md to {}", output);
                if let Err(e) = fs::write(output, summary) {
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
}

#[cfg(test)]
mod tests {
    use crate::SummaryItem;

    #[test]
    fn t_item_name() {
        // enter temp dir
        let temp_dir = std::env::temp_dir();
        let temp_dir = temp_dir.join("test_item_name");
        std::fs::create_dir_all(&temp_dir).unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        fn test(p: &str, is_dir: bool, expect: &str) {
            if is_dir {
                std::fs::create_dir_all(p).unwrap();
            }
            let name = SummaryItem::item_name_from_path_str(p).unwrap();
            if is_dir {
                std::fs::remove_dir_all(p).unwrap();
            }
            assert_eq!(name, expect);
        }

        test("README.md", false, "README");
        test("One.AA.md", false, "One.AA");
        test("blog/Pro/readme.md", false, "readme");
    }
}
