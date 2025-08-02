fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}.");

    let mut s = String::from("hello");
    println!("s will be changed from '{s}'");
    change(&mut s);
    println!("s changed to '{s}'");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped, so its memory goes away.
