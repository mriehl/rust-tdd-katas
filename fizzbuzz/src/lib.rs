mod fizzbuzz  {

pub fn fizzbuzz(number: int) -> ~str {
    if number % 15 == 0 {
        ~"fizzbuzz"
    }
    else if number % 3 == 0 {
        ~"fizz"
    }
    else if number % 5 == 0 {
        ~"buzz"
    }
    else {
        number.to_str()
    }

}

#[test]
fn fizzbuzz_test_1() {
    assert!(fizzbuzz(1) == ~"1");
}

#[test]
fn fizzbuzz_test_3() {
    assert!(fizzbuzz(3) == ~"fizz");
}

#[test]
fn fizzbuzz_test_5() {
    assert!(fizzbuzz(5) == ~"buzz");
}

#[test]
fn fizzbuzz_test_7() {
    assert!(fizzbuzz(7) == ~"7");
}

#[test]
fn fizzbuzz_test_10() {
    assert!(fizzbuzz(10) == ~"buzz");
}


#[test]
fn fizzbuzz_test_15() {
    assert!(fizzbuzz(15) == ~"fizzbuzz");
}

}
