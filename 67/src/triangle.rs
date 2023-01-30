const MAX_ROW: usize = 99;
const MAX_VALUE: u8 = 99;

use std::{fs, vec};
use crate::cell::Cell;

pub struct Triangle {
    rows: Vec<Vec<Cell>>,
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
            let values: Vec<&str> = values.into_iter().filter(|x| *x != "").collect();
            let expected_values = row + 1;
            if values.len() != expected_values {
                panic_f!("row {row} in input file doesn't have {expected_values} values");
            }

            for (index, value) in values.iter().enumerate() {
                let value = value.parse::<u8>().unwrap();
                if value > MAX_VALUE {
                    panic_f!("row {row}, index {index} in input file has invalid value {value}")
                }
                result.rows[row].push(Cell::new(value));
            }
        }
        result
    }

    pub fn find_max_total(&mut self) -> u16 {
        self.rows[0][0].update(0);
        for ri in 0..MAX_ROW {
            for ci in 0..(ri + 1) {
                let curr_sum_value = self.rows[ri][ci].sum_value;

                // Update left and right children
                self.rows[ri+1][ci].update(curr_sum_value);
                self.rows[ri+1][ci+1].update(curr_sum_value);
            }
        }

        let mut max_total: u16 = 0;
        for ci in 0..MAX_ROW {
            max_total = std::cmp::max(max_total, self.rows[MAX_ROW][ci].sum_value);
        }
        return max_total
    }
}
