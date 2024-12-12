use std::collections::{HashMap, HashSet};
use std::mem::swap;
use std::{char, iter, result};
use std::str::FromStr;

use crate::matrix::{Matrix, MatrixIndex, MatrixIter};



fn first(data: &str) -> anyhow::Result<i64> {
    fn find(prev: i64, iter: MatrixIter<i64>, acc: &mut HashSet<MatrixIndex>) {
        if prev == 9 {
            acc.insert(iter.index());
            return;
        }

        let mut result = 0;
        let current = prev + 1;

        let mut n_iter = iter;
        if Some(&current) == n_iter.n() {
            find(current, n_iter, acc);
        }

        let mut e_iter = iter;
        if Some(&current) == e_iter.e() {
            find(current, e_iter, acc);
        }

        let mut s_iter = iter;
        if Some(&current) == s_iter.s() {
            find(current, s_iter, acc);
        }

        let mut w_iter = iter;
        if Some(&current) == w_iter.w() {
            find(current, w_iter, acc);
        }
    }

    let matrix = Matrix::<i64>::from(data);

    let mut result = 0;
    let iter = matrix.iter();

    for (index, number) in  matrix.iter().enumerate() {
        if *number == 0 {
            let mut acc = HashSet::<MatrixIndex>::new();
            find(0, iter.skip(index as u64), &mut acc);
            result += acc.len() as i64;
        }
    }

    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    fn find(prev: i64, iter: MatrixIter<i64>) -> i64 {
        if prev == 9 {
            return 1;
        }

        let mut result = 0;
        let current = prev + 1;

        let mut n_iter = iter;
        if Some(&current) == n_iter.n() {
            result += find(current, n_iter);
        }

        let mut e_iter = iter;
        if Some(&current) == e_iter.e() {
            result += find(current, e_iter);
        }

        let mut s_iter = iter;
        if Some(&current) == s_iter.s() {
            result += find(current, s_iter);
        }

        let mut w_iter = iter;
        if Some(&current) == w_iter.w() {
            result += find(current, w_iter);
        }

        result
    }

    let matrix = Matrix::<i64>::from(data);

    let mut result = 0;
    let iter = matrix.iter();

    for (index, number) in  matrix.iter().enumerate() {
        if *number == 0 {
            result += find(0, iter.skip(index as u64));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;
    use super::*;

    const DAY: i8 = 10;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 667);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 81);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 1344);
    }
}
