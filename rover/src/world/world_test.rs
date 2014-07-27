use world::World;
use coords::Position2D;

#[test]
fn x_overflows_height_bounds(){
    let w = World{width: 5, height: 5};

    let actual = w.recalculate_position_overflow(&Position2D{x: 5, y: 6});

    assert_eq!(actual, Position2D{x: 5, y: 1});
}

#[test]
fn x_overflows_width_bounds(){
    let w = World{width: 5, height: 5};

    let actual = w.recalculate_position_overflow(&Position2D{x: 9, y: 3});

    assert_eq!(actual, Position2D{x: 4, y: 3});
}

#[test]
fn x_overflows_both_bounds(){
    let w = World{width: 5, height: 5};

    let actual = w.recalculate_position_overflow(&Position2D{x: 6, y: 9});

    assert_eq!(actual, Position2D{x: 1, y: 4});
}
