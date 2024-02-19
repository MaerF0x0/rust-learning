

fn main() {
    let n = 20;
    println!("fib(n) = {}", fib(n))
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return n
    }

    return fib(n-1) + fib(n-2)
}