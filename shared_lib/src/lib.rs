use std::fs::read_to_string;


pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
    .expect("Could not find file").lines().map(String::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

}
