fn main() {
    let days = 12;

    let gift = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-layin'",
        "swans a-swimmin'",
        "maids a-milkin'",
        "lords a-leapin'",
        "ladies dancin'",
        "pipers pipin'",
        "drummers drummin'",
    ];

    let which = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 0..days {
        println!(
            "On the {} day of Christmas my true love gave to me",
            which[day]
        );

        if day > 0 {
            println!("{} {}", day + 1, gift[day]);
            for rev_cumulative_day in (1..day).rev() {
                println!("{} {}", rev_cumulative_day + 1, gift[rev_cumulative_day]);
            }
            print!("and ");
        };
        println!("a {}", gift[0]);
        println!();
    }
}
