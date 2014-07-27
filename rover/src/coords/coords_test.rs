use coords::Position2D;

#[test]
fn test_position_symmetric_equality(){
    let c1 = Position2D{x: 4, y: 5};
    let c2 = Position2D{x: 4, y: 5};

    assert_eq!(c1 == c2, true);
    assert_eq!(c2 == c1, true);
}

#[test]
fn test_position_equality_reflexivity(){
    let c = Position2D{x: 4, y: 5};

    assert_eq!(c == c, true);
}

#[test]
fn test_position_unequality(){
    let c = Position2D{x: 4, y: 5};
    let other = Position2D{x: 4, y: 42};

    assert_eq!(c == other, false);
}
