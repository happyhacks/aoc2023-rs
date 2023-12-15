fn main() {
    let mut lenses: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
    include_str!("/tmp/input.txt")
        .trim_end()
        .split(",")
        .for_each(|l| {
            let parts = l.split_once(|c| c == '=' || c == '-');
            match parts {
                Some((label, "")) => {
                    let h = label.chars().fold(0, |h, c| ((h + c as usize) * 17) % 256);
                    lenses[h].retain(|l| l.0 != label)
                },
                Some((label, lens)) => {
                    let h = label.chars().fold(0, |h, c| ((h + c as usize) * 17) % 256);
                    let lens = lens.parse().unwrap();
                    lenses[h]
                        .iter_mut()
                        .find(|l| l.0 == label)
                        .map(|l| l.1 = lens)
                        .or_else(|| {
                            lenses[h].push((label.to_owned(), lens));
                            Some(())
                        });
                }
                _ => unreachable!(),
            }
        });
    println!(
        "{}",
        lenses
            .iter()
            .enumerate()
            .map(|(bidx, b)| b
                .iter()
                .enumerate()
                .map(|(lidx, (_, l))| l * (1 + bidx as usize) * (1 + lidx as usize))
                .sum::<usize>())
            .sum::<usize>()
    );
}
