pub fn add(x: u32, y: u32) -> u32 {
    return x + y;
}

pub fn mul(x: u32, y: u32) -> u32 {
    return x * y;
}

pub fn div(x: u32, y: u32) -> u32 {
    return x / y;
}

pub fn divWithRemainder(x: u32, y: u32) -> (u32, u32) {
    return (x / y, x % y);
}

pub fn fnWithNoReturn(s: String) {
    println!("{s}{s}{s}{s}{s}");
}
