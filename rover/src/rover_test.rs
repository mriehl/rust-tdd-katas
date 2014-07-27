use rover;
use coords::Position2D;

#[test]
fn should_create_new_rover(){
    let r = rover::new();
    assert_eq!(r.position, Position2D{x: 0, y: 0})
}

#[test]
fn should_move_north(){
    let mut r = rover::new();
    r.move(rover::YDelta(1));
    assert_eq!(r.position, Position2D{x: 0, y: 1})
}

#[test]
fn should_move_north_twice(){
    let mut r = rover::new();
    r.move(rover::YDelta(2));
    assert_eq!(r.position, Position2D{x: 0, y: 2})
}


#[test]
fn should_move_east(){
    let mut r = rover::new();
    r.move(rover::XDelta(1));
    assert_eq!(r.position, Position2D{x: 1, y: 0})
}

#[test]
fn should_move_east_twice(){
    let mut r = rover::new();
    r.move(rover::XDelta(2));
    assert_eq!(r.position, Position2D{x: 2, y: 0})
}

#[test]
fn should_move_south(){
    let mut r = rover::new();
    r.move(rover::YDelta(-1));
    assert_eq!(r.position, Position2D{x: 0, y: -1})
}

#[test]
fn should_move_south_twice(){
    let mut r = rover::new();
    r.move(rover::YDelta(-2));
    assert_eq!(r.position, Position2D{x: 0, y: -2})
}


#[test]
fn should_move_west(){
    let mut r = rover::new();
    r.move(rover::XDelta(-1));
    assert_eq!(r.position, Position2D{x: -1, y: 0})
}

#[test]
fn should_move_west_twice(){
    let mut r = rover::new();
    r.move(rover::XDelta(-2));
    assert_eq!(r.position, Position2D{x: -2, y: 0})
}
