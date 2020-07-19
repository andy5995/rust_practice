/* Vector practice and examples */

// https://doc.rust-lang.org/book/ch08-01-vectors.html

// let v: Vec<i32> = Vec::new();

// let v = vec![1, 2, 3];

/* let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8); */

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /* If we want to access each element in a vector in turn, we can iterate through all of the elements rather than use indices to access one at a time. Listing 8-8 shows how to use a for loop to get immutable references to each element in a vector of i32 values and print them. */
    for i in &v {
        println!("{}", i);
    }

    /* We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The for loop in Listing 8-9 will add 50 to each element. */
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    /* Using an Enum to Store Multiple Types */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
