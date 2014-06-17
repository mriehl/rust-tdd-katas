mod fizzlib;

#[test]
fn fizzbuzz_test_1() {
    assert!(::fizzlib::fizzbuzz(1) == ~"1");
}

#[test]
fn fizzbuzz_test_3() {
    assert!(::fizzlib::fizzbuzz(3) == ~"fizz");
}

#[test]
fn fizzbuzz_test_5() {
    assert!(::fizzlib::fizzbuzz(5) == ~"buzz");
}

#[test]
fn fizzbuzz_test_7() {
    assert!(::fizzlib::fizzbuzz(7) == ~"7");
}

#[test]
fn fizzbuzz_test_10() {
    assert!(::fizzlib::fizzbuzz(10) == ~"buzz");
}


#[test]
fn fizzbuzz_test_15() {
    assert!(::fizzlib::fizzbuzz(15) == ~"fizzbuzz");
}
