use std::fs;

pub fn add_header_footer(html: &mut String) {
    // TODO: handle group and status
    let header_path = "boilerplate/header.include";
    let footer_path = "boilerplate/footer.include";
    let header = fs::read_to_string(header_path).expect("Fail to open header file");
    let footer = fs::read_to_string(footer_path).expect("Fail to open footer file");
    *html = [header, html.clone(), footer].join("\n");
}
