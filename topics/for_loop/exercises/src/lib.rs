pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for x in nums.iter() {
        sum += x;
    }

    return sum;
}

pub fn fill(value: u32, n: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    for _ in 0..n {
        v.push(value);
    }

    return v;
}
