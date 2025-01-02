// fn main() {
//     let mut x = 5; // HERE!
//     println!("The value of x is : {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
