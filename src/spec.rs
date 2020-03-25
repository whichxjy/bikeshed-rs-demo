use std::collections::HashMap;
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
    mm: MetadataManager,
    mm_baseline: MetadataManager,
    mm_document: MetadataManager,
    mm_command_line: MetadataManager,
    pub macros: HashMap<&'a str, String>,
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
            mm: MetadataManager::new(),
            mm_baseline: mm_baseline,
            mm_document: MetadataManager::new(),
            mm_command_line: MetadataManager::new(),
            macros: HashMap::new(),
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
        let (mm_document, lines) = metadata::parse(&self.lines);
        self.mm_document = mm_document;
        self.lines = lines;

        let mm = MetadataManager::join_all(&[
            &self.mm_baseline,
            &self.mm_document,
            &self.mm_command_line,
        ]);
        mm.fill_macros(self);
        mm.validate();
        self.mm = mm;
        println!("{:?}", self.mm);
        println!("{:?}", self.macros);

        self.html = self
            .lines
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<String>>()
            .join("\n");
        boilerplate::add_header_footer(&mut self.html);

        // println!("{}", self.html);
    }
}
