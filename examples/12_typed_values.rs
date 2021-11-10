use clap::App;

fn main() {
    let matches = App::new("myapp")
        // Create two arguments, a required positional which accepts multiple values
        // and an optional '-l value'
        .arg("<seq>... 'A sequence of whole positive numbers, i.e. 20 25 30'")
        .arg("-l [len] 'A length to use, defaults to 10 when omitted'")
        .get_matches();

    // This code loops through all the values provided to "seq" and adds 2
    // If seq fails to parse, the program exits, you don't have an option
    for v in matches
        .values_of_t::<u32>("seq")
        .unwrap_or_else(|e| e.exit())
    {
        println!("Sequence part {} + 2: {}", v, v + 2);
    }

    // Here we get a value of type u32 from our optional -l argument.
    // If the value provided to len fails to parse or not present, we default to 10
    //
    // Using other methods such as unwrap_or_else(|e| println!("{}",e))
    // are possible too.
    let len: u32 = matches.value_of_t("l").unwrap_or(10);

    println!("len ({}) + 2 = {}", len, len + 2);
}
