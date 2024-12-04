use std::{iter::{self, zip}, result};
use regex::Regex;

fn first(data: &str) -> anyhow::Result<i64> {
    let re = Regex::new(r"(?m)(mul\((\d{1,3}),(\d{1,3})\))")?;
    let result = re.captures_iter(data).map(|c| c.extract()).flat_map(|(_, [_, a, b])| {
        if let (Ok(a), Ok(b)) = (a.parse::<i64>(), b.parse::<i64>()) { Some(a*b) } else { None }
    }).sum();
    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let dont_re = Regex::new(r"(?ms)(don't.*?do[^n])")?;
    let data = dont_re.replace_all(data, "").into_owned();
    let re = Regex::new(r"(?m)(mul\((\d{1,3}),(\d{1,3})\))")?;
    let result = re.captures_iter(&data).map(|c| c.extract()).flat_map(|(_, [_, a, b])| {
        if let (Ok(a), Ok(b)) = (a.parse::<i64>(), b.parse::<i64>()) { Some(a*b) } else { None }
    }).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 3;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 182780583);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 48);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 90772405);
    }
}