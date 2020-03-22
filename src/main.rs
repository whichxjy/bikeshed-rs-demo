mod spec;
mod line;
mod metadata;

use clap::{Arg, App, SubCommand};
use spec::Spec;

fn handle_spec(infile: &str) {
    let doc = Spec::new(infile);
    doc.preprocess();
    // println!("{:?}", doc);
}

fn main() {
    let matches = App::new("Bikeshed Demo")
                    .version("1.0")
                    .author("whichxjy")
                    .subcommand(SubCommand::with_name("spec")
                                .about("process a spec source file into a valid output file")
                                .arg(Arg::with_name("infile")
                                    .required(true)
                                    .takes_value(true)
                                    .help("path to the source file")
                                    .index(1)))
                    .get_matches();

    let infile = matches.subcommand_matches("spec").unwrap().value_of("infile").unwrap();
    handle_spec(infile);
}
