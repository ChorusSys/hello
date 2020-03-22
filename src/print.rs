pub fn run() {
    // print to the console
    println!("Hello from print.rs file");

    // print a number
    println!("{}", 1);

    // Basic formatting
    println!("{} is from {}", "Alice", "Seattle");

    // Positional arguments and formatting
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Betsy", "Denver", "code"
    );

    // Named arguments
    println!(
        "{name} lives in {city} and works as a {occupation}",
        name = "Andy",
        city = "Moscow",
        occupation = "linguist"
    );

    // Placeholder traits
    println!(
        "Binary: {0:b}, Octal: {0:o}, Hexadecimal: {0:x}, Decimal: {0}", 256
    );

    // Debug traits
    println!(
        "{:?}", (true, "john", 3.1415, 'x')
    );
}
