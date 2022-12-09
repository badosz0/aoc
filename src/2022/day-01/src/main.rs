const INPUT: &str = include_str!("../input.txt");

fn main() {
  let part1 = INPUT
    .trim()
    .split("\n\n")
    .map(|elf| {
      elf
        .split("\n")
        .map(|cal| {
          cal
            .trim()
            .parse()
            .unwrap_or(0)
        })
        .sum()
    })
    .max()
    .unwrap_or(0);

  let mut elfs: Vec<i32> = INPUT
    .trim()
    .split("\n\n")
    .map(|elf| {
      elf
        .split("\n")
        .map(|cal| {
          cal
            .trim()
            .parse()
            .unwrap_or(0)
        })
        .sum()
    })
    .collect();

    elfs.sort_by(|a, z| z.cmp(a));

    let part2: i32 = elfs[0..=2]
      .iter()
      .sum();


  println!("Part 1: {part1}, Part 2: {part2}")
}
