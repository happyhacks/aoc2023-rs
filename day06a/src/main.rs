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
    let mut n = 1;
    for i in 0..times.len() {
        let mut c = 0;
        for x in 0..times[i] {
            println!("{} : {}", x, x * (times[i] - x));
            if x * (times[i] - x) > distances[i] {
                c += 1;
            }
        }
        println!("{}", c);
        n *= c;
    }
    println!("{}", n);
}
