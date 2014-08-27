use rover;
use coords::Position2D;

describe!(
    it "should create new rover" {
        let r = rover::new();
        assert_eq!(r.position, Position2D{x: 0, y: 0})
    }

    it "should move north" {
        let mut r = rover::new();
        r.move(rover::YDelta(1));
        assert_eq!(r.position, Position2D{x: 0, y: 1})
    }

    it "should move north twice" {
        let mut r = rover::new();
        r.move(rover::YDelta(2));
        assert_eq!(r.position, Position2D{x: 0, y: 2})
    }

    it "should move east" {
        let mut r = rover::new();
        r.move(rover::XDelta(1));
        assert_eq!(r.position, Position2D{x: 1, y: 0})
    }

    it "should move east twice" {
        let mut r = rover::new();
        r.move(rover::XDelta(2));
        assert_eq!(r.position, Position2D{x: 2, y: 0})
    }

    it "should move south" {
        let mut r = rover::new();
        r.move(rover::YDelta(-1));
        assert_eq!(r.position, Position2D{x: 0, y: 99})
    }

    it "should move south twice" {
        let mut r = rover::new();
        r.move(rover::YDelta(-2));
        assert_eq!(r.position, Position2D{x: 0, y: 98})
    }

    it "should move west" {
        let mut r = rover::new();
        r.move(rover::XDelta(-1));
        assert_eq!(r.position, Position2D{x: 99, y: 0})
    }

    it "should move west twice" {
        let mut r = rover::new();
        r.move(rover::XDelta(-2));
        assert_eq!(r.position, Position2D{x: 98, y: 0})
    }

    it "should move by vector" {
        let mut r = rover::new();
        r.move(rover::Vector(2, 3));
        assert_eq!(r.position, Position2D{x: 2, y: 3})
    }

    it "should advance north" {
        let mut r = rover::new();
        r.advance();
        assert_eq!(r.position, Position2D{x: 0, y: 1})
    }

    it "should advance east" {
        let mut r = rover::new();
        r.facing = rover::East;
        r.advance();
        assert_eq!(r.position, Position2D{x: 1, y: 0})
    }

    it "should advance south" {
        let mut r = rover::new();
        r.facing = rover::South;
        r.advance();
        assert_eq!(r.position, Position2D{x: 0, y: 99})
    }

    it "should advance west" {
        let mut r = rover::new();
        r.facing = rover::West;
        r.advance();
        assert_eq!(r.position, Position2D{x: 99, y: 0})
    }

    it "should not overflow default world while moving east" {
        let mut r = rover::new();
        r.move(rover::XDelta(150));

        assert_eq!(r.position, Position2D{x: 49, y: 0})
    }

    it "should not overflow default world while moving north" {
        let mut r = rover::new();
        r.move(rover::YDelta(150));

        assert_eq!(r.position, Position2D{x: 0, y: 49})
    }

    it "should not overflow default world while moving west" {
        let mut r = rover::new();
        r.facing = rover::West;
        r.advance();
        r.advance();

        assert_eq!(r.position, Position2D{x: 98, y: 0})
    }

    it "should not overflow both coordinates" {
        let mut r = rover::new();
        r.move(rover::Vector(1338, 143));

        assert_eq!(r.position, Position2D{x: 37, y: 42})
    }

    it "should not overflow while advancing" {
        let mut r = rover::new();
        r.move(rover::YDelta(100));
        assert_eq!(r.position, Position2D{x: 0, y: 100})

        r.advance();

        assert_eq!(r.position, Position2D{x: 0, y: 0})
    }

    it "should abort transaction when colliding" {
        let mut r = rover::new();
        r.world.insert("moon monster", Position2D{x: 5, y: 5});
        r.move(rover::Vector(5, 5));
        assert_eq!(r.position, Position2D{x: 0, y: 0})
    }

    it "should abort last transaction when colliding in single steps" {
        let mut r = rover::new();
        r.world.insert("moon monster", Position2D{x: 5, y: 5});
        r.move(rover::Vector(5, 4));
        r.advance();

        assert_eq!(r.position, Position2D{x: 5, y: 4})
    }
  )
