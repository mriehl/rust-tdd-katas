use rover;
use coords::Position2D;

#[test]
fn should_create_new_rover(){
    let r = rover::new();
    assert_eq!(r.position, Position2D{x: 0, y: 0})
}
