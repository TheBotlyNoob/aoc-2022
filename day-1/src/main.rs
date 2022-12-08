use std::sync::atomic::{AtomicU16, Ordering::SeqCst};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let elf = elf_with_most_cal(&input).unwrap();
    println!("Elf {} has the most calories ({})", elf.id, elf.calories);

    let top_three = top_three(&input);
    println!("Top three elves:");
    for elf in top_three.iter() {
        println!("Elf {} has {} calories", elf.id, elf.calories);
    }
    println!(
        "The top-three elves' calories in total is {}",
        top_three.iter().map(|elf| elf.calories).sum::<u32>()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    id: u16,
    calories: u32,
}

static ELF: AtomicU16 = AtomicU16::new(0);
impl Elf {
    pub fn new() -> Self {
        Self {
            id: ELF.fetch_add(1, SeqCst),
            calories: 0,
        }
    }
    pub fn with_calories(cal: u32) -> Self {
        Self {
            id: ELF.fetch_add(1, SeqCst),
            calories: cal,
        }
    }
    pub fn parse_elf(input: &str) -> Self {
        let mut elf = Self::new();
        for line in input.lines() {
            let calories: u32 = line.parse().unwrap();
            elf.calories += calories;
        }
        elf
    }
}
impl Default for Elf {
    fn default() -> Self {
        Self::new()
    }
}

pub fn elf_with_most_cal(input: &str) -> Option<Elf> {
    let elves = input.split("\n\n").map(Elf::parse_elf);
    elves.max_by_key(|elf| elf.calories)
}
pub fn top_three(input: &str) -> [Elf; 3] {
    let mut elves = input.split("\n\n").map(Elf::parse_elf).collect::<Vec<_>>();
    elves.sort_by_key(|elf| elf.calories);
    elves.reverse();
    elves.truncate(3);
    let mut top_three = [Elf::default(); 3];
    top_three.copy_from_slice(&elves);
    top_three
}

#[test]
fn day_1() {
    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    let elf = elf_with_most_cal(EXAMPLE_INPUT).unwrap();
    dbg!(&elf);
    assert_eq!(elf.id, 3);
    assert_eq!(elf.calories, 24000);

    let top_three = top_three(EXAMPLE_INPUT);
    dbg!(&top_three);
    assert_eq!(top_three[0].id, 3);
    assert_eq!(top_three[0].calories, 24000);
    assert_eq!(top_three[1].id, 2);
    assert_eq!(top_three[1].calories, 21000);
    assert_eq!(top_three[2].id, 1);
    assert_eq!(top_three[2].calories, 18000);

    assert_eq!(top_three.iter().map(|elf| elf.calories).sum::<u32>(), 63000);
}
