use regex::Regex;
struct RGB {
    ln: usize,
    r: u8,
    g: u8,
    b: u8,
}
fn main() {
    let r = Regex::new(r"(\d+) (.)").unwrap();
    println!(
        "{}",
        include_str!("/tmp/input.txt")
            .lines()
            .enumerate()
            .map(|(idx, l)| {
                let mut s = RGB {
                    ln: idx + 1,
                    r: 0,
                    g: 0,
                    b: 0,
                };
                r.find_iter(l).for_each(|m| {
                    let x = m.as_str();
                    let n: u8 = x.split_whitespace().nth(0).unwrap().parse().ok().unwrap();
                    match x.chars().last().unwrap() {
                        'r' => s.r = s.r.max(n),
                        'g' => s.g = s.g.max(n),
                        'b' => s.b = s.b.max(n),
                        _ => {}
                    }
                });
                s
            })
            .filter(|rgb| rgb.r <= 12 && rgb.g <= 13 && rgb.b <= 14)
            .map(|rgb| rgb.ln)
            .sum::<usize>(),
    );
}
