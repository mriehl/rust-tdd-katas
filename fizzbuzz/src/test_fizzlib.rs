use std::string::String;

#[test]
fn fizzbuzz_test_1() {
    assert!(::fizzlib::fizzbuzz(&1) == String::from_str("1"));
}

#[test]
fn fizzbuzz_test_3() {
    assert!(::fizzlib::fizzbuzz(&3) == String::from_str("fizz"));
}

#[test]
fn fizzbuzz_test_5() {
    assert!(::fizzlib::fizzbuzz(&5) == String::from_str("buzz"));
}

#[test]
fn fizzbuzz_test_7() {
    assert!(::fizzlib::fizzbuzz(&7) == String::from_str("7"));
}

#[test]
fn fizzbuzz_test_10() {
    assert!(::fizzlib::fizzbuzz(&10) == String::from_str("buzz"));
}


#[test]
fn fizzbuzz_test_15() {
    assert!(::fizzlib::fizzbuzz(&15) == String::from_str("fizzbuzz"));
}
