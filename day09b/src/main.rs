fn future(h: Vec<i32>) -> i32 {
    let mut h = h;
    let mut s = 0;
    while !h.iter().all(|&x| x == 0) {
        s += h.iter().last().unwrap();
        h = h.windows(2).map(|x| x[1] - x[0]).collect();
    }
    s
}
fn main() {
    let histories: Vec<Vec<i32>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .rev() // reverse the timeline and the past will be the future
                .collect()
        })
        .collect();
    println!("{}", histories.into_iter().map(|h| future(h)).sum::<i32>());
}
