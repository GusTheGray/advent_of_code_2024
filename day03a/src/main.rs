use shared_lib::{print_answer, read_lines};

fn main() {
    let input = read_lines("inputs/day02.txt");

    let answer = find_solution(input);

    print_answer(&answer.to_string());
}

fn find_solution(input: Vec<String>) -> usize {

    0
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