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
    println!("fib_nohash({}) = {}", n, fib!(n));
    println!("fib_hash({}) = {}", n, fib!(n, &mut fibs));
    println!("fib_iterative({}) = {}", n, fib_iterative(n));
}

// Compute without a hash
fn fib_nohash(n: u32) -> u32 {
    return if n < 2 {
        n
    } else {
        fib_nohash(n - 1) + fib_nohash(n - 2)
    };
}

fn fib_iterative(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut cur: u32 = 1;
    let mut prev: u32 = 1;
    let mut this_fib: u32 = 2;

    while this_fib < n {
        this_fib = this_fib + 1;
        let tmp = cur + prev;
        prev = cur;
        cur = tmp
    }

    return cur;
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
