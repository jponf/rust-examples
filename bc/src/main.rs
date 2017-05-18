
extern crate rustc_serialize;
extern crate docopt;

use std::io::{BufRead, BufReader, Read};

use docopt::Docopt;


mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

//use self::grammar;


//////////////////////////////////////////////////////////////////////////////
// Doctopt usage & args

static USAGE_STR: &'static str = "
Usage: 
    bc
    bc -h | --help
    bc -v | --version

Options:
    -h, --help             Show this message and exit
    -v, --version          output version information and exit
";


#[derive(RustcDecodable)]
struct Args {
}


//////////////////////////////////////////////////////////////////////////////
//

fn main() {
    let args: Args = parse_command_line_arguments();

    let stdin = std::io::stdin();
    
    for line in stdin.lock().lines() {
        match grammar::decimal(line.unwrap().as_str()) {
            Err(e) => println!("{}", e),
            Ok(dec) => println!("Decimal: {}", dec)
        }
    }
    
    println!("Done...");
    std::process::exit(0);    
}


fn parse_command_line_arguments() -> Args {
    let args: Args = Docopt::new(USAGE_STR).and_then(|d| d.decode())
                                           .unwrap_or_else(|e| e.exit());
    args
}
