use clap::App;

fn main() {
    // You can use some convenience methods provided by clap to get typed values, so long as the
    // type you're converting into implements std::str::FromStr
    //
    // This works for both single, and multiple values (multiple values returns a Vec<T>)
    //
    // There are also two ways in which to get types, those where failures cause the program to exit
    // with an error and usage string, and those which return a Result<T,String> or Result<Vec<T>,String>
    // respectively. Both methods support single and multiple values.
    //
    // The method which returns a Result allows you decide what to do upon a failure, exit, provide a
    // default value, etc. You have control. But it also means you have to write the code or boiler plate
    // to handle those instances.
    //
    // That is why the second method exists, so you can simply get a T or Vec<T> back, or be sure the
    // program will exit gracefully. The catch is, the second method should *only* be used on required
    // arguments, because if the argument isn't found, it exits. Just FYI ;)
    //
    // The following example shows both methods.
    let matches = App::new("myapp")
        // Create two arguments, a required positional which accepts multiple values
        // and an optional '-l value'
        .arg("<seq>... 'A sequence of whole positive numbers, i.e. 20 25 30'")
        .arg("-l [len] 'A length to use, defaults to 10 when omitted'")
        .get_matches();

    // Here we get a value of type u32 from our optional -l argument.
    // If the value provided to len fails to parse or not present, we default to 10
    //
    // Using other methods such as unwrap_or_else(|e| println!("{}",e))
    // are possible too.
    let len: u32 = matches.value_of_t("len").unwrap_or(10);

    println!("len ({}) + 2 = {}", len, len + 2);

    // This code loops through all the values provided to "seq" and adds 2
    // If seq fails to parse, the program exits, you don't have an option
    for v in matches
        .values_of_t::<u32>("seq")
        .unwrap_or_else(|e| e.exit())
    {
        println!("Sequence part {} + 2: {}", v, v + 2);
    }
}
