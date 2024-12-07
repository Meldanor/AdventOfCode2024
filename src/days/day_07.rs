use std::ops::{Add, Mul};

use itertools::{repeat_n, Itertools};

struct Calibration {
    result: usize,
    inputs: Vec<usize>,
}

type Operation = fn(usize, usize) -> usize;

impl Calibration {
    fn calculate(&self, operators: Vec<Operation>) -> usize {
        let mut result: usize = self.inputs[0];
        for i in 1..self.inputs.len() {
            result = operators[i - 1](result, self.inputs[i]);
        }
        return result;
    }
}

pub fn run(input: &Vec<String>) {
    let calibrations = parse_input(input);
    let sum_test_results = sum_valid_calibrations(&calibrations);
    println!(
        "(Part 1) The sum of valid test results is {}",
        sum_test_results
    );
}

fn sum_valid_calibrations(calibrations: &Vec<Calibration>) -> usize {
    let mut sum: usize = 0;
    let possible_operators: [Operation; 2] = [usize::add, usize::mul];
    for calibration in calibrations {
        'inner: for operators in
            repeat_n(possible_operators, calibration.inputs.len() - 1).multi_cartesian_product()
        {
            if calibration.calculate(operators) == calibration.result {
                sum += calibration.result;
                break 'inner;
            }
        }
    }
    return sum;
}

fn parse_input(input: &Vec<String>) -> Vec<Calibration> {
    let mut results: Vec<Calibration> = Vec::new();

    for line in input {
        let (left, right) = line.split_once(":").unwrap();
        let result: usize = left.parse().unwrap();
        let mut inputs: Vec<usize> = Vec::new();
        for number in right.split_whitespace() {
            inputs.push(number.parse().unwrap());
        }
        results.push(Calibration { result, inputs });
    }

    return results;
}
