fn main() {
    let first = 22;
    let second = 33;

    let sum = add(first, second);

    println!("the sum of of {first} and {second} are {sum}");

    // a string could be retrieved from input
    // so undefined size at compile time
    let s1: String = String::from("hello");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
