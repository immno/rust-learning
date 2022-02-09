use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Stdout, Write};
use std::ops::Range;
use std::path::Path;

use clap::Parser;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

pub use error::GrepError;
use colored::*;

mod error;

pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;

/// 简化版的grep，支持正则表达式和文件通配符
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Tyr Chen <tyr@chen.com>")]
pub struct GrepConfig {
    /// 用于查找的正则表达式
    pattern: String,

    /// 文件通配符
    glob: String,
}

impl GrepConfig {
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        let files: Vec<_> = glob::glob(&self.glob)?.collect();
        files.into_par_iter().for_each(|f| {
            if let Ok(filename) = f {
                if let Ok(file) = File::open(&filename) {
                    let reader = BufReader::new(file);
                    let mut stdout = io::stdout();

                    if let Err(e) = strategy(filename.as_path(), reader, &regex, &mut stdout) {
                        println!("Internal error: {}", e);
                    }
                }
            }
        });
        Ok(())
    }
}

pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W,
) -> Result<(), GrepError> {
    let matches = reader.lines().enumerate()
        .map(|(linno, line)| {
            line.ok().map(|line| {
                pattern.find(&line).map(|m| format_line(&line, linno + 1, m.range()))
            })
                .flatten()
        })
        .filter_map(|v| v.ok_or(()).ok())
        .join("\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes())?;
        writer.write(b"\n")?;
        writer.write(matches.as_bytes())?;
        writer.write(b"\n")?;
    }

    Ok(())
}


pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn format_line_should_work() {
        let result = format_line("Hello, Tyr~", 1000, 7..10);
        let expected = format!(
            "{0: >6}:{1: <3} Hello, {2}~",
            "1000".blue(),
            "8".cyan(),
            "Tyr".red()
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn default_strategy_should_work() {
        let path = Path::new("src/main.rs");
        let input = b"hello world!\nhey Tyr!";
        let reader = BufReader::new(&input[..]);
        let pattern = Regex::new(r"he\\w+").unwrap();
        let mut writer = Vec::new();
        default_strategy(path, reader, &pattern, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        let expected = [
            String::from("src/main.rs"),
            format_line("hello world!", 1, 0..5),
            format_line("hey Tyr!\n", 2, 0..3),
        ];

        assert_eq!(result, expected.join("\n"));
    }
}