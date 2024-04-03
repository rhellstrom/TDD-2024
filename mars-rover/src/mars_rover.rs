
#[derive(Debug, PartialEq)]
pub enum Direction{
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coordinates{
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct MarsRover {
    position: Coordinates,
    direction: Direction,
}

impl MarsRover {
    fn new(position: Coordinates, direction: Direction) -> MarsRover {
        MarsRover{
            position,
            direction,
        }
    }
    fn move_rover(&mut self){
        //TODO implement
    }
}

#[cfg(test)]
mod tests {
    use crate::mars_rover::Direction::*;
    use super::*;

    #[test]
    fn test_new_rover() {
        let position = Coordinates { x: 3, y: 5 };
        let rover = MarsRover::new(position, East);
        assert_eq!(rover.direction, East);
        assert_eq!(rover.position, position);
    }
}