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
    fn move_rover(&mut self, movement: Movement){
        match movement {
            Movement::Forward => self.move_forward(),
            Movement::Backward => self.move_backward(),
            Movement::Left => {}
            Movement::Right => {}
        }
    }
    
    fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.position.y += 1,
            Direction::South => self.position.y -= 1,
            Direction::East => self.position.x += 1,
            Direction::West => self.position.x -= 1,
        }
    }
    fn move_backward(&mut self) {
        match self.direction {
            Direction::North => self.position.y -= 1,
            Direction::South => self.position.y += 1,
            Direction::East => self.position.x -= 1,
            Direction::West => self.position.x += 1,
        }
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
    fn test_move_rover_forward_from_south() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 3, y: 4 };
        let mut rover = MarsRover::new(start_position, South);

        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn test_move_rover_backward_from_east() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 2, y: 5 };
        let mut rover = MarsRover::new(start_position, East);

        rover.move_rover(Backward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn test_move_rover_forward_from_north() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 3, y: 6 };
        let mut rover = MarsRover::new(start_position, North);

        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn test_move_rover_backward_from_west() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 4, y: 5 };
        let mut rover = MarsRover::new(start_position, West);

        rover.move_rover(Backward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn test_turn_rover_right_from_north() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, North);

        rover.move_rover(Right);
        assert_eq!(rover.direction, East);
    }
    #[test]
    fn test_turn_rover_left_from_north() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, North);

        rover.move_rover(Left);
        assert_eq!(rover.direction, West);
    }
    #[test]
    fn test_turn_rover_right_from_west() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, West);

        rover.move_rover(Right);
        assert_eq!(rover.direction, North);
    }
    #[test]
    fn test_turn_rover_left_from_south() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, South);

        rover.move_rover(Left);
        assert_eq!(rover.direction, East);
    }

}