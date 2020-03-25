pub fn parse_date(val: &String) -> String {
    String::from("TODO: parse_date") + " " + val
}

pub fn parse_editor(val: &String) -> String {
    String::from("TODO: parse_editor") + " " + val
}

pub fn parse_level(val: &String) -> String {
    if val == "none" {
        String::new()
    } else {
        val.clone()
    }
}

pub fn parse_vec(val: &String) -> Vec<String> {
    let mut vec = Vec::new();
    vec.push(val.clone());
    vec
}
