use std::collections::{HashMap, HashSet};
use std::result;
use std::str::FromStr;

fn first(data: &str) -> anyhow::Result<i64> {
    fn check(result: i64, test: Vec<i64>) -> bool {
        if test.len() == 1 {
            result == test[0]
        } else if let Some((head, tail)) = test.split_first() {
            (result % head == 0 && check(result / head, tail.to_vec())) || check(result - head, tail.to_vec())    
        } else {
            false
        }
    }

    let mut result = 0;

    for line in data.lines() {
        let mut line = line.split(": ");
        let head = line.next().map(i64::from_str).and_then(Result::ok).unwrap_or_default();
        let tail = line.next().unwrap_or_default().split_ascii_whitespace().flat_map(i64::from_str).rev().collect::<Vec<_>>();

        if check(head, tail) {
            result += head;
        }
    }
    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    fn check(result: Option<i64>, test: Vec<i64>, acc: &mut HashSet<i64>) {
        let Some((head, tail)) = test.split_first() else {
            return;
        };
        let mul = result.unwrap_or(1) * head;
        let sum = result.unwrap_or(0) + head;
        let conact= result.map(|result| format!("{}{}", result, head).parse::<i64>().unwrap_or_default()).unwrap_or(*head) ;

        if tail.is_empty() {
            acc.insert(mul);
            acc.insert(sum);
            acc.insert(conact);
        } else {
            check(Some(mul), tail.to_vec(), acc);
            check(Some(sum), tail.to_vec(), acc);
            check(Some(conact), tail.to_vec(), acc);
        }
    }

    let mut result = 0;

    for line in data.lines() {
        let mut line = line.split(": ");
        let head = line.next().map(i64::from_str).and_then(Result::ok).unwrap_or_default();
        let tail = line.next().unwrap_or_default().split_ascii_whitespace().flat_map(i64::from_str).collect::<Vec<_>>();

        let mut acc = HashSet::<i64>::new();
        check(None, tail, &mut acc);
        if acc.contains(&head) {
            result += head;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 7;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 3749);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 3312271365652);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 11387);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 509463489296712);
    }
}
