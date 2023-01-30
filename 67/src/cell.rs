
#[derive(Debug)]
pub struct Cell {
    pub value: u8,
    pub sum_value: u16,
}

impl Cell {
    pub fn new(value: u8) -> Cell {
        Cell{
            value,
            sum_value: 0
        }
    }
    pub fn update(&mut self, max_sum_value: u16) {
        if self.value as u16 + max_sum_value > self.sum_value {
            self.sum_value = self.value as u16 + max_sum_value;
        }
    }
}


