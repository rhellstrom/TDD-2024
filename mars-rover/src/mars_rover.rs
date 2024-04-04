#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Direction{
    North,
    South,
    East,
    West,
}

#[allow(dead_code)]
pub enum Movement{
    Forward,
    Backward,
    Left,
    Right,
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

#[allow(dead_code)]
impl MarsRover {
    fn new(position: Coordinates, direction: Direction) -> MarsRover {
        MarsRover{
            position,
            direction,
        }
    }
    fn move_rover(&mut self, _movement: Movement){
        //TODO implement
    }
}

#[cfg(test)]
mod tests {
    use crate::mars_rover::Direction::*;
    use crate::mars_rover::Movement::*;
    use super::*;

    #[test]
    fn test_new_rover() {
        let position = Coordinates { x: 3, y: 5 };
        let rover = MarsRover::new(position, East);
        assert_eq!(rover.direction, East);
        assert_eq!(rover.position, position);
    }

    #[test]
    fn test_move_rover_forward() {
        let position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 4, y: 5 };
        let mut rover = MarsRover::new(position, East);
        
        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn test_move_rover_backward() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 3, y: 4 };
        let mut rover = MarsRover::new(start_position, North);

        rover.move_rover(Backward);
        assert_eq!(rover.position, expected_position);
    }
}