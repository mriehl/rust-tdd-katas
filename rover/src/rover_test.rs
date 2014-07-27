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

#[test]
fn should_move_by_vector(){
    let mut r = rover::new();
    r.move(rover::Vector(2, 3));
    assert_eq!(r.position, Position2D{x: 2, y: 3})
}

#[test]
fn should_advance_north(){
    let mut r = rover::new();
    r.advance();
    assert_eq!(r.position, Position2D{x: 0, y: 1})
}

#[test]
fn should_advance_east(){
    let mut r = rover::new();
    r.facing = rover::East;
    r.advance();
    assert_eq!(r.position, Position2D{x: 1, y: 0})
}

#[test]
fn should_advance_south(){
    let mut r = rover::new();
    r.facing = rover::South;
    r.advance();
    assert_eq!(r.position, Position2D{x: 0, y: -1})
}

#[test]
fn should_advance_west(){
    let mut r = rover::new();
    r.facing = rover::West;
    r.advance();
    assert_eq!(r.position, Position2D{x: -1, y: 0})
}

#[test]
fn should_not_overflow_default_world_while_moving_north(){
    let mut r = rover::new();
    r.move(rover::XDelta(150));

    assert_eq!(r.position, Position2D{x: 49, y: 0})
}

#[test]
fn should_not_overflow_default_world_while_moving_east(){
    let mut r = rover::new();
    r.move(rover::YDelta(150));

    assert_eq!(r.position, Position2D{x: 0, y: 49})
}

#[test]
fn should_not_overflow_both_coordinates(){
    let mut r = rover::new();
    r.move(rover::Vector(1338, 143));

    assert_eq!(r.position, Position2D{x: 37, y: 42})
}

#[test]
fn should_not_overflow_while_advancing(){
    let mut r = rover::new();
    r.move(rover::YDelta(100));
    assert_eq!(r.position, Position2D{x: 0, y: 100})

    r.advance();

    assert_eq!(r.position, Position2D{x: 0, y: 0})
}

#[test]
fn should_abort_transaction_when_colliding(){
    let mut r = rover::new();
    r.world.insert("moon monster", Position2D{x: 5, y: 5});
    r.move(rover::Vector(5, 5));
    assert_eq!(r.position, Position2D{x: 0, y: 0})
}

#[test]
fn should_abort_last_transaction_when_colliding_in_single_steps(){
    let mut r = rover::new();
    r.world.insert("moon monster", Position2D{x: 5, y: 5});
    r.move(rover::Vector(5, 4));
    r.advance();

    assert_eq!(r.position, Position2D{x: 5, y: 4})

}
