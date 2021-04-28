extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("ls")
        .version("1.0")
        .author("SyedAhkam")
        .about("Lists the contents of a directory")
        .arg(Arg::with_name("dir"))
        .get_matches();

    // TODO
    println!("{:#?}", matches);
}
