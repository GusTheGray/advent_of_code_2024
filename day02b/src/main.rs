use shared_lib::{print_answer, read_lines};

fn main() {
    let input = read_lines("inputs/day02.txt");

    let answer = find_solution(input);

    print_answer(&answer.to_string());
}

fn find_solution(input: Vec<String>) -> i32 {
    let mut num_safe: i32 = 0;
    for s in input {
        let numbers: Vec<i32> = s
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut safe = is_safe(&numbers);

        if !safe {
            for i in 0..numbers.len() {
                let mut temp_numbers = numbers.clone();
                temp_numbers.remove(i);
                let temp_safe = is_safe(&temp_numbers);
                if temp_safe {
                    safe = true;
                    break;
                }
            }
        }

        if safe {
            num_safe += 1;
        }
    }
    num_safe
 }

 fn is_safe(numbers: &Vec<i32>) -> bool {
    let mut safe = true;
    if numbers[0] > numbers[1] {
        for i in 1..numbers.len() {
            let diff = numbers[i - 1] - numbers[i];
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }
    } else {
        for i in 1..numbers.len() {
            let diff = numbers[i] - numbers[i - 1];
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }
    }
    safe
 }

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_solution() {
        let test_input = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        let answer = find_solution(test_input);

        assert_eq!(answer, 4);
    }
}
