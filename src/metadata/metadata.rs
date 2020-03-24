use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use titlecase::titlecase;

use super::join::JoinType;
use super::parse::{self, ParseResult, ParseType};
use crate::line::Line;

#[derive(Debug)]
struct KeyType<'a> {
    human_name: &'a str,
    parse_type: ParseType,
    join_type: JoinType,
}

impl<'a> KeyType<'a> {
    pub fn new(human_name: &'a str, parse_type: ParseType, join_type: JoinType) -> KeyType<'a> {
        KeyType {
            human_name: human_name,
            parse_type: parse_type,
            join_type: join_type,
        }
    }
}

lazy_static! {
    static ref KNOWN_KEYS: HashMap<&'static str, KeyType<'static>> = {
        let mut known_keys = HashMap::new();
        known_keys.insert(
            "ED",
            KeyType::new("ED", ParseType::Literal, JoinType::Value),
        );
        known_keys
    };
}

#[derive(Debug, Default)]
pub struct Metadata {
    ed: Option<String>,
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

        if KNOWN_KEYS.contains_key(key.as_str()) {
            let parse_type: &ParseType = &KNOWN_KEYS.get(key.as_str()).unwrap().parse_type;

            if let Some(parse_result) = parse::parse_value(parse_type, val) {
                if let ParseResult::Literal(parsed_val) = parse_result {
                    self.add_parsed_data(&key, &parsed_val);
                }
            }
        } else {
            eprintln!("Unknown metadata key \"{}\" at line {}", key, line_num);
        }
    }

    pub fn add_parsed_data(&mut self, key: &String, val: &String) {
        println!("key: {}, parsed_val: {}", key, val);
        self.manually_set_keys.insert(key.to_owned());
        self.set_key(&key, val);
    }

    // #[allow(dead_code, unused_variables)]
    pub fn set_key<T>(&mut self, key_name: &String, val: T)
    where
        String: From<T>,
    {
        match key_name.as_str() {
            "ED" => self.data.ed = Some(String::from(val)),
            _ => {}
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
                mm.add_data(&"Title".to_string(), &title, line.index);
            }
            new_lines.push(line.clone());
        } else {
            // handle lines that do not contain metadata
            new_lines.push(line.clone());
        }
    }

    (mm, new_lines)
}
