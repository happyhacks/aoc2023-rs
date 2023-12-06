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
    let times = vec![input[0].clone()];
    let distances = vec![input[1].clone()];
    println!("{}", times.iter().zip(distances).fold(1, |n, (&time, distance)| {
        n * (0..time).filter(|x| x*(time-x) > distance).count()
    }));
}
