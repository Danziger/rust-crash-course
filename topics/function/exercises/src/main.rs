use function::add;
use function::mul;
use function::div;
use function::divWithRemainder;
use function::fnWithNoReturn;

fn main() {
    println!("{}", add(1, 2));
    println!("{}", mul(10, 20));
    println!("{}", div(20, 2));
    println!("{}, {}", divWithRemainder(20, 3).0, divWithRemainder(20, 3).1);

    fnWithNoReturn("🐸".to_string())
}