use std::io;
fn main() {
    loop {
        let mut input = String::new();
        let r = io::stdin().read_line(&mut input);
        match r {
            Ok(a) => {
                if input.trim() == "q" {
                    break;
                }
                println!("{}", input);
            }
            Err(e) => {}
        }
    }
}
