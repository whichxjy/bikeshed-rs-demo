use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::boilerplate;
use crate::line::Line;
use crate::metadata::metadata::{self, MetadataManager};

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Spec<'a> {
    infile: &'a str,
    lines: Vec<Line>,
    mm: Option<MetadataManager>,
    mm_baseline: MetadataManager,
    mm_document: Option<MetadataManager>,
    mm_command_line: MetadataManager,
    html: String,
}

impl<'a> Spec<'a> {
    pub fn new(infile: &str) -> Spec {
        let lines = Spec::read_lines_from_source(infile);

        let mut mm_baseline = MetadataManager::new();
        mm_baseline.add_data(
            &String::from("Date"),
            &String::from("TODO: current time"),
            None,
        );

        Spec {
            infile: infile,
            lines: lines,
            mm: None,
            mm_baseline: mm_baseline,
            mm_document: None,
            mm_command_line: MetadataManager::new(),
            html: String::new(),
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
            let (mm, new_lines) = metadata::parse(&self.lines);
            self.lines = new_lines;
            self.mm_document = Some(mm);
        }
        self.mm = Some(MetadataManager::join_all(&[
            &self.mm_baseline,
            self.mm_document.as_ref().unwrap(),
            &self.mm_command_line,
        ]));
        self.html = self
            .lines
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<String>>()
            .join("\n");
        boilerplate::add_header_footer(&mut self.html);
        println!("{:?}", self.mm);
        println!("{}", self.html);
    }
}
