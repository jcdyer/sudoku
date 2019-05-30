fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Value {
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Board {
    squares: Vec<Option<Value>>,
}

impl Board {
    /// Create a new Sudoku board.  Must be passed a board of 81 squares
    pub fn new(squares: Vec<Option<Value>>) -> Board {
        if squares.len() != 81 {
            panic!("Improper board size")
        }
        Board { squares }
    }
}
fn constrain_row(board: &Board, row: u8) -> Vec<Value> {
    assert!(row < 9);
    let mut possible = Vec::with_capacity(9);
    let start = row as usize * 9;
    let end = start + 9;
    for pt in start..end {
        if let Some(val) = board.squares[pt] {
            possible.push(val);
        }
    }
    use Value::*;
    vec![N1, N2, N3, N4, N5, N6, N7, N8, N9]
        .into_iter()
        .filter(|x| !possible.contains(x))
        .collect()
}

fn constrain_column(board: &Board, column: u8) -> Vec<Value> {
    assert!(column < 9);
    let mut possible = Vec::with_capacity(9);
    for i in 0..9 {
        let pt = i * 9 + column as usize;
        if let Some(val) = board.squares[pt] {
            possible.push(val);
        }
    }
    use Value::*;
    vec![N1, N2, N3, N4, N5, N6, N7, N8, N9]
        .into_iter()
        .filter(|x| !possible.contains(x))
        .collect()
}

fn constrain_block(board: &Board, block: (u8, u8)) -> Vec<Value> {
    // Unimplemented
    assert!(block.0 < 3);
    assert!(block.1 < 3);
    use Value::*;
    let col = block.0 as usize * 3;
    let row = block.1 as usize * 3;
    let positions =vec![
        9 * row + col,
        9 * row + col + 1,
        9 * row + col + 2,
        9 * (row + 1) + col,
        9 * (row + 1) + col + 1,
        9 * (row + 1) + col + 2,
        9 * (row + 2) + col,
        9 * (row + 2) + col + 1,
        9 * (row + 2) + col + 2,
    ];
    let mut possible = Vec::with_capacity(9);
    for pt in positions {
        if let Some(val) = board.squares[pt] {
            possible.push(val);
        }
    }
    use Value::*;
    vec![N1, N2, N3, N4, N5, N6, N7, N8, N9]
        .into_iter()
        .filter(|x| !possible.contains(x))
        .collect()
}

fn row(position: u8) -> u8 {
    assert!(position < 81);
    position / 9
}
fn column(position: u8) -> u8 {
    assert!(position < 81);
    position % 9
}

fn block(position: u8) -> (u8, u8) {
    let x = column(position) / 3;
    let y = row(position) / 3;
    (x, y)
}

fn possible(board: &Board, position: u8) -> Vec<Value> {
    assert!(position < 81);
    use Value::*;
    let possible_row = constrain_row(board, row(position));
    let possible_column = constrain_column(board, column(position));
    let possible_block = constrain_block(board, block(position));
    vec![N1, N2, N3, N4, N5, N6, N7, N8, N9]
        .into_iter()
        .filter(|x| possible_row.contains(x))
        .filter(|x| possible_column.contains(x))
        .filter(|x| possible_block.contains(x))
        .collect()
}

fn step(board: &Board) -> Vec<Board> {
    use Value::*;
    let mut least_possible = vec![N1, N2, N3, N4, N5, N6, N7, N8, N9];
    let mut least_position: usize = 100;
    for (position, value) in board.squares.iter().enumerate() {
        if value.is_none() {
            let poss = possible(board, position as u8);
            println!("Position: {}, Value: {:?}", position, poss);
            if poss.len() < least_possible.len() {
                least_possible = poss;
                least_position = position;
            }
        }
    }
    least_possible
        .into_iter()
        .map(|val| {
            let mut board = board.clone();
            board.squares[least_position] = Some(val);
            board
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{step, Board, Value::*};

    #[test]
    fn test_step() {
        #[rustfmt::skip]
        let board = Board::new(vec![
            Some(N1),Some(N2),Some(N3),Some(N4),Some(N5),Some(N6),None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,Some(N9),
        ]);
        #[rustfmt::skip]
        let expected1 = Board::new(vec![
            Some(N1),Some(N2),Some(N3),Some(N4),Some(N5),Some(N6),None,None,Some(N7),
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,Some(N9),
        ]);
        #[rustfmt::skip]
        let expected2 = Board::new(vec![
            Some(N1),Some(N2),Some(N3),Some(N4),Some(N5),Some(N6),None,None,Some(N8),
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,Some(N9),
        ]);
        assert_eq!(step(&board), vec![expected1, expected2])
    }
    #[test]
    fn test_step_with_constrained_block() {
        #[rustfmt::skip]
        let board = Board::new(vec![
            Some(N1),Some(N2),Some(N3),Some(N4),Some(N5),Some(N6),None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,Some(N8),None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,Some(N9),
        ]);
        #[rustfmt::skip]
        let expected1 = Board::new(vec![
            Some(N1),Some(N2),Some(N3),Some(N4),Some(N5),Some(N6),None,None,Some(N7),
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,Some(N8),None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,None,
            None,None,None,None,None,None,None,None,Some(N9),
        ]);

        assert_eq!(step(&board), vec![expected1])
    }
}

