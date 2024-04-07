use crate::mars_rover::Movement::*;
use crate::mars_rover::Direction::*;

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
    grid_size: i32,
}

#[allow(dead_code)]
impl MarsRover {
    fn new(position: Coordinates, direction: Direction, grid_size: i32) -> MarsRover {
        MarsRover{
            position,
            direction,
            grid_size,
        }
    }

    fn command_input(&mut self, commands: &str) {
        for command in commands.chars() {
            match command {
                'F' => self.move_rover(Forward),
                'B' => self.move_rover(Backward),
                'L' => self.move_rover(Left),
                'R' => self.move_rover(Right),
                _ => println!("Unrecognized command"),
            }
        }
    }

    fn move_rover(&mut self, movement: Movement){
        match movement {
            Forward => self.move_forward(),
            Backward => self.move_backward(),
            Left => self.turn_left(),
            Right => self.turn_right(),
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            North => {
                // If y is at 1 we'd have to adjust x to mimic crossing the northern pole
                if self.position.y == 1 {
                    self.position.x = (self.position.x + self.grid_size / 2) % self.grid_size + 1
                } else {
                    self.position.y += 1
                }
            },
            South => {
                //If we're at the southern pole
                if self.position.y == self.grid_size {
                    self.position.x = (self.position.x + self.grid_size / 2) % self.grid_size
                } else {
                    self.position.y -= 1
                }
            }

            East => self.position.x = if self.position.x == self.grid_size { 1 } else { self.position.x + 1 },
            West => self.position.x = if self.position.x == 1 { self.grid_size } else { self.position.x - 1 },
        }
    }

    fn move_backward(&mut self) {
        match self.direction {
            North => self.position.y = if self.position.y == 1 { self.grid_size } else { self.position.y - 1 },
            South => self.position.y = if self.position.y == self.grid_size { 1 } else { self.position.y + 1 },
            East => self.position.x = if self.position.x == 1 { self.grid_size } else { self.position.x - 1 },
            West => self.position.x = if self.position.x == self.grid_size { 1 } else { self.position.x + 1 },
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            North => West,
            West => South,
            South => East,
            East => North,
        };
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::mars_rover::Movement::*;
    use super::*;

    #[test]
    fn new_rover() {
        let position = Coordinates { x: 3, y: 5 };
        let rover = MarsRover::new(position, East, 6);
        assert_eq!(rover.direction, East);
        assert_eq!(rover.position, position);
    }
    #[test]
    fn move_rover_forward_from_south() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 3, y: 4 };
        let mut rover = MarsRover::new(start_position, South, 6);

        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn move_rover_backward_from_east() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 2, y: 5 };
        let mut rover = MarsRover::new(start_position, East, 6);

        rover.move_rover(Backward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn move_rover_forward_from_north() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 3, y: 6 };
        let mut rover = MarsRover::new(start_position, North, 6);

        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn move_rover_backward_from_west() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates { x: 4, y: 5 };
        let mut rover = MarsRover::new(start_position, West, 6);

        rover.move_rover(Backward);
        assert_eq!(rover.position, expected_position);
    }
    #[test]
    fn turn_rover_right_from_north() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, North, 6);

        rover.move_rover(Right);
        assert_eq!(rover.direction, East);
    }
    #[test]
    fn turn_rover_left_from_north() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, North, 6);

        rover.move_rover(Left);
        assert_eq!(rover.direction, West);
    }
    #[test]
    fn turn_rover_right_from_west() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, West, 6);

        rover.move_rover(Right);
        assert_eq!(rover.direction, North);
    }
    #[test]
    fn turn_rover_left_from_south() {
        let start_position = Coordinates { x: 3, y: 5 };
        let mut rover = MarsRover::new(start_position, South, 6);

        rover.move_rover(Left);
        assert_eq!(rover.direction, East);
    }
    #[test]
    fn several_movement_commands_one() {
        let start_position = Coordinates { x: 3, y: 5 };
        let expected_position = Coordinates {x: 4, y: 6 };
        let mut rover = MarsRover::new(start_position, East, 6);
        let commands_input = "FFRBLB";
        rover.command_input(commands_input);
        assert_eq!(rover.position, expected_position);
    }

    #[test]
    fn several_movement_commands_two() {
        let start_position = Coordinates { x: 5, y: 2 };
        let expected_position = Coordinates {x: 5, y: 2 };
        let mut rover = MarsRover::new(start_position, West, 6);
        let commands_input = "FBLLRFB";
        rover.command_input(commands_input);
        assert_eq!(rover.position, expected_position);
    }

    #[test]
    fn wrapping_around_the_north_pole_forward() {
        let start_position = Coordinates { x: 8, y: 1 };
        let expected_position = Coordinates {x: 5, y: 1 };
        let mut rover = MarsRover::new(start_position, North, 8);
        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }

    #[test]
    fn wrapping_around_the_north_pole_forward_two() {
        let start_position = Coordinates { x: 7, y: 1 };
        let expected_position = Coordinates {x: 4, y: 1 };
        let mut rover = MarsRover::new(start_position, North, 8);
        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }

    #[test]
    fn wrapping_around_the_south_pole_forward() {
        let start_position = Coordinates { x: 1, y: 8 };
        let expected_position = Coordinates {x: 5, y: 8 };
        let mut rover = MarsRover::new(start_position, South, 8);
        rover.move_rover(Forward);
        assert_eq!(rover.position, expected_position);
    }
}