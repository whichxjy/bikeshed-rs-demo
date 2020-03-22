#[derive(Debug)]
pub struct Spec<'a> {
    infile: &'a str,
}

impl<'a> Spec<'a> {
    pub fn new(infile: &str) -> Spec {
        Spec { infile: infile }
    }
}