
pub fn fibonacci(n: u64, mut f0: u64, mut f1: u64) {
    for _ in 1..n {
        let f2: u64 = f0 + f1;
        println!("{}", f2);
        f0 = f1;
        f1 = f2;
    }
}