use std::io;

fn main() {
    loop {
        println!("Please input a temperature in fahrenheit.");

        let mut temp_f = String::new();

        io::stdin()
            .read_line(&mut temp_f)
            .expect("Failed to read line");

        // we want temp to be a signed integer, since it can be negative or positive
        let temp_f: i32 = match temp_f.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("You inputted {}℉", temp_f);

        let temp_c: i32 = ((temp_f - 32) * 5) % 9;

        println!("that's {}℃", temp_c);
    }
}
