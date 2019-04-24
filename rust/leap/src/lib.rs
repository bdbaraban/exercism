pub fn is_leap_year(year: u64) -> bool {
    (year % 100 != 0 || year % 400 == 0) && year % 4 == 0
}
