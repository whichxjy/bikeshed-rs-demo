use kuchiki::NodeRef;
use markup5ever::QualName;

pub fn new_style(text: &str) -> NodeRef {
    let el = NodeRef::new_element(QualName::new(None, ns!(html), local_name!("style")), None);
    el.append(NodeRef::new_text(text));
    el
}
