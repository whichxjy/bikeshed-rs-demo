use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::line::Line;
use crate::metadata::metadata::{self, MetadataManager};

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
    md: Option<MetadataManager>,
    md_baseline: MetadataManager,
    md_document: Option<MetadataManager>,
    md_command_line: MetadataManager,
    html: Option<String>,
}

impl<'a> Spec<'a> {
    pub fn new(infile: &str) -> Spec {
        let lines = Spec::read_lines_from_source(infile);

        let mut md_baseline = MetadataManager::new();
        md_baseline.add_parsed_data(&String::from("Date"), &String::from("TODO: current time"));

        Spec {
            infile: infile,
            lines: lines,
            md: None,
            md_baseline: md_baseline,
            md_document: None,
            md_command_line: MetadataManager::new(),
            html: None,
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
            // println!("{:?} \n {:?}", self.md_document, self.lines);
        }
        self.md = Some(MetadataManager::join_all(&[
            &self.md_baseline,
            self.md_document.as_ref().unwrap(),
            &self.md_command_line,
        ]));
        self.html = Some(
            self.lines
                .iter()
                .map(|l| l.text.clone())
                .collect::<Vec<String>>()
                .join("\n"),
        );
        println!("{}", self.html.as_ref().unwrap());
    }
}
