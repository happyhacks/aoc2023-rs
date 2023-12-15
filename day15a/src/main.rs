fn main() {
    println!(
        "{}",
        include_str!("/tmp/input.txt")
            .trim_end()
            .split(",")
            .map(|l| { l.chars().fold(0, |h, c| { ((h + c as usize) * 17) % 256 }) })
            .sum::<usize>()
    )
}
