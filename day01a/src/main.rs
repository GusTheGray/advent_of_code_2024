use std::usize;

use shared_lib::read_lines;

fn main() {
    let input = read_lines("inputs/day01.txt");

    let answer = find_solution(input);
    println!("Answer is: {}", answer)
    
}


fn find_solution(input: Vec<String>) -> usize {
    let mut left = Vec::new();
    let mut right = Vec::new();


    for l in input{
        let vals: Vec<&str> = l.split(' ').collect();
        println!("vals {:?}", vals);
        left.push(vals[0].parse::<usize>().unwrap());
        right.push(vals[vals.len()-1].parse::<usize>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut total_distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        let distance = if l > r {l - r } else {r-l};
        total_distance += distance
    }

    total_distance
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_solution() {
        let sample_input = vec![
            "3 4".to_string(),
            "4 3".to_string(),
            "2 5".to_string(),
            "1 3".to_string(),
            "3 9".to_string(),
            "3 3".to_string(),
        ];

        let result = find_solution(sample_input);
        assert_eq!(result, 11, "Expected distance is 11");
    }
}