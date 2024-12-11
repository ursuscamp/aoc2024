use crate::utils::input;

pub fn run(example: bool) -> anyhow::Result<()> {
    println!("Day 11");
    let data = input(11, example)?;
    p1(&data);
    p2(&data);

    Ok(())
}

fn p1(input: &str) {
    let nums = parse(input);
    let result = process_times(nums, 25).len();
    println!("P1: {result:?}");
}

fn p2(input: &str) {}

fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().map(ToString::to_string).collect()
}

fn process_times(nums: Vec<String>, times: usize) -> Vec<String> {
    if times == 0 {
        return nums;
    }

    let mut nums = nums;
    for _ in 0..times {
        nums = process(nums).collect();
    }

    nums
}

fn process(nums: Vec<String>) -> impl Iterator<Item = String> {
    nums.into_iter().flat_map(process_num)
}

fn process_num(num: String) -> impl Iterator<Item = String> {
    match num.as_str() {
        "" | "0" => [None, Some(String::from("1"))],
        _ if num.len() % 2 == 0 => split_even(num),
        _ => {
            let n = num.parse::<u64>().unwrap() * 2024u64;
            [None, Some(n.to_string())]
        }
    }
    .into_iter()
    .flatten()
}

fn split_even(num: String) -> [Option<String>; 2] {
    let len = num.len() / 2;
    let l = num[..len].trim_start_matches('0').to_string();
    let r = num[len..].trim_start_matches('0').to_string();
    [Some(l), Some(r)]
}
