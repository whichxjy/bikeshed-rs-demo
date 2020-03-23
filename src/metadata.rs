use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use titlecase::titlecase;

use crate::line::Line;

#[derive(Debug)]
enum JoinType {
    Value,
    List,
}

#[derive(Debug)]
pub enum ParseType {
    Date,
    Editor,
    Literal,
    Level,
    LiteralList,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseResult {
    Literal(String),
    LiteralVec(Vec<String>),
}

fn parse_value(parse_type: &ParseType, val: &String) -> Option<ParseResult> {
    match parse_type {
        ParseType::Date => {
            // TODO
            Some(ParseResult::Literal(String::from("TODO: ParseType::Date")))
        }
        ParseType::Editor => {
            // TODO
            Some(ParseResult::Literal(String::from(
                "TODO: ParseType::Editor",
            )))
        }
        ParseType::Literal => Some(ParseResult::Literal(val.clone())),
        ParseType::Level => {
            let val = val.to_lowercase().trim().to_string();
            if val == "none" {
                Some(ParseResult::Literal(String::new()))
            } else {
                Some(ParseResult::Literal(val))
            }
        }
        _ => None,
    }
}

#[derive(Debug)]
struct Metadata<'a> {
    human_name: &'a str,
    attr_name: &'a str,
    join_type: JoinType,
    parse_type: ParseType,
}

impl<'a> Metadata<'a> {
    pub fn new(
        human_name: &'a str,
        attr_name: &'a str,
        join_type: JoinType,
        parse_type: ParseType,
    ) -> Metadata<'a> {
        Metadata {
            human_name: human_name,
            attr_name: attr_name,
            join_type: join_type,
            parse_type: parse_type,
        }
    }
}

lazy_static! {
    static ref KNOWN_KEYS: HashMap<&'static str, Metadata<'static>> = {
        let mut known_keys = HashMap::new();
        known_keys.insert(
            "Abstract",
            Metadata::new(
                "Abstract",
                "abstract",
                JoinType::List,
                ParseType::LiteralList,
            ),
        );
        known_keys.insert(
            "Date",
            Metadata::new("Date", "date", JoinType::Value, ParseType::Date),
        );
        known_keys.insert(
            "ED",
            Metadata::new("ED", "ED", JoinType::Value, ParseType::Literal),
        );
        known_keys.insert(
            "Editor",
            Metadata::new("Editor", "editors", JoinType::List, ParseType::Editor),
        );
        known_keys.insert(
            "Group",
            Metadata::new("Group", "group", JoinType::Value, ParseType::Literal),
        );
        known_keys.insert(
            "Level",
            Metadata::new("Level", "level", JoinType::Value, ParseType::Level),
        );
        known_keys.insert(
            "Shortname",
            Metadata::new(
                "Shortname",
                "displayShortname",
                JoinType::Value,
                ParseType::Literal,
            ),
        );
        known_keys.insert(
            "Status",
            Metadata::new("Status", "rawStatus", JoinType::Value, ParseType::Literal),
        );
        known_keys.insert(
            "Title",
            Metadata::new("Title", "title", JoinType::Value, ParseType::Literal),
        );
        known_keys
    };
}

#[derive(Debug)]
pub struct MetadataManager {
    pub has_metadata: bool,
    pub title: Option<String>,
    pub manually_set_keys: HashSet<String>,
}

impl MetadataManager {
    pub fn new() -> MetadataManager {
        MetadataManager {
            has_metadata: false,
            title: None,
            manually_set_keys: HashSet::new(),
        }
    }

    pub fn add_data(&mut self, key: &String, val: &String, line_num: u32) {
        let mut key = key.trim().to_string();

        if key != "ED" && key != "TR" && key != "URL" {
            key = titlecase(&key);
        }

        if KNOWN_KEYS.contains_key(key.as_str()) {
            let parse_type: &ParseType = &KNOWN_KEYS.get(key.as_str()).unwrap().parse_type;

            if let Some(parse_result) = parse_value(parse_type, val) {
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
    }
}

pub fn parse(lines: &Vec<Line>) -> (MetadataManager, Vec<Line>) {
    let mut md = MetadataManager::new();
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
            md.has_metadata = true;
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
                md.add_data(last_key.as_mut().unwrap(), &line.text, line.index);
            } else if pair_reg.is_match(&line.text) {
                // handle key-val pair
                let caps = pair_reg.captures(&line.text).unwrap();
                let key = caps
                    .get(1)
                    .map_or(String::new(), |k| k.as_str().to_string());
                let val = caps
                    .get(2)
                    .map_or(String::new(), |v| v.as_str().to_string());
                md.add_data(&key, &val, line.index);
                last_key = Some(key);
            } else {
                // wrong key-val pair
                eprintln!("Incorrectly formatted metadata line: {}", line.index);
            }
        } else if title_reg.is_match(&line.text) {
            // handle title
            if md.title.is_none() {
                let caps = title_reg.captures(&line.text).unwrap();
                let title = caps
                    .get(1)
                    .map_or(String::new(), |m| m.as_str().to_string());
                md.add_data(&"Title".to_string(), &title, line.index);
            }
            new_lines.push(line.clone());
        } else {
            // handle lines that do not contain metadata
            new_lines.push(line.clone());
        }
    }

    (md, new_lines)
}
