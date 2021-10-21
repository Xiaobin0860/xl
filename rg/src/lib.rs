use colored::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
    ops::Range,
    path::Path,
};

mod error;

pub use error::GrepError;

/// 定义类型，这样，在使用时可以简化复杂类型的书写
pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;

pub struct Greper {
    pattern: String,
    glob: String,
}

impl Greper {
    pub fn new(pattern: String, glob: String) -> Self {
        Self { pattern, glob }
    }

    /// 使用缺省策略来查找匹配
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    /// 简化版本的 grep，支持正则表达式和文件通配符
    pub fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        let files: Vec<_> = glob::glob(&self.glob)?.collect();
        files.into_par_iter().for_each(|v| {
            if let Ok(f) = v {
                if let Ok(file) = File::open(&f) {
                    let reader = BufReader::new(file);
                    let mut stdout = io::stdout();
                    if let Err(e) = strategy(f.as_path(), reader, &regex, &mut stdout) {
                        eprintln!("Internal error: {:?}", e);
                    }
                }
            }
        });
        Ok(())
    }
}

/// 缺省策略，从头到尾串行查找，最后输出到 writer
pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W,
) -> Result<(), GrepError> {
    let name = path.file_name().unwrap().to_str().unwrap();
    reader.lines().enumerate().for_each(|(n, v)| {
        if let Ok(line) = v {
            if let Some(m) = pattern.find(&line) {
                let formated = format_line(name, &line, n + 1, m.range());
                if let Err(e) = writer.write_all(formated.as_bytes()) {
                    eprintln!("Internal write error: {:?}", e);
                }
            }
        }
    });
    Ok(())
}

/// 格式化输出匹配的行，包含行号，列号和带有高亮的第一个匹配项
pub fn format_line(name: &str, line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    format!(
        "{}({}): {}{}{}\n",
        name.cyan(),
        lineno.to_string().blue(),
        &line[..start],
        &line[start..end].red(),
        &line[end..]
    )
}
