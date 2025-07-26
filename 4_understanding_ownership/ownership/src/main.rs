fn main() {
    println!("# integer");
    let a = 42;
    let b = a;
    println!("{}", a);
    println!("{}", b);

    println!("# string moves");
    let s1 = String::from("hello");
    let s2 = s1;  // value moved here
    // println!("{}", s1);  // s1 is no longer valid
    println!("{}", s2);

    println!("# string cloned");
    let s3 = s2.clone(); // deep copy with clone
    println!("{}", s2);
    println!("{}", s3);


    println!("# functions");
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
