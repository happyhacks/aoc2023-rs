use regex::Regex;
#[derive(Default)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}
fn main() {
    let r = Regex::new(r"(\d+) (.)").unwrap();
    println!(
        "{}",
        include_str!("/tmp/input.txt")
            .lines()
            .map(|l| {
                let mut s = RGB::default();
                r.find_iter(l).for_each(|m| {
                    let x = m.as_str();
                    let n: u32 = x.split_whitespace().nth(0).unwrap().parse().ok().unwrap();
                    match x.chars().last().unwrap() {
                        'r' => s.r = s.r.max(n),
                        'g' => s.g = s.g.max(n),
                        'b' => s.b = s.b.max(n),
                        _ => panic!()
                    }
                });
                s
            })
            .map(|rgb| rgb.r * rgb.g * rgb.b)
            .sum::<u32>(),
    );
}
