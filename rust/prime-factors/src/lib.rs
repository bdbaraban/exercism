pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut n = n;

    while n % 2 == 0 {
        primes.push(2);
        n /= 2;
    }

    let mut div: u64 = 3;
    let root: u64 = (n as f64).sqrt() as u64;
    while div <= root {
        while n % div == 0 {
            primes.push(div);
            n /= div;
        }
        div += 2;
    }

    if n > 2 {
        primes.push(n);
    }

    primes
}
