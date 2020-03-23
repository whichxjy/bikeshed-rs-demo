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

pub fn parse_value(parse_type: &ParseType, val: &String) -> Option<ParseResult> {
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
