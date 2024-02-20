use std::collections::HashMap;

macro_rules! fib {
    ($x: expr) => {
        fib_nohash($x)
    }; //call function f1 when there's one variable
    ($x: expr, $y: expr) => {
        fib_hash($x, $y)
    }; //call f2 when there are two
}

fn main() {
    let mut fibs = HashMap::<u32, u32>::new();

    let n = 20;
    println!("fib({}) = {}", n, fib!(n));
    println!("fib({}) = {}", n, fib!(n, &mut fibs))
}

// Compute without a hash
fn fib_nohash(n: u32) -> u32 {
    return if n <= 2 {
        n
    } else {
        fib_nohash(n - 1) + fib_nohash(n - 2)
    };
}

// Compute using a hash to memoize
fn fib_hash(n: u32, fibs: &mut HashMap<u32, u32>) -> u32 {
    match fibs.get(&n) {
        Some(n) => return *n,
        None => {
            if n <= 2 {
                let this_fib = fib_nohash(n);
                fibs.insert(n, this_fib);
                return this_fib;
            }

            let this_fib = fib_hash(n - 1, fibs) + fib_hash(n - 2, fibs);
            fibs.insert(n, this_fib);
            return this_fib;
        }
    }
}
