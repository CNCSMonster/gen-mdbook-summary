use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};
use log::info;

/// 对路径进行简单转义，只处理空格
/// mdbook 的 SUMMARY.md 使用文件系统路径，不是 URL，不需要完整的 URL 编码
fn escape_path(path: &str) -> String {
    // mdbook 支持 Unicode 文件名，只需转义空格
    path.replace(' ', "%20")
}

#[derive(Debug)]
pub struct SummaryItem {
    name: String,
    path: PathBuf,          // 相对路径
    absolute_path: PathBuf, // 绝对路径（用于文件系统操作）
    introduction: Option<String>,
    chapters: Vec<SummaryItem>,
}

impl SummaryItem {
    pub fn new(dir: &str, ignore: &Ignore, base_dir: &Path) -> anyhow::Result<Self> {
        info!("try to create SummaryItem from {}", dir);
        let mut chapters = Vec::new();
        let absolute_path = Path::new(dir).canonicalize()?;

        // 计算相对路径
        let relative_path = absolute_path
            .strip_prefix(base_dir)
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| absolute_path.clone());

        // 如果相对路径为空，使用 "." 表示当前目录
        let relative_path = if relative_path.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            relative_path
        };

        let dir = absolute_path.display().to_string();
        let meta = std::fs::metadata(&dir)?;
        let name = Self::item_name_from_path_str(&dir)?.to_string();

        if meta.is_file() {
            return Ok(Self {
                name,
                path: relative_path,
                absolute_path,
                introduction: None,
                chapters,
            });
        }

        // check if the dir is a directory
        if !meta.is_dir() {
            bail!("{} is neither a file nor a directory", dir);
        }
        info!("{dir}");
        // check introduction
        let mut introduction: Option<String> = None;
        let names = vec!["README.md", "readme.md", "README", "readme"];
        // check if the introduction file exists
        for readme_name in names {
            let path = format!("{}/{}", dir, readme_name);
            if Path::new(&path).exists() && !ignore.is_ignore(&path) {
                // 存储相对路径
                let intro_absolute = Path::new(&path).canonicalize()?;
                let intro_relative = intro_absolute
                    .strip_prefix(base_dir)
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|_| intro_absolute.clone());

                // 如果相对路径为空，使用 "." 表示当前目录
                let intro_relative = if intro_relative.as_os_str().is_empty() {
                    PathBuf::from(".")
                } else {
                    intro_relative
                };

                introduction = Some(intro_relative.to_string_lossy().to_string());
                break;
            }
        }

        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let path_str = &path.display().to_string();
            info!("{}", path_str);
            if ignore.is_ignore(path_str) {
                info!("ignore {}", path_str);
                continue;
            }
            if path_str.ends_with("readme.md") || path_str.ends_with("README.md") {
                info!("path_str {} is readme.md,skip", path_str);
                continue;
            }
            chapters.push(Self::new(path_str, ignore, base_dir)?);
        }
        let path = relative_path;
        Ok(Self {
            name,
            path,
            absolute_path,
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

    pub fn gen_summary(&self) -> anyhow::Result<String> {
        let mut summary = String::new();
        summary.push_str("# Summary\n\n");
        if self.absolute_path.is_dir() {
            for chapter in &self.chapters {
                summary.push('\n');
                summary.push_str(&chapter.item(0)?);
            }
        } else {
            let path_str = self
                .path
                .to_str()
                .with_context(|| format!("[{}:{}]", file!(), line!(),))?;
            summary.push_str(&format!("- [{}]({})", self.name, path_str));
        }
        Ok(summary)
    }

    pub fn item(&self, depth: usize) -> anyhow::Result<String> {
        let mut item = String::new();
        for _ in 0..depth {
            item.push('\t');
        }
        let path_str = self
            .path
            .to_str()
            .with_context(|| format!("[{}:{}]", file!(), line!(),))?;

        // 只转义空格，mdbook 支持 Unicode 文件名
        let encoded_path = escape_path(path_str);

        if self.absolute_path.is_dir() {
            if let Some(introduction) = &self.introduction {
                let encoded_intro = escape_path(introduction);
                item.push_str(format!("- [{}]({})", &self.name, encoded_intro).as_str());
            } else {
                item.push_str(format!("- [{}]()", &self.name).as_str());
            }
            for chapter in self.chapters.iter() {
                item.push('\n');
                item.push_str(&chapter.item(depth + 1)?);
            }
        } else {
            item.push_str(&format!("- [{}]({})", &self.name, encoded_path));
        }
        info!("{item}");
        Ok(item)
    }

    fn item_name_from_path_str(path_name: &str) -> anyhow::Result<String> {
        let name = path_name
            .split("/")
            .last()
            .ok_or(anyhow::anyhow!(
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
    pub fn new(dir: &str, ignore_file: &str) -> anyhow::Result<Self> {
        use ignore::WalkBuilder;
        let mut unignored = HashSet::new();
        for result in WalkBuilder::new(dir)
            .add_custom_ignore_filename(ignore_file)
            .build()
        {
            let result = result?;
            let path = result.path().canonicalize()?;
            let path = path
                .to_str()
                .with_context(|| format!("[{}:{}]", file!(), line!(),))?;
            unignored.insert(path.to_string());
        }
        Ok(Self { unignored })
    }

    pub fn is_ignore(&self, path: &str) -> bool {
        !self.unignored.contains(path)
    }
}

#[cfg(test)]
mod tests {
    use super::SummaryItem;

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
