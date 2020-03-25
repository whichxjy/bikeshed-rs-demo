use regex::Regex;
use std::collections::HashSet;
use titlecase::titlecase;

use super::join::Joinable;
use super::parse;
use crate::line::Line;

#[derive(Debug, Default)]
pub struct Metadata {
    abs: Option<Vec<String>>,
    date: Option<String>,
    ed: Option<String>,
    editors: Option<Vec<String>>,
    group: Option<String>,
    level: Option<String>,
    shortname: Option<String>,
    status: Option<String>,
    title: Option<String>,
}

#[derive(Debug)]
pub struct MetadataManager {
    pub has_metadata: bool,
    pub title: Option<String>,
    pub manually_set_keys: HashSet<String>,
    pub data: Metadata,
}

impl MetadataManager {
    pub fn new() -> MetadataManager {
        MetadataManager {
            has_metadata: false,
            title: None,
            manually_set_keys: HashSet::new(),
            data: Default::default(),
        }
    }

    #[allow(unused_variables)]
    pub fn join_all(sources: &[&MetadataManager]) -> MetadataManager {
        let mut mm = MetadataManager::new();
        mm.has_metadata = sources.iter().any(|&s| s.has_metadata);

        for source in sources {}

        mm
    }

    pub fn add_data(&mut self, key: &String, val: &String, line_num: u32) {
        let mut key = key.trim().to_string();

        if key != "ED" && key != "TR" && key != "URL" {
            key = titlecase(&key);
        }

        match key.as_str() {
            "Abstract" => {
                self.data.abs = self.data.abs.join(Some(parse::parse_vec(val)));
            }
            "Date" => {
                self.data.date = Some(parse::parse_date(val));
            }
            "ED" => {
                self.data.ed = Some(val.clone());
            }
            "Editor" => {
                self.data.editors = self.data.editors.join(Some(parse::parse_editor(val)));
            }
            "Group" => {
                self.data.group = Some(val.clone());
            }
            "Level" => {
                self.data.level = Some(parse::parse_level(val));
            }
            "Shortname" => {
                self.data.shortname = Some(val.clone());
            }
            "Status" => {
                self.data.status = Some(val.clone());
            }
            "Title" => {
                self.data.title = Some(val.clone());
            }
            _ => eprintln!("Unknown metadata key \"{}\" at line {}", key, line_num),
        }
    }
}

pub fn parse(lines: &Vec<Line>) -> (MetadataManager, Vec<Line>) {
    let mut mm = MetadataManager::new();
    let mut new_lines: Vec<Line> = Vec::new();
    let mut in_metadata = false;
    let mut last_key: Option<String> = None;

    let title_reg = Regex::new(r"\s*<h1[^>]*>(.*?)</h1>").unwrap();
    let begin_tag_reg = Regex::new(r"<(pre|xmp) [^>]*class=[^>]*metadata[^>]*>").unwrap();
    let mut end_tag_reg: Option<Regex> = None;
    let pair_reg = Regex::new(r"([^:]+):\s*(.*)").unwrap();

    for line in lines {
        if !in_metadata && begin_tag_reg.is_match(&line.text) {
            // handle begin tag
            in_metadata = true;
            mm.has_metadata = true;
            if line.text.starts_with("<pre") {
                end_tag_reg = Some(Regex::new(r"</pre>\s*").unwrap());
            } else {
                end_tag_reg = Some(Regex::new(r"</xmp>\s*").unwrap());
            }
        } else if in_metadata && end_tag_reg.as_mut().unwrap().is_match(&line.text) {
            // handle end tag
            in_metadata = false;
        } else if in_metadata {
            if last_key.is_some() && line.text.trim().is_empty() {
                // if the line is empty, continue the previous key
                mm.add_data(last_key.as_mut().unwrap(), &line.text, line.index);
            } else if pair_reg.is_match(&line.text) {
                // handle key-val pair
                let caps = pair_reg.captures(&line.text).unwrap();
                let key = caps
                    .get(1)
                    .map_or(String::new(), |k| k.as_str().to_string());
                let val = caps
                    .get(2)
                    .map_or(String::new(), |v| v.as_str().to_string());
                mm.add_data(&key, &val, line.index);
                last_key = Some(key);
            } else {
                // wrong key-val pair
                eprintln!("Incorrectly formatted metadata line: {}", line.index);
            }
        } else if title_reg.is_match(&line.text) {
            // handle title
            if mm.title.is_none() {
                let caps = title_reg.captures(&line.text).unwrap();
                let title = caps
                    .get(1)
                    .map_or(String::new(), |m| m.as_str().to_string());
                mm.add_data(&String::from("Title"), &title, line.index);
            }
            new_lines.push(line.clone());
        } else {
            // handle lines that do not contain metadata
            new_lines.push(line.clone());
        }
    }

    (mm, new_lines)
}
