use std::collections::HashSet;
use std::collections::LinkedList;

type Maze = Vec<Vec<char>>;

type Coordinate = (i32, i32);

fn find_starting_coordinates(maze: &Maze) -> Option<Coordinate> {
  const STARTING_SYMBOL: char = 'S';

  for x in 0..maze.len() {
    for y in 0..maze[x].len() {
      if maze[x][y] == STARTING_SYMBOL {
        return Some((x as i32, y as i32));
      }
    }
  }

  None
}

fn is_exit(coordinate_value: char) -> bool {
  const EXIT_SYMBOL: char = 'E';

  coordinate_value == EXIT_SYMBOL
}

fn is_valid_coordinate((x, y): &(i32, i32), maze: &Maze) -> bool {
  let rows = maze.len() as i32;

  let columns = maze[0].len() as i32;

  *x >= 0 && *x < rows && *y >= 0 && *y < columns
}

fn is_wall(coordinate_value: char) -> bool {
  const WALL_SYMBOL: char = '#';

  coordinate_value == WALL_SYMBOL
}

fn get_neighbors_coordinates(coordinate: Coordinate, maze: &Maze) -> Vec<Coordinate> {
  let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

  let (current_x, current_y) = coordinate;

  directions
    .iter()
    .filter_map(|direction| {
      let neighbor = (current_x + direction.0, current_y + direction.1);

      if !is_valid_coordinate(&neighbor, maze)
        || is_wall(maze[neighbor.0 as usize][neighbor.1 as usize])
      {
        None
      } else {
        Some(neighbor)
      }
    })
    .collect()
}

pub fn count_steps_to_exit(maze: &Maze) -> Option<i32> {
  let mut visited_coordinates = HashSet::new();

  let starting_coordinates =
    find_starting_coordinates(&maze).expect("maze without starting coordinate");

  let mut queue = LinkedList::new();

  queue.push_back((0, starting_coordinates));

  while !queue.is_empty() {
    let (steps, (x, y)) = queue.pop_front().unwrap();

    let coordinate_value: char = maze[x as usize][y as usize];

    if is_exit(coordinate_value) {
      return Some(steps);
    }

    if visited_coordinates.contains(&(x, y)) {
      continue;
    }

    visited_coordinates.insert((x, y));

    let neighbors_coordinates = get_neighbors_coordinates((x, y), &maze);

    for coordinate in &neighbors_coordinates {
      queue.push_back((steps + 1, *coordinate));
    }
  }

  None
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let test_cases = vec![
      (
        vec![
          vec!['S', '.', '.', '#', '.', '.', '.'],
          vec!['.', '#', '.', '.', '.', '#', '.'],
          vec!['.', '#', '.', '.', '.', '.', '.'],
          vec!['.', '.', '#', '#', '.', '.', '.'],
          vec!['#', '.', '#', 'E', '.', '#', '.'],
        ],
        Some(9),
      ),
      (
        vec![
          vec!['S', '.', '.', '#', '.', '.', '.'],
          vec!['.', '#', '.', '#', '.', '#', '.'],
          vec!['.', '#', '.', '#', '.', '.', '.'],
          vec!['.', '.', '#', '#', '.', '.', '.'],
          vec!['#', '.', '#', 'E', '.', '#', '.'],
        ],
        None,
      ),
    ];

    for (maze, expected_steps) in test_cases {
      let actual = count_steps_to_exit(&maze);

      assert_eq!(expected_steps, actual);
    }
  }
}
