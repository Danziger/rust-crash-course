use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut debts: HashMap<String, u32> = HashMap::new();
    
    debts.insert(address, amount);

    return debts;
}
