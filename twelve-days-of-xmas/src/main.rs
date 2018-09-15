fn main() {
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let ord = ["ˢᵗ", "ⁿᵈ", "ʳᵈ", "ᵗʰ"];

    let mut index = 0;

    while index < 11 {
        println!(
            "On the {}{} day of christmas my true love sent to me",
            index + 1,
            if index < 3 { ord[index] } else { ord[3] }
        );

        let mut countdown = index + 1;

        while countdown != 0 {
            if countdown == 1 {
                println!("a {}", gifts[countdown - 1]);
            } else {
                println!("{} {}", countdown, gifts[countdown - 1]);
            }

            countdown -= 1;
        }

        index += 1;
    }
}
