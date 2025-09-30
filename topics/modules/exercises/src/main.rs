use my_modules::{math, util};

fn main() {
    util::log::debug(&format!("min: {}", math::min(1, 2)));        
    util::log::debug(&format!("max: {}", math::max(1, 2)));

    let v: Vec<u32> = vec![1, 2, 3];

    // With the ? operator:
    util::log::debug(&format!("first: {:?}", util::vec::first(&v)));
    util::log::debug(&format!("last: {:?}", util::vec::last(&v)));    

    // Same with unwrap:
    // util::log::debug(&format!("first: {}", util::vec::first(&v).unwrap()));
    // util::log::debug(&format!("last: {}", util::vec::last(&v).unwrap()));   
}
