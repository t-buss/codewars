fn solution(s: &str) -> String {
    let mut string = String::from(s);
    let mut capital_letters = vec!();
    let mut index = 0;
    for char in s.chars() {
        if char == char.to_uppercase().next().unwrap() {
            capital_letters.push(index + capital_letters.len());
        }
        index += 1;
    }
    for capital_letter in capital_letters {
        string.insert(capital_letter, ' ');
    }
    string
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }

    #[test]
    fn mytest() {
        let mut my_string = String::with_capacity(1);
        my_string.insert(0, 'f');
        my_string.insert(1, 'o');
        my_string.insert(2, 'o');
        println!("{}", my_string);
    }
}
