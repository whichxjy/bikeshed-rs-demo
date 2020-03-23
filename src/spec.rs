use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::line::Line;
use crate::metadata::{self, MetadataManager};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Spec<'a> {
    infile: &'a str,
    lines: Vec<Line>,
    md_document: Option<MetadataManager>,
}

impl<'a> Spec<'a> {
    pub fn new(infile: &str) -> Spec {
        let lines = Spec::read_lines_from_source(infile);
        Spec {
            infile: infile,
            lines: lines,
            md_document: None,
        }
    }

    fn read_lines_from_source(infile: &str) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        if let Ok(src_lines) = read_lines(infile) {
            for (index, src_line) in src_lines.enumerate() {
                if let Ok(text) = src_line {
                    lines.push(Line {
                        index: 1 + (index as u32),
                        text: text,
                    });
                }
            }
        }
        lines
    }

    pub fn preprocess(&mut self) {
        self.assemble_document();
    }

    fn assemble_document(&mut self) {
        {
            let (md, new_lines) = metadata::parse(&self.lines);
            self.lines = new_lines;
            self.md_document = Some(md);
            println!("{:?} \n {:?}", self.md_document, self.lines);
        }
    }
}
