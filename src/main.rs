use std::env;
use std::fmt;
use std::io::{self, Read};
use std::fs::File;
use std::collections::BTreeMap;


fn main() {
    let mut worker = Worker::new();
    let filename = match env::args().nth(1) {
        Some(filename) => {
            eprintln!("Got filename {}", filename);
            filename
        },
        None => {
            eprintln!("Usage {} [filename]", env::args().next().unwrap());
            return;
        }
    };
    let board = Board::from_reader(&mut File::open(filename).unwrap()).unwrap();
    println!("Initial: {}", &board);
    println!("Solution: {}", run(&board, &mut worker).unwrap());
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::N1 => 1,
                Value::N2 => 2,
                Value::N3 => 3,
                Value::N4 => 4,
                Value::N5 => 5,
                Value::N6 => 6,
                Value::N7 => 7,
                Value::N8 => 8,
                Value::N9 => 9,
            }
        )
    }
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

    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Board> {
        let mut data = String::new();
        let mut squares = Vec::with_capacity(81);
        reader.read_to_string(&mut data)?;
        use Value::*;
        for c in data.chars() {
            match c {
                '_' => squares.push(None),
                '1' => squares.push(Some(N1)),
                '2' => squares.push(Some(N2)),
                '3' => squares.push(Some(N3)),
                '4' => squares.push(Some(N4)),
                '5' => squares.push(Some(N5)),
                '6' => squares.push(Some(N6)),
                '7' => squares.push(Some(N7)),
                '8' => squares.push(Some(N8)),
                '9' => squares.push(Some(N9)),
                _ => {}
            }
        }
        Ok(Board::new(squares))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Some(value) => write!(f, "{} ", value)?,
                None => write!(f, "_ ")?,
            }
            if i % 9 == 8 {
                write!(f, "\n")?;
            } else if i % 3 == 2 {
                write!(f, "  ")?;
            }

            if i % 27 == 26 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

struct Worker {
    buffer: Vec<Value>,
    available_values: Vec<Value>,
    block_offsets: BTreeMap<(u8, u8), Vec<usize>>,
}

impl Worker {
    fn new() -> Worker {
        use Value::*;
        let mut worker = Worker {
            buffer: Vec::with_capacity(9),
            available_values: vec![N1, N2, N3, N4, N5, N6, N7, N8, N9],
            block_offsets: BTreeMap::new(),
        };
        worker.calculate_block_offsets();
        worker
    }

    fn possible(&mut self, board: &Board, position: u8) -> Vec<Value> {
        assert!(position < 81);

        let possible_row = self.constrain_row(board, position);
        let possible_column = self.constrain_column(board, position);
        let possible_block = self.constrain_block(board, position);
        self.available_values
            .iter()
            .filter(|x| possible_row.contains(x))
            .filter(|x| possible_column.contains(x))
            .filter(|x| possible_block.contains(x))
            .cloned()
            .collect()
    }

    fn constrain_row(&mut self, board: &Board, position: u8) -> Vec<Value> {
        let row = row(position);
        assert!(row < 9);

        self.buffer.truncate(0);
        let start = row as usize * 9;
        let end = start + 9;
        for pt in start..end {
            if let Some(val) = board.squares[pt] {
                self.buffer.push(val);
            }
        }
        self.available_values
            .iter()
            .filter(|x| !self.buffer.contains(*x))
            .cloned()
            .collect()
    }

    fn constrain_column(&mut self, board: &Board, position: u8) -> Vec<Value> {
        let column = column(position);
        assert!(column < 9);

        self.buffer.truncate(0);
        for i in 0..9 {
            let pt = i * 9 + column as usize;
            if let Some(val) = board.squares[pt] {
                self.buffer.push(val);
            }
        }
        self.available_values
            .iter()
            .filter(|x| !self.buffer.contains(*x))
            .cloned()
            .collect()
    }

    fn calculate_block_offsets(&mut self) {
        for blockrow in 0..3 {
            for blockcol in 0..3 {
                let row = blockrow as usize * 3;
                let col = blockcol as usize * 3;

                self.block_offsets.insert((blockrow, blockcol), vec![
                    9 * row + col,
                    9 * row + col + 1,
                    9 * row + col + 2,
                    9 * (row + 1) + col,
                    9 * (row + 1) + col + 1,
                    9 * (row + 1) + col + 2,
                    9 * (row + 2) + col,
                    9 * (row + 2) + col + 1,
                    9 * (row + 2) + col + 2,
                ]);
            }
        }
    }

    fn constrain_block(&mut self, board: &Board, position: u8) -> Vec<Value> {
        let block = block(position);
        assert!(block.0 < 3);
        assert!(block.1 < 3);

        self.buffer.truncate(0);

        let offsets = self.block_offsets.get(&block).expect("offset map");

        for pt in offsets {
            if let Some(val) = board.squares[*pt] {
                self.buffer.push(val);
            }
        }
        self.available_values
            .iter()
            .filter(|x| !self.buffer.contains(*x))
            .cloned()
            .collect()
    }
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
    (y, x)
}

fn next(board: &Board, worker: &mut Worker) -> Vec<Board> {
    use Value::*;
    let mut least_possible = vec![N1, N2, N3, N4, N5, N6, N7, N8, N9];
    let mut least_position: usize = 100;
    for (position, value) in board.squares.iter().enumerate() {
        if value.is_none() {
            let poss = worker.possible(board, position as u8);
            if poss.len() < least_possible.len() {
                least_possible = poss;
                least_position = position;
            }
        }
    }
    let boards = least_possible.to_owned()
        .into_iter()
        .map(|val| {
            let mut board = board.clone();
            board.squares[least_position] = Some(val);
            board
        })
        .collect::<Vec<_>>();
    if boards.len() > 1 {
        println!("-----");
        for (i, board) in boards.iter().enumerate() {
            println!("{}.\n{}", i + 1, board);
        }
    }
    boards
}

fn run(board: &Board, worker: &mut Worker) -> Option<Board> {
    let choices = next(board, worker);
    for choice in choices {
        if !choice.squares.contains(&None) {
            return Some(choice);
        }
        let outcome = run(&choice, worker);
        if outcome.is_some() {
            return outcome;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{next, Board, Value::*};

    #[test]
    fn test_next() {
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
        assert_eq!(next(&board), vec![expected1, expected2])
    }
    #[test]
    fn test_next_with_constrained_block() {
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
        assert_eq!(next(&board), vec![expected1])
    }
}
