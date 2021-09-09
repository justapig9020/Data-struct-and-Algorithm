mod manager;
use interactive::{Interactor, Management, interactive};
use manager::Manager;
use clap::{App, Arg};

fn main() {
    let prog = App::new("AVLTree")
                    .arg(Arg::with_name("input")
                            .help("input script")
                            .short("i")
                            .long("input")
                            .takes_value(true))
                    .arg(Arg::with_name("output")
                            .help("output file")
                            .short("o")
                            .long("output")
                            .takes_value(true))
                    .get_matches();
    let mut manager = Manager::new().unwrap();
    let manager = &mut manager as &mut dyn Management;
    let inter = Interactor::new(manager);
    let input = prog.value_of("input");
    let output = prog.value_of("output");
    interactive(inter, input, output);
}
