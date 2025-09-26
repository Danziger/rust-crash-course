#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    return if y == 0 {
        Err(MathError::DivByZero)
    } else {
        Ok(x/y)
    }
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    /*
    return match v.get(i) {
        Some(val) => *val,
        None => default_val,
    };
    */

    // if let alternative:
    return if let Some(val) = v.get(i) {
        *val
    } else {
        default_val
    };
}
