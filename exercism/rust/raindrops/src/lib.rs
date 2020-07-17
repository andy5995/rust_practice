/*    If the number has 3 as a factor, output 'Pling'.
    If the number has 5 as a factor, output 'Plang'.
    If the number has 7 as a factor, output 'Plong'.
    If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through. */

pub fn raindrops(n: u32) -> String {
    let mut owned_string = String::new();
    if n % 3 == 0 {
        owned_string.push_str("Pling");
    }

    if n % 5 == 0 {
        owned_string.push_str("Plang");
    }

    if n % 7 == 0 {
        owned_string.push_str("Plong");
    }

    if owned_string != "" {
        return owned_string;
    }

    return n.to_string();
}

/* Other Solutions */

/*
 * https://exercism.io/tracks/rust/exercises/raindrops/solutions/063bf4216e1c4a748f7a6041d193313a
 *  pub fn raindrops(n: u32) -> String {
    let mut res = String::from("");
    if n % 3 == 0 { res = format!("{}{}",res,"Pling");}
    if n % 5 == 0 { res = format!("{}{}",res,"Plang");}
    if n % 7 == 0 { res = format!("{}{}",res,"Plong");}
    if res.eq("") { res = n.to_string();}
    res
} */

/* https://exercism.io/tracks/rust/exercises/raindrops/solutions/6218001325d246309ca9bda239ae9108
 * pub fn raindrops(x: u32) -> String {
    let is_factor = |factor| x % factor == 0;
    let mut rez = String::new();

    if is_factor(3) { rez.push_str("Pling"); }
    if is_factor(5) { rez.push_str("Plang"); }
    if is_factor(7) { rez.push_str("Plong"); }

    if rez.is_empty() { rez = x.to_string(); }

    rez
}
*/

/* https://exercism.io/tracks/rust/exercises/raindrops/solutions/8cdd283f526244f48764179642acf181
 * pub fn raindrops(num: i64) -> String {
    let outputs: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter(|&&(n, _)| num % n == 0)    // whether n is num's factor
        .map(|&(_, s)| s)                   // we only need the str
        .collect();
    if outputs.len() != 0 {
        outputs
    } else {
        num.to_string()
    }
}
*/

