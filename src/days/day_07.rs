use std::ops::{Add, Mul};

use itertools::{repeat_n, Itertools};

struct Calibration {
    result: u128,
    inputs: Vec<u128>,
}

type Operation = fn(u128, u128) -> u128;

impl Calibration {
    fn calculate(&self, operators: Vec<&Operation>) -> u128 {
        let mut result: u128 = self.inputs[0];
        for i in 1..self.inputs.len() {
            result = operators[i - 1](result, self.inputs[i]);
        }
        return result;
    }
}

pub fn run(input: &Vec<String>) {
    let calibrations = parse_input(input);

    // Part 1
    let simple_operators = vec![u128::add, u128::mul];
    let sum_test_results = sum_valid_calibrations(&calibrations, &simple_operators);
    println!(
        "(Part 1) The sum of valid test results with + and * is {}",
        sum_test_results
    );

    let complex_operators = vec![u128::add, u128::mul, concat_numbers];

    // Part 2
    let sum_test_results = sum_valid_calibrations(&calibrations, &complex_operators);
    println!(
        "(Part 2) The sum of valid test results with + and * and || is {}",
        sum_test_results
    );
}

fn concat_numbers(a: u128, b: u128) -> u128 {
    let magnitude = (b as f64).log10().ceil() as u32;
    let left_shift: u128 = 10_u128.pow(magnitude);
    return a.checked_mul(left_shift).unwrap().checked_add(b).unwrap();
}

fn sum_valid_calibrations(
    calibrations: &Vec<Calibration>,
    possible_operators: &Vec<Operation>,
) -> u128 {
    let mut sum: u128 = 0;

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
        let result: u128 = left.parse().unwrap();
        let mut inputs: Vec<u128> = Vec::new();
        for number in right.split_whitespace() {
            inputs.push(number.parse().unwrap());
        }
        results.push(Calibration { result, inputs });
    }

    return results;
}
