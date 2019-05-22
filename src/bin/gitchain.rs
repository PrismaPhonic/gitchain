use std::process;
use std::io::Write;
extern crate gitchain;
use structopt::StructOpt;

fn main() {
    let config = gitchain::Opts::from_args();

    if let Err(ref e) = gitchain::run(config) {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        process::exit(1);
    };
}
