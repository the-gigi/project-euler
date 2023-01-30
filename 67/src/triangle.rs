const MAX_ROW: u8 = 99;
const MAX_VALUE: u8 = 99;

use crate::cell;
use std::{fs, vec};
use fstrings;
use crate::cell::Cell;

pub struct Triangle {
    rows: Vec<Vec<cell::Cell>>,
}

impl Triangle {
    pub fn new(filename: &str) -> Triangle {
        let mut result: Triangle = Triangle{
            rows: vec![]
        };
        let contents = fs::read_to_string(filename).expect("failed to read file");
        let lines: Vec<&str> = contents.split('\n').collect();
        let expected_lines = (MAX_ROW + 1) as usize;
        if lines.len() != expected_lines {
            panic_f!("input file doesn't have a {expected_lines} lines")
        }
        for (row, line) in lines.iter().enumerate() {
            result.rows.push(vec![]);
            let values: Vec<&str> = line.split(' ').collect();
            let expected_values = row + 1;
            if values.len() != expected_values {
                panic_f!("row {row} in input file doesn't have {expected_values} values");
            }

            for (index, value) in values.iter().enumerate() {
                let value = value.parse::<u8>().unwrap();
                if value > MAX_VALUE {
                    panic_f!("row {row}, index {index} in input file has invalid value {value}")
                }
                result.rows[row].push(cell::Cell::new(row as u8, index as u8, value));
            }
        }
        result
    }

    fn get_left_child(&self, row: u8, index: u8) -> Option<&cell::Cell> {
        if row == MAX_ROW {
            return None
        }
        let cell= &(self.rows[(row + 1) as usize][index as usize]);
        Some(cell)
    }

    fn get_right_child(&self, row: u8, index: u8) -> Option<&cell::Cell> {
        if row == MAX_ROW {
            return None
        }

        let cell = &(self.rows[(row + 1) as usize][(index + 1) as usize]);
        Some(cell)
    }

    fn get_left_neighbor(&self, row: u8, index: u8) -> Option<&cell::Cell> {
        if index == 0 {
            return None
        }
        let cell = &(self.rows[row as usize][(index - 1) as usize]);
        Some(cell)
    }

    fn get_right_neighbor(&self, row: u8, index: u8) -> Option<&cell::Cell> {
        // The max index of each row is the row index
        if index == row {
            return None
        }

        let cell = &(self.rows[row as usize][(index + 1) as usize]);
        Some(cell)
    }

    pub fn find_max_total(&self) -> u16 {
        0
    }
}
