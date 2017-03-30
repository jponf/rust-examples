//
// Classic hello world example
//

macro_rules! say_hello_world {
    // `()` indicates that the macro takes no argument.
    () => (
        println!("Hello world!");
    )
}


fn main() {
    // string slice
    let hello = "Hello";  
    // string slice with splicit type annotation
    let world: &'static str = "world";

    // Rust insert extern crate std by default
    // print & println are part of std  

    print!("[1] Hello world!\n");

    // println, like print but appends \n
    println!("[2] Hello world!");

    // formatting 
    println!("[3] {} {}!", hello, world);

    // formatting using names
    println!("[4] {h} {w}!", w=world, h=hello);

    // using our macro
    print!("[5] ");  // no new line
    say_hello_world!();
}