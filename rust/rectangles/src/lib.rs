extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

fn valid_rect(p1: &(usize, usize),
              p2: &(usize, usize),
              board: &HashMap<(usize, usize), char>)
              -> bool {
    let &(p1x, p1y) = p1;
    let &(p2x, p2y) = p2;

    // helper functions
    let reorder = |n, m| if n > m {
        (m, n)
    } else {
        (n, m)
    };
    let is_corner = |x, y| *board.get(&(x, y)).unwrap() == '+';
    let horizontal_line = |i, x1, x2| {
        let (x1, x2) = reorder(x1, x2);
        board.iter()
             .filter(|&(&(x, y), _)| i == y && x1 < x && x < x2)
             .all(|(_, &chr)| chr == '-' || chr == '+')
    };

    let vertical_line = |i, y1, y2| {
        let (y1, y2) = reorder(y1, y2);
        board.iter()
             .filter(|&(&(x, y), _)| i == x && y1 < y && y < y2)
             .all(|(_, &chr)| chr == '|' || chr == '+')
    };

    // if coordinates are in the same column/row they are not corners
    if p1x == p2x || p1y == p2y {
        false
    }
    // check all four corners are + characters
    else if !is_corner(p1x, p2y) || !is_corner(p2x, p1y) {
        false
        // has all four sides as continous lines
    } else if horizontal_line(p1y, p1x, p2x) && horizontal_line(p2y, p1x, p2x) &&
       vertical_line(p1x, p1y, p2y) && vertical_line(p2x, p1y, p2y) {
        true
    } else {
        false
    }
}

pub fn count(lines: &Vec<&str>) -> u32 {
    // let mut board: Vec<Vec<char>> = Vec::new();
    let mut board: HashMap<(usize, usize), char> = HashMap::new();
    let mut corners: Vec<(usize, usize)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            board.insert((x, y), chr);
            if chr == '+' {
                corners.push((x, y));
            }
        }
    }
    let mut ans = 0;
    // iterate every corner pair candidate
    for candidate in corners.iter().combinations_n(2) {
        if valid_rect(candidate[0], candidate[1], &board) {
            ans += 1
        };
    }
    // we counted every rectange twice with the two diagonal corners
    ans / 2
}
