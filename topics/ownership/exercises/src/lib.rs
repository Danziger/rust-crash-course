pub fn exercise_1() {
    let s = "rust".to_string();
    let s1 = s;
    // let s2 = s;
    println!("{s1}");
}

pub fn exercise_2() {
    let s = "rust".to_string();
    {
        let s1 = s;
        
        // Can also be fixed doing this:
        // let s1 = s.clone();

        println!("{s1}");
    }

    // println!("{s}");
}

// Can also be fixed doing this:
// fn take(s: &String) {
fn take(s: String) {
    println!("take {s}");
}

pub fn exercise_3() {
    let s = "rust".to_string();

    // Can also be fixed doing this:
    // take(&s);

    // take(s);

    println!("{s}");
    println!("{s}");
}
