use std::char;
use std::collections::HashMap;


#[derive(PartialEq)]
enum Cell {
    Empty,
    Mine,
}

fn neighbors(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let x1 = x1 as isize;
    let x2 = x2 as isize;
    let y1 = y1 as isize;
    let y2 = y2 as isize;

    x1 - 1 == x2 && y1 - 1 == y2 || x1 == x2 && y1 - 1 == y2 || x1 + 1 == x2 && y1 - 1 == y2 ||
    x1 - 1 == x2 && y1 + 1 == y2 ||
    x1 == x2 && y1 + 1 == y2 || x1 + 1 == x2 && y1 + 1 == y2 || x1 - 1 == x2 && y1 == y2 ||
    x1 + 1 == x2 && y1 == y2
}

fn calc_value(x: usize, y: usize, cells: &HashMap<(usize, usize), Cell>) -> char {
    let mut n = 0;
    for (&(cx, cy), cell) in cells.iter() {
        if neighbors(x, y, cx, cy) {
            n += match cell {
                &Cell::Empty => 0,
                &Cell::Mine => 1,
            }
        }
    }
    if n == 0 {
        ' '
    } else {
        char::from_digit(n, 10).unwrap()
    }
}

pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    let mut cells: HashMap<(usize, usize), Cell> = HashMap::new();
    let height = board.len();
    let width = board[0].len();

    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            let c = match cell {
                ' ' => Cell::Empty,
                '*' => Cell::Mine,
                _ => panic!(),
            };
            cells.insert((x, y), c);
        }
    }

    let mut ret = Vec::new();
    for y in 0..height {
        let mut row = String::new();
        for x in 0..width {
            let cell = cells.get(&(x, y)).unwrap();
            let c = match cell {
                &Cell::Empty => calc_value(x, y, &cells),
                &Cell::Mine => '*',
            };
            row.push(c);
        }
        ret.push(row);
    }
    ret
}
