
//calculator

//function for addition
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mult(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    println!("cool calc");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_addition() {
        assert_eq!(addition(4, 5), 9);
    }
    #[test]
    fn test_sub() {
        assert_eq!(sub(4, 5), -1);
    }
    #[test]
    fn test_mult() {
        assert_eq!(mult(4, 5), 20);
    }
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 5.0), 2.0);
    }
}
