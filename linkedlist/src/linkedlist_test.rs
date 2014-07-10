use linkedlist::Nil;
use linkedlist::Cons;
use linkedlist::repr;
use linkedlist::equal;
use linkedlist::visual_compare;


#[test]
fn test_should_render_empty_list() {
    let list = Nil;

    assert_eq!(repr(&list).as_slice(), "<[]>");
}

#[test]
fn test_should_render_list_with_one_element() {
    let mut list = Nil;
    list = Cons(42, box list);

    assert_eq!(repr(&list).as_slice(), "<[42]>");
}

#[test]
fn test_should_render_list_with_several_elements() {
    let mut list = Nil;
    list = Cons(42, box list);
    list = Cons(23, box list);
    list = Cons(31416, box list);

    assert_eq!(repr(&list).as_slice(), "<[31416, 23, 42]>");
}

#[test]
fn test_two_empty_lists_should_be_equal() {
    let nil1 = Nil;
    let nil2 = Nil;

    assert_eq!(true, equal(&nil1, &nil2));
}

#[test]
fn test_should_render_comparison_of_equal_lists() {
    let nil1 = Nil;
    let nil2 = Nil;

    assert_eq!(String::from_str("<[]> and <[]> are equal"), visual_compare(&nil1, &nil2));
}


#[test]
fn test_list_should_not_equal_empty_list() {
    let mut list = Nil;
    list = Cons(42, box list);
    let nil = Nil;

    assert_eq!(false, equal(&list, &nil));
}

#[test]
fn test_should_render_comparison_of_unequal_lists() {
    let mut list = Nil;
    list = Cons(42, box list);
    let nil = Nil;

    assert_eq!(String::from_str("<[42]> and <[]> are not equal"), visual_compare(&list, &nil));
}

#[test]
fn test_should_equal_other_list_when_single_element_matches() {
    let mut list1 = Nil;
    list1 = Cons(42, box list1);
    let mut list2 = Nil;
    list2 = Cons(42, box list2);

    assert_eq!(true, equal(&list1, &list2));
}

#[test]
fn test_should_not_equal_other_list_when_multiple_elements_mismatch() {
    let mut list1 = Nil;
    list1 = Cons(1, box list1);
    list1 = Cons(3, box list1);
    list1 = Cons(5, box list1);

    let mut list2 = Nil;
    list2 = Cons(2, box list2);
    list2 = Cons(4, box list2);
    list2 = Cons(6, box list2);

    assert_eq!(false, equal(&list1, &list2));
}
