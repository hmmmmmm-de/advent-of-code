fn main() {
    init()
}

fn part1(input: &str) -> String {
    // TODO-nfgk93 implement logic for the task here
    return "MyDemoResult".to_string();
}

fn init() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let output = part1(input);

        assert_eq!(output, "MyDemoResult".to_string());
    }
}
