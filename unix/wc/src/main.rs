
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::ascii::AsciiExt;
use std::io::{BufRead, BufReader, Read};
use std::result::Result;


//////////////////////////////////////////////////////////////////////////////
// Doctopt usage & args

static USAGE_STR: &'static str = "
Usage: 
    wc -h
    wc [options] <files> ...

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
    arg_files: Option<Vec<String>>,
    flag_bytes: bool,
    flag_chars: bool,
    flag_lines: bool,
    flag_words: bool,
    flag_max_line_length: bool,
}


//////////////////////////////////////////////////////////////////////////////
// Other structures

struct WcResult {
    title: String,
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
    max_line_length: usize,
}


//////////////////////////////////////////////////////////////////////////////
//

fn main() {
    let args: Args = parse_command_line_arguments();
    
    let result = match args.arg_files {
        None => { 
            println!("Error: no file");
            std::process::exit(-1);
        },
        Some(ref files_paths) => wc_all(files_paths)
    };

    match result {
        Ok(ref wc_results) => print_results(&args, wc_results),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(-1);
        }
    }
}


fn parse_command_line_arguments() -> Args {
    // the following line is the same as
    // let docopt = match Docopt::new(USAGE_STR) {
    //    Ok(d) => d,
    //    Err(e) => e.exit(),
    // };
    // let args: Args = match docopt.decode() {
    //    Ok(args) => args,
    //    Err(e) => e.exit(),
    // };
    let args: Args = Docopt::new(USAGE_STR).and_then(|d| d.decode())
                                           .unwrap_or_else(|e| e.exit());

    if args.flag_bytes || args.flag_chars || args.flag_lines
            || args.flag_words || args.flag_max_line_length {
        return args;
    }

    Args {
        arg_files: args.arg_files,
        flag_bytes: true,
        flag_chars: true,
        flag_lines: true,
        flag_words: true,
        flag_max_line_length: false
    }
}



//////////////////////////////////////////////////////////////////////////////
// word count

fn wc_all(files_paths: &Vec<String>) -> Result<Vec<WcResult>, std::io::Error> {
    let mut total_byte_count: usize = 0;
    let mut total_char_count: usize = 0;
    let mut total_line_count: usize = 0;
    let mut total_word_count: usize = 0;    
    let mut total_longest_line_length: usize = 0;

    let mut results : Vec<WcResult> = Vec::new();

    for f_path in files_paths {
        let result = try!(wc_file(f_path));
        total_byte_count += result.bytes;
        total_char_count += result.chars;
        total_line_count += result.lines;
        total_word_count += result.words;

        total_longest_line_length = std::cmp::max(result.max_line_length, 
                                                  total_longest_line_length);

        results.push(result);
    }

    if files_paths.len() > 1 {
        results.push(WcResult {
            title: "total".to_owned(),
            bytes: total_byte_count,
            chars: total_char_count,
            lines: total_line_count,
            words: total_word_count,
            max_line_length: total_longest_line_length
        });
    }

    return Ok(results);
}


fn wc_file(file_path: &String) -> Result<WcResult, std::io::Error> {
    let mut byte_count: usize = 0;
    let mut char_count: usize = 0;
    let mut line_count: usize = 0;
    let mut word_count: usize = 0;
    let mut longest_line_length: usize = 0;
    let mut raw_line = Vec::new();

    let mut reader = try!(open_buf_reader(file_path));

    while match reader.read_until(LF, &mut raw_line) {
        Ok(n) if n > 0 => true,
        Err(ref e) if !raw_line.is_empty() => {
            println!("Error while reading {}: {}", file_path, e);
            !raw_line.is_empty()
        },
        _ => false,
    } { // while body
        if *raw_line.last().unwrap() == LF {
            line_count += 1;
        }

        byte_count += raw_line.len();

        // try and convert the bytes to utf-8 first
        let num_chars;
        match std::str::from_utf8(&raw_line[..]) {
            Ok(line) => {
                word_count += line.split_whitespace().count();
                num_chars = line.chars().count();
            },
            Err(..) => {
                word_count += raw_line.split(|&x| is_word_separator(x)).count();
                num_chars = raw_line.iter().filter(|c| c.is_ascii()).count();
            }
        }
        char_count += num_chars;
        longest_line_length = std::cmp::max(num_chars, longest_line_length);

        raw_line.clear();
    }

    Ok(WcResult {
        title: file_path.clone(),
        bytes: byte_count,
        chars: char_count,
        lines: line_count,
        words: word_count,
        max_line_length: longest_line_length
    })
}


fn print_results(args: &Args, wc_results: &Vec<WcResult>) {
    for wc_res in wc_results {
        print_result(args, wc_res)
    }
}

fn print_result(args: &Args, wc_res: &WcResult) {
    if args.flag_lines {
        print!("{:1$}", wc_res.lines, 2);
    }
}

//////////////////////////////////////////////////////////////////////////////
// WC utility functions

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


fn open_buf_reader(path: &str) 
-> Result<BufReader<Box<Read+'static>>, std::io::Error> {
    if "-" == path {
        let reader = Box::new(std::io::stdin());
        return Ok(BufReader::new(reader));
    }

    let fpath = std::path::Path::new(path);
    if fpath.is_dir() {
        println!("{}: is a directory", path)
    }
    match std::fs::File::open(&fpath) {
        Ok(fd) => {
            let reader = Box::new(fd);
            Ok(BufReader::new(reader))
        }
        Err(e) => Err(e)
    }
}
