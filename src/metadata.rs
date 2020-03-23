use regex::Regex;

use crate::line::Line;

#[derive(Debug)]
pub struct MetadataManager {
    pub has_metadata: bool,
    pub title: Option<String>,
}

impl MetadataManager {
    pub fn new() -> MetadataManager {
        MetadataManager {
            has_metadata: false,
            title: None,
        }
    }

    pub fn add_data(&self, key: &String, val: &String, line_num: u32) {
        println!("key: {}, val: {}, line_num: {}", key, val, line_num);
    }
}

pub fn parse(lines: &Vec<Line>) {
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
                    .map_or(String::new(), |m| m.as_str().to_string());
                let val = caps
                    .get(2)
                    .map_or(String::new(), |m| m.as_str().to_string());
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
            new_lines.push(Line {
                index: line.index,
                text: line.text.clone(),
            });
        } else {
            // handle lines that do not contain metadata
            new_lines.push(Line {
                index: line.index,
                text: line.text.clone(),
            });
        }
    }

    println!("{:?} \n {:?}", md, new_lines);
}
