use regex::Regex;
use shared_lib::{print_answer, read_lines};

fn main() {
    let input = read_lines("inputs/day03.txt");

    let answer = find_solution(input);

    print_answer(&answer.to_string());
}

enum Command {
    Mul(i32, i32)
}

fn find_solution(input: Vec<String>) -> i32 {

    let input = input.join("");

    let command_list = parse_string(&input);

    let mut total = 0;

    for c in command_list {
        match c {
            Command::Mul(a, b) => {
                total += a*b;
            }
            
        }
    }

   total 
}

fn parse_string(s: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    let mut is_processing: bool = true;

    let mut i:usize = 0;
    let s = s.to_string();

    while i < s.len() {
        if s[i..].starts_with("do()") {
            is_processing = true;
            i+=4;
            continue;
        }

        if s[i..].starts_with("don't()") {
            is_processing = false;
            i += 7;
            continue;
        }

        if is_processing && s[i..].starts_with("mul(") {
            let (maybe_cmd, consumed) = parse_first_mul(&s[i..]);
            if let Some(cmd) = maybe_cmd {
                commands.push(cmd);
                i += consumed;
                continue;
            }

        }
        i+=1;
    }

    commands
}


fn parse_first_mul(s: &str) -> (Option<Command>, usize) {
    let re = Regex::new(r"^mul\(\d{1,3},\d{1,3}\)").unwrap();

    if let Some(m) = re.find(s) {
        let matched = m.as_str();
        let inner = &matched[4..matched.len()-1];

        let nums: Vec<i32> = inner.split(',').map(|x| x.parse().unwrap()).collect();

        (Some(Command::Mul(nums[0], nums[1])), m.end()) 
    } else {
        (None, 1)
    }

}


#[cfg(test)]
mod tests {
    use crate::find_solution;

    #[test]
    fn test_find_solution() {
        let test_input = vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()];

        let answer = find_solution(test_input);
        assert_eq!(answer, 48);
    }
}