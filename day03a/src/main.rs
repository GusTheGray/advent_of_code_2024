use regex::Regex;
use shared_lib::{print_answer, read_lines};

fn main() {
    let input = read_lines("inputs/day03.txt");

    let answer = find_solution(input);

    print_answer(&answer.to_string());
}

fn find_solution(input: Vec<String>) -> usize {

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut matches: Vec<String> = Vec::new();

    for s in input {
        let m: Vec<String> = re.find_iter(&s).map(|m| m.as_str().to_string()).collect();
        matches.extend(m);
    }
    let mut total =0;

    for s in matches {
        let s = s[4..].to_string();
        let s = s[..s.len()-1].to_string();

        let nums: Vec<usize> = s.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        total += nums[0] * nums[1];
        println!("{}", s);
    }



    total
}



#[cfg(test)]
mod tests {
    use crate::find_solution;

    #[test]
    fn test_find_solution() {
        let test_input = vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()];

        let answer = find_solution(test_input);
        assert_eq!(answer, 161);
    }
}