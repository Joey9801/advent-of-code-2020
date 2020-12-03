const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Open,
    Tree,
}

impl Cell {
    fn from_byte(c: u8) -> Option<Self> {
        match c {
            b'.' => Some(Self::Open),
            b'#' => Some(Self::Tree),
            _ => None,
        }
    }
}

struct Map {
    data: Vec<Cell>,
    height: usize,
    width: usize,
}

impl Map {
    fn from_input(input: &str) -> Self {
        debug_assert!(input.is_ascii());

        let width = input.as_bytes().iter().position(|b| *b == b'\n').unwrap();
        let data = input.as_bytes()
            .iter()
            .filter_map(|b| Cell::from_byte(*b))
            .collect::<Vec<Cell>>();
        
        debug_assert!(data.len() % width == 0);
        let height = data.len() / width;

        Self {
            data,
            width,
            height,
        }
    }
    
    fn get_cell(&self, x: usize, y: usize) -> Cell {
        let x = x % self.width;
        self.data[y * self.width + x]
    }
    
    fn count_trees(&self, step_x: usize, step_y: usize) -> usize {
        assert!(step_y > 0);

        (0..self.height)
            .step_by(step_y)
            .enumerate()
            .map(|(xi, y)| self.get_cell(xi * step_x, y))
            .filter(|cell| *cell == Cell::Tree)
            .count()
    }
}

fn main() {
    let map = Map::from_input(INPUT);
    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    
    let product: usize = slopes.iter()
        .map(|(step_x, step_y)| map.count_trees(*step_x, *step_y))
        .product();

    println!("There were {} trees on the 3/1 slope", map.count_trees(3, 1));
    println!("Product of tree counts: {}", product);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_1() {
        let map = Map::from_input(INPUT);
        assert_eq!(map.count_trees(3, 1), 228);
    }
    
    #[test]
    fn test_part_2() {
        let slopes: [(usize, usize); 5] = [
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2)
        ];
        
        let map = Map::from_input(INPUT);
        let product: usize = slopes.iter()
            .map(|(step_x, step_y)| map.count_trees(*step_x, *step_y))
            .product();
        
        assert_eq!(product, 6818112000);
    }
}