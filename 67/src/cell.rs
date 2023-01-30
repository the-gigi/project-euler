
#[derive(Debug)]
pub struct Cell {
    row: u8,
    index: u8,
    value: u8,
    max_sum_value: u16,
}

impl Cell {
    pub fn new(row: u8, index: u8, value: u8) -> Cell {
        Cell{
            row,
            index,
            value,
            max_sum_value: 0
        }
    }
    fn update(&mut self, max_sum_value: u16) {
        self.max_sum_value = max_sum_value;
    }
}


