use std::string::String;

pub fn fizzbuzz(number: &int) -> String{
    if number % 15 == 0 {
        String::from_str("fizzbuzz")
    }
    else if number % 3 == 0 {
        String::from_str("fizz")
    }
    else if number % 5 == 0 {
        String::from_str("buzz")
    }
    else {
        number.to_str()
    }
}
