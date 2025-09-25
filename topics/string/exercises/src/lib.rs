pub fn hello() -> String {
    return "Hello Rust".to_string();
}

pub fn greet(name: &str) -> String {
    return format!("Hello {}", name);
}

pub fn append(mut s: String) -> String {
    s = s + "!";
    
    return s;
}
