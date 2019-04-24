pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);
    primes.push(3);
    primes.push(5);
    primes.push(7);
    primes.push(11);

    if n < 5 {
        return primes[n as usize];
    }

    let mut p = 13;
    let mut count = 4;
    while count < n {
        let mut is_prime = true;
        for div in &primes {
            if p % div == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(p);
            count += 1;
        }
        p += 2;
    }
    primes[count as usize]
}
