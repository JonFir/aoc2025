use std::iter::zip;
use std::str::FromStr;

fn first(data: &str) -> anyhow::Result<i64> {
    let mut first = Vec::<i64>::new();
    let mut second = Vec::<i64>::new();
    for line in data.split("\n") {
        let numbers = line.split_whitespace().flat_map(|s| s.parse::<i64>()).collect::<Vec<_>>();
        first.push(numbers[0]);
        second.push(numbers[1]);
    }
    first.sort();
    second.sort();
    let summ = zip(first, second).map(|pair| (pair.0 - pair.1).abs()).sum();
    Ok(summ)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let mut first = Vec::<i64>::new();
    let mut second = Vec::<i64>::new();
    for line in data.split("\n") {
        let numbers = line.split_whitespace().flat_map(|s| s.parse::<i64>()).collect::<Vec<_>>();
        first.push(numbers[0]);
        second.push(numbers[1]);
    }
    first.sort();
    second.sort();
    let mut f = 0;
    let mut s = 0;
    let mut c = 0i64;
    let mut summ = 0i64;
    while  f < first.len() && s < second.len() {
        match first[f].cmp(&second[s]) {
            std::cmp::Ordering::Equal => {
                c += 1;
                s += 1;
            },
            std::cmp::Ordering::Greater => {
                s += 1;
            },
            std::cmp::Ordering::Less => {
                let result = first[f] * c;
                summ += result;
                c = 0;
                f += 1;
                while f > 0 && f < first.len() && first[f] == first[f-1] {
                    summ += result;
                    f += 1;
                }
            },
        }
    }
    Ok(summ)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    #[test]
    fn t1() {
        let data = data(1, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn i1() {
        let data = data(1, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 1873376);
    }

    #[test]
    fn t2() {
        let data = data(1, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 31);
    }

    #[test]
    fn i2() {
        let data = data(1, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 18997088);
    }
}