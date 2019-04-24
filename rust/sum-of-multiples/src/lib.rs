pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;

    for num in 1..limit {
        for f in factors {
            if *f as i32 != 0 && num % f == 0 {
                sum += num;
                break;
            }
        }
    }

    sum
}
