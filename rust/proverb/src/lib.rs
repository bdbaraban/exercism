pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    let size = list.len();

    if size == 0 {
        return proverb;
    }

    for i in 1..size {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i - 1],
            list[i]
        ));
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
