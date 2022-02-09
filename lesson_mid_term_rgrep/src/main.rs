use std::{env, fs};
use std::env::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use colored::Colorize;
use globset::{Glob, GlobMatcher};
use regex::{Match, Regex};

fn main() -> std::io::Result<()> {
    let opts: Opts = env::args().try_into().unwrap();
    let paths = fs::read_dir(&Path::new(&env::current_dir()?))?;
    paths.filter_map(|entry| { entry.ok() })
        .filter(|entry| { opts.file_matcher.is_match(entry.file_name()) })
        .filter(|entry| { entry.path().is_file() })
        .for_each(|entry| {
            let path = entry.path();
            let file_searcher = FileSearcher::new(opts.context_regex.clone(), path);
            file_searcher.search();
        });
    Ok(())
}

/// 文件查找器
struct FileSearcher {
    context: Regex,
    file: PathBuf,
}

impl FileSearcher {
    pub fn new(context: Regex, file: PathBuf) -> Self {
        FileSearcher { context, file }
    }

    fn search(&self) {
        self.print_file();
        let mut row = 1;
        let file = File::open(&self.file).unwrap();
        for line in BufReader::new(file).lines() {
            if let Ok(context) = line {
                let str = context.as_str();
                if let Some(m) = self.context.find(str) {
                    self.print_result(row, &context, &m);
                }
            }
            row += 1;
        }
    }

    fn print_file(&self) {
        println!("{}", self.file.file_name().unwrap().to_str().unwrap());
    }

    fn print_result(&self, row: i32, context: &String, m: &Match) {
        let start = m.start();
        let end = m.end();
        println!("  {}:{} {}{}{}", row.to_string().blue(), start.to_string().blue(), &context[..start], &context[start..end].to_string().blue(), &context[end..]);
    }
}

/// rgrep: 文本查找工具
struct Opts {
    /// 文本内容；支持正则
    context_regex: Regex,
    /// 文件名；支持通配符
    file_matcher: GlobMatcher,
}

impl TryFrom<Args> for Opts {
    type Error = String;

    fn try_from(s: Args) -> Result<Self, Self::Error> {
        let args: Vec<String> = s.collect();
        if args.len() < 3 {
            return Err("参数数量错误.".into());
        }
        Ok(Self {
            context_regex: Regex::new(args[1].as_str()).unwrap(),
            file_matcher: Glob::new(args[2].as_str()).unwrap().compile_matcher(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_works() {
        let matcher = Glob::new("aa.rs").unwrap().compile_matcher();
        assert!(matcher.is_match("aa.rs"));
        assert!(!matcher.is_match("aa1.rs"));
    }

    #[test]
    fn regex_works() {
        let re = Regex::new(r"Hel[^\\s]+").unwrap();
        assert!(re.is_match("Hello world."));
        let re = Regex::new(r"Hello").unwrap();
        assert!(re.is_match("Hello world."));
    }
}
