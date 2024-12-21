use shared_lib::read_lines;

fn main() {
    let input = read_lines("inputs/day01.txt");

    let answer = find_solution(input);

    println!("The answer is: {}", answer);
}


fn find_solution(input: Vec<String>) -> usize {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut answer :usize = 0;


    for l in input{
        let vals: Vec<&str> = l.split(' ').collect();
        println!("vals {:?}", vals);
        left.push(vals[0].parse::<usize>().unwrap());
        right.push(vals[vals.len()-1].parse::<usize>().unwrap());
    }

    for x in left.iter() {
        let c = right.iter().filter(|i| *i == x).count();
        answer += x*c;
    }

    answer
    
}
