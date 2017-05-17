
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;


//////////////////////////////////////////////////////////////////////////////
// Doctopt usage & args

static USAGE_STR: &'static str = "
Usage: 
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

    
}


fn parse_command_line_arguments() -> Args {
    let args: Args = Docopt::new(USAGE_STR).and_then(|d| d.decode())
                                           .unwrap_or_else(|e| e.exit());
    args
}