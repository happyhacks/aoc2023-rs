fn main() {
    let input: Vec<usize> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect();
    let time = input[0].clone();
    let distance = input[1].clone();
    let mut c = 0;
    for x in 0..time {
        if x * (time - x) > distance {
            c += 1;
        }
    }
    println!("{}", c);
}
