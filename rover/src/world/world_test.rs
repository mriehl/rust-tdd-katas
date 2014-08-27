use world;
use coords::Position2D;

#[test]
fn x_overflows_height_bounds(){
    let w = world::new(5, 5);

    let actual = w.recalculate_position_overflow(&Position2D{x: 5, y: 6});

    assert_eq!(actual, Some(Position2D{x: 5, y: 0}));
}

#[test]
fn x_overflows_width_bounds(){
    let w = world::new(5, 5);

    let actual = w.recalculate_position_overflow(&Position2D{x: 9, y: 3});

    assert_eq!(actual, Some(Position2D{x: 3, y: 3}));
}

#[test]
fn x_overflows_both_bounds(){
    let w = world::new(5, 5);

    let actual = w.recalculate_position_overflow(&Position2D{x: 6, y: 9});

    assert_eq!(actual, Some(Position2D{x: 0, y: 3}));
}

#[test]
fn prevent_multiple_occupations(){
    let mut w = world::new(5, 5);
    w.insert("obstacle", Position2D{x: 2, y: 3});

    let actual = w.recalculate_position_overflow(&Position2D{x: 2, y: 3});

    assert_eq!(actual, None);
}

#[test]
fn overflow_width_low(){
    let w = world::new(5, 5);

    let actual = w.recalculate_position_overflow(&Position2D{x: -1, y: 0});

    assert_eq!(actual, Some(Position2D{x: 4, y: 0}));
}

#[test]
fn overflow_height_low(){
    let w = world::new(5, 5);

    let actual = w.recalculate_position_overflow(&Position2D{x: 0, y: -1});

    assert_eq!(actual, Some(Position2D{x: 0, y: 4}));
}
