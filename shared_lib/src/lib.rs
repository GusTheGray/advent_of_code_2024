use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Could not find file")
        .lines()
        .map(String::from)
        .collect()
}

pub fn print_answer(answer: &str) {
    println!("The answer is: {}", answer);
}

#[cfg(test)]
mod tests {}
