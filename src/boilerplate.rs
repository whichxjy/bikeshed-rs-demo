use std::fs;

use crate::html;
use crate::spec::Spec;

pub fn add_header_footer(html: &mut String) {
    // TODO: handle group and status
    let header_path = "boilerplate/header.include";
    let footer_path = "boilerplate/footer.include";
    let header = fs::read_to_string(header_path).expect("Fail to open header file");
    let footer = fs::read_to_string(footer_path).expect("Fail to open footer file");
    *html = [header, html.clone(), footer].join("\n");
}

pub fn add_bikeshed_boilerplate(doc: &mut Spec) {
    // TODO: insert <style> nodes to body and move them to head later
    for (key, value) in doc.extra_styles.iter() {
        doc.head.as_ref().unwrap().append(html::node::new_style(
            (format!("/* style-{} */\n", key) + value).as_str(),
        ));
    }
}
