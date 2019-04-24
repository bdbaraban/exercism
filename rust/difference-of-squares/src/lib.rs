pub fn square_of_sum(n: u32) -> u32 {
    let mut sq_sum = 0;

    for num in 1..=n {
        sq_sum += num;
    }

    sq_sum * sq_sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum_sq = 0;

    for num in 1..=n {
        sum_sq += num * num;
    }

    sum_sq
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
