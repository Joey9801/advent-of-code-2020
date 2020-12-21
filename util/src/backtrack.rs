struct BtStackElement<S: BacktrackState + ?Sized> {
    index: S::Index,
    values: S::ValueIter,
}

pub enum BacktrackResult {
    Success {
        iter_count: usize
    },
    Failure,
}

impl BacktrackResult {
    fn is_success(&self) -> bool {
        match self {
            BacktrackResult::Success { .. } => true,
            BacktrackResult::Failure => false,
        }
    }
}

pub trait BacktrackState {
    type Index: Copy;
    type Value: Copy;
    type ValueIter: Iterator<Item=Self::Value>;
    
    fn is_solved(&self) -> bool;
    fn set(&mut self, index: Self::Index, value: Option<Self::Value>);
    fn next_index(&self) -> Option<Self::Index>;
    fn possible_values(&self, index: Self::Index) -> Self::ValueIter;
    
    fn backtrack_solve(&mut self) -> BacktrackResult {
        let mut stack: Vec<BtStackElement<Self>> = Vec::new();
        let mut iter_count = 0;

        loop {
            // Each iteration we can either:
            //    1. Declare the problem solved
            //    2. Add a new index to the stack
            //    3. Pop elements from the stack until the head of the stack has next available option, and apply that option
            //    4. Declare the problem unsolvable if the stack ran out of elements during 3.

            // 1. Declare the problem solved
            if self.is_solved() {
                break BacktrackResult::Success { iter_count };
            }
            
            // 2. Attempt to add a new index to the stack
            match self.next_index() {
                Some(index) => {
                    let values = self.possible_values(index);
                    stack.push(BtStackElement { index, values });
                    // Fall through to 3 -> will end up applying the first value of the iterator to current_state
                },
                None => (),
            }

            // 3. Pop stack elements until the head can be advanced to the next value
            while stack.len() > 0 {
                let (head_index, next_head_value) = {
                    let head = stack.last_mut().unwrap();
                    (head.index, head.values.next())
                };
                
                self.set(head_index, next_head_value);
                match next_head_value {
                    Some(_) => break,
                    None => { stack.pop(); }
                }
            }

            // 4. Declare the problem unsolvable
            if stack.len() == 0 {
                break BacktrackResult::Failure;
            }
            
            iter_count += 1;
        }
    }
}

#[cfg(test)]
mod sudoku_test {
    use std::iter;
    use super::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct SudokuBoard {
        numbers: [u8; 9 * 9],
    }
    struct NextValueIter {
        storage: [u8; 9],
        len: usize,
        next: usize,
    }
    
    impl Iterator for NextValueIter {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next == self.len {
                None
            } else {
                self.next += 1;
                Some(self.storage[self.next - 1])
            }
        }
    }
    

    impl BacktrackState for SudokuBoard {
        type Index = (u8, u8);
        type Value = u8;
        type ValueIter = NextValueIter;

        fn is_solved(&self) -> bool {
            self.numbers.iter().all(|x| *x != 0)
        }

        fn set(&mut self, (x, y): Self::Index, value: Option<Self::Value>) {
            let offset = (x + y * 9) as usize;
            match value {
                Some(value) => self.numbers[offset] = value,
                None => self.numbers[offset] = 0,
            }
        }
        
        fn next_index(&self) -> Option<Self::Index> {
            let offset = self.numbers
                .iter()
                .enumerate()
                .filter(|(_offset, x)| **x == 0)
                .next()
                .map(|(offset, _)| offset)? as u8;
                
            let x = offset % 9;
            let y = offset / 9;
            
            Some((x, y))
        }
        
        fn possible_values(&self, (x, y): (u8, u8)) -> Self::ValueIter {
            let mut possible = [true; 9];
            // Top left of the subsquare
            let tl = ((x / 3) * 3, (y / 3) * 3);
            for (x, y) in 
                (0..9).zip(iter::repeat(y)) // All elements in the row
                    .chain(iter::repeat(x).zip(0..9)) // All elements in the column
                    .chain((tl.0..(tl.0 + 3)).flat_map(|x| iter::repeat(x).zip(tl.1..(tl.1 + 3)))) // The subsquare
            {
                match self.numbers[(x + y * 9) as usize] {
                    0 => (),
                    x if x <= 9 => possible[x as usize - 1] = false,
                    _ => unreachable!(),
                }
            }
            
            let mut storage = [0u8; 9];
            let mut len = 0;
            (1..=9)
                .filter(|x| possible[*x as usize - 1])
                .enumerate()
                .for_each(|(idx, x)| {
                    storage[idx] = x;
                    len += 1;
                });

            NextValueIter {
                storage,
                len,
                next: 0,
            }
        }
    }
    
    #[test]
    fn test_backtracker_solves_sudoku() {
        let puzzle = SudokuBoard {
            numbers: [
                5, 3, 0,   0, 7, 0,   0, 0, 0,
                6, 0, 0,   1, 9, 5,   0, 0, 0,
                0, 9, 8,   0, 0, 0,   0, 6, 0,

                8, 0, 0,   0, 6, 0,   0, 0, 3,
                4, 0, 0,   8, 0, 3,   0, 0, 1,
                7, 0, 0,   0, 2, 0,   0, 0, 6,

                0, 6, 0,   0, 0, 0,   2, 8, 0,
                0, 0, 0,   4, 1, 9,   0, 0, 5,
                0, 0, 0,   0, 8, 0,   0, 7, 9,
            ],
        };
        
        let expected_solution = SudokuBoard {
            numbers: [
                5, 3, 4,   6, 7, 8,   9, 1, 2,
                6, 7, 2,   1, 9, 5,   3, 4, 8,
                1, 9, 8,   3, 4, 2,   5, 6, 7,

                8, 5, 9,   7, 6, 1,   4, 2, 3,
                4, 2, 6,   8, 5, 3,   7, 9, 1,
                7, 1, 3,   9, 2, 4,   8, 5, 6,

                9, 6, 1,   5, 3, 7,   2, 8, 4,
                2, 8, 7,   4, 1, 9,   6, 3, 5,
                3, 4, 5,   2, 8, 6,   1, 7, 9,
            ]
        };
        
        let mut bt_solution = puzzle.clone();
        let bt_result = bt_solution.backtrack_solve();
        assert!(bt_result.is_success());
        assert_eq!(bt_solution, expected_solution);
    }
}