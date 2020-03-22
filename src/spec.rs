use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::line::Line;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Spec<'a> {
    infile: &'a str,
    lines: Vec<Line>,
}

impl<'a> Spec<'a> {
    pub fn new(infile: &str) -> Spec {

        let mut lines: Vec<Line> = Vec::new();

        if let Ok(src_lines) = read_lines(infile) {
            for (index, src_line) in src_lines.enumerate() {
                if let Ok(text) = src_line {
                    lines.push(Line { index: (index as u32) + 1, text: text });
                }
            }
        }

        Spec { infile: infile, lines: lines }
    }
}