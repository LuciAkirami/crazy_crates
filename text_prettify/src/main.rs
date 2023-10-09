mod color;

fn main() {

    //strikeLineThrough
    let s1 = "\x1b[9mHello, world!\x1b[0m";

    //Bold
    // ANSI escape code for bold text is "\x1b[1m"
    // ANSI escape code for resetting text formatting is "\x1b[0m"
    let s2 = "\x1b[1mHello, world!\x1b[0m";

    // ANSI escape code for underlined text is "\x1b[4m"
    // ANSI escape code for resetting text formatting is "\x1b[0m"
    let s3 = "\x1b[4mHello, world!\x1b[0m";

    // ANSI escape code for italic text is "\x1b[3m"
    // ANSI escape code for resetting text formatting is "\x1b[0m"
    let s4 = "\x1b[3mHello, world!\x1b[0m";

    
    println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
    println!("{}",s4);

    println!("----------------");

    color::main_color();

}