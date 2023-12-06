fn main() {
    let input: Vec<Vec<usize>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| {
            l.split(':')
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let times = input[0].clone();
    let distances = input[1].clone();
    println!("{}", times.iter().zip(distances).fold(1, |n, (&time, distance)| {
        n * (0..time).filter(|x| x*(time-x) > distance).count()
    }));
}
