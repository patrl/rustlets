use std::io;

fn main() {
    loop {
        println!("Give me an *n*, and I'll give you the *n*th Fibonacci number.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("The {}nth Fibonacci number is: {}", input, fibo(input));

        fn fibo(n: u32) -> u32 {
            if n < 2 {
                n - 1
            } else {
                let mut fst = 0;
                let mut snd = 1;
                let mut counter = 1;
                loop {
                    counter += 1;
                    let old_fst = fst;
                    fst = snd;
                    if counter == n {
                        break fst;
                    }
                    snd = old_fst + snd;
                }
            }
        }
    }
}
