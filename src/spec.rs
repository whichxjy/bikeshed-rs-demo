use std::collections::HashMap;

use crate::boilerplate;
use crate::html::helper;
use crate::line::Line;
use crate::metadata::metadata::{self, MetadataManager};
use crate::util::reader;

#[derive(Debug)]
pub struct Spec<'a> {
    infile: &'a str,
    lines: Vec<Line>,
    mm: MetadataManager,
    mm_baseline: MetadataManager,
    mm_document: MetadataManager,
    mm_command_line: MetadataManager,
    pub macros: HashMap<&'static str, String>,
    html: String,
}

impl<'a> Spec<'a> {
    pub fn new(infile: &str) -> Spec {
        let lines = Spec::read_lines_from_source(infile);

        let mut mm_baseline = MetadataManager::new();
        mm_baseline.add_data("Date", &String::from("now"), None);

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
        if let Ok(src_lines) = reader::read_lines(infile) {
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
        let (mm_document, lines) = metadata::parse_metadata(&self.lines);
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

        self.html = self
            .lines
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<String>>()
            .join("\n");
        boilerplate::add_header_footer(&mut self.html);

        self.html = helper::replace_macros(&self.html, &self.macros);
        // println!("{:?}", self.mm);
        // println!("{:?}", self.macros);
        // println!("{}", self.html);
    }
}
