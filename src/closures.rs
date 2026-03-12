fn main() {
    let string = "Hey, there.";

    let closure = || println!("{string}");

    // Closures borrow by default
    closure();
    closure();

    let mut string = "hi";

    closure(); // Holds the existing borrow of "string"

    println!("{string}");

    let closure = || println!("{string}");

    closure();

    // The below would error due to existing borrow
    // string = "";
    // closure();

    let mut string = "Move me!";

    let closure_with_move = move || println!("{string}");

    closure_with_move();

    string = "WHO, MEEEE!?";

    closure_with_move();

    println!("{string}");

    // Closures can be used as args to functions
    // To use closures as arguments in Rust, the function or method receiving the closure must use one of three closure traits as a trait bound on a generic type, or as a trait object (e.g., Box<dyn Fn(...)>).
    //
    // FnOnce: The closure can be called a single time because it takes ownership of its captured variables (moves them out of its environment). This is the least restrictive trait.
    // FnMut: The closure can be called multiple times and might mutate the captured variables (captures by mutable reference &mut T). Closures implementing FnMut also implement FnOnce.
    // Fn: The closure can be called multiple times and only captures variables by immutable reference (&T), meaning it can be called concurrently or in immutable contexts. This is the most restrictive trait.
}
