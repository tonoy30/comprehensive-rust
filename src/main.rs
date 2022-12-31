fn main() { // program entry point
    let mut x: i32 = 6; // mutable variable binding
    print!("{x}"); // marco for printing
    while x != 1 { // no parenthesis around expression
        if x % 2 == 0 { // math like other language
            x = x / 2;
        }else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}
