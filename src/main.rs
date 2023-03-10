use std::io;

fn main() {
    println!("Computes Nth Fibonacci number. N=?");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    let n: u128 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => return,
    };

    println!("Fibonacci({}) = {}", n, fib(n));
}

fn fib(n: u128) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 0;
    for _ in 0..n {
        let tmp = b;
        b = a + b;
        a = tmp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(11), 89);
        assert_eq!(fib(12), 144);
        assert_eq!(fib(13), 233);
        assert_eq!(fib(14), 377);
        assert_eq!(fib(15), 610);
        assert_eq!(fib(16), 987);
        assert_eq!(fib(17), 1597);
        assert_eq!(fib(18), 2584);
        assert_eq!(fib(19), 4181);
        assert_eq!(fib(20), 6765);
        assert_eq!(fib(21), 10946);
        assert_eq!(fib(22), 17711);
        assert_eq!(fib(23), 28657);
    }
}
