
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

//////////////////////////////////////////////////////////////////////////////
// Doctopt usage & args

static USAGE_STR: &'static str = "
Usage: wc [options] <file> ...

Options:
    -c, --bytes            print the byte counts
    -m, --chars            print the character counts
    -l, --lines            print the newline counts
    -w, --words            print the word counts
    -L, --max-line-length  print the length of the longest line
    -h, --help             display this help and exit
    -v, --version          output version information and exit
";

#[derive(Debug,RustcDecodable)]
struct Args {
    arg_file: Option<Vec<String>>,
    flag_bytes: bool,
    flag_chars: bool,
    flag_lines: bool,
    flag_words: bool,
    flag_max_line_length: bool,
}


//////////////////////////////////////////////////////////////////////////////
//

fn main() {
    let args: Args = Docopt::new(USAGE_STR).and_then(|d| d.decode())
                                           .unwrap_or_else(|e| e.exit());
    // the previous line is the same as
    // let docopt = match Docopt::new(USAGE_STR) {
    //    Ok(d) => d,
    //    Err(e) => e.exit(),
    // };
    // let args: ARgs = match docopt.decode() {
    //    Ok(args) => args,
    //    Err(e) => e.exit(),
    // };
    println!("{0:?}", args);
}


//////////////////////////////////////////////////////////////////////////////
// word count

//fn wc(file: )


//////////////////////////////////////////////////////////////////////////////
// Utility functions

const CR: u8 = '\r' as u8;
const LF: u8 = '\n' as u8;
const SPACE: u8 = ' ' as u8;
const TAB: u8 = '\t' as u8;
const FF: u8 = 0x0C as u8;  // \f
const SYN: u8 = 0x16 as u8;


#[inline(always)]
fn is_space(byte: u8) -> bool {
    byte == CR || byte == LF || byte == SPACE || byte == TAB || byte == FF
}

#[inline(always)]
fn is_word_separator(byte: u8) -> bool {
    is_space(byte) || byte == SYN
}
