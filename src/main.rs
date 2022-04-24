use std::io;
fn main() {
    loop {
        let mut input = String::new();
        let r = io::stdin()
            .read_line(&mut input)
            .expect("please input valid");
        // if let Ok(_) = r {
        if input.trim() == "q" {
            break;
        }
        println!("{}", input);
        // }
        // match r {
        //     Ok(a) => {
        //         println!("{}", a);
        //         if input.trim() == "q" {
        //             break;
        //         }
        //         println!("{}", input);
        //     }
        //     Err(e) => {}
        // }
    }
}
