pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let a: u32 = a.parse().expect("Failed to parse variable");
    let b: u32 = b.parse().expect("Failed to parse variable");

    return a + b;
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    let x = x.unwrap();
    let y = y.unwrap();

    return x + y;
}
