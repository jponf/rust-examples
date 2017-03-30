
use std::env;
use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::process;


fn main() {
    // check number of arguments
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(-1);
    }

    // open file within a buffered reader
    println!("**** Opening: {}", args[1]);
    let file_buf = match fs::File::open(&args[1]) {
        Err(why) => { 
            println!("[1] Error: {}", why.description());
            process::exit(-1);
        },
        Ok(file) => BufReader::new(file),
    };

    // read and print lines
    println!("**** File content ****");
    for (i, line) in file_buf.lines().enumerate() {
        match line {
            Err(why) => {
                println!("[2] Error: {}", why.description());
                process::exit(-1);
            },
            Ok(text) => {
                let line_no = format!("{}:", i);
                println!("{:<6} {}", line_no, text);
            },
        };
    }
}