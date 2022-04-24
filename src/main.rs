use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let i1 = io::stdin().read_line(&mut input1);
    let i2 = io::stdin().read_line(&mut input2);
    let i3 = io::stdin().read_line(&mut input3);

    let i1: i32 = input1.trim().parse().expect("failed");
    println!("i1 {:?}", i1);
    println!("i2 {:?}", input2);
    let i2: i32 = input2.trim().parse().expect("fail");
    println!("i2 {:?}", i2);
    let i3 = input3.trim();
    let output = match i3 {
        "+" => i1 + i2,
        "/" => i1 / i2,
        "*" => i1 * i2,
        "-" => i1 - i2,
        _ => 0,
    };

    println!("output {}", output);
}
