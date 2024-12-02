fn first(data: &str) -> anyhow::Result<i64> {
    let mut result = 0;
    for line in data.split("\n") {
        let mut previous = Option::<i64>::None;
        let mut is_negative = Option::<bool>::None;
        let mut safe = true;
        for number in line.split_whitespace().flat_map(|s| s.parse::<i64>()) {
            match (previous, is_negative) {
                (None, None) => {
                    let _ = previous.insert(number);
                },
                (Some(p), None) => {
                    let diff = p - number;
                    if !(1..=3).contains(&diff.abs()) {
                        safe = false;
                        break;
                    }
                    let _ = previous.insert(number);
                    let _ = is_negative.insert(diff.is_negative());
                },
                (Some(p), Some(d)) => {
                    let diff = p - number;
                    if !(1..=3).contains(&diff.abs()) || d != diff.is_negative() {
                        safe = false;
                        break;
                    }
                    let _ = previous.insert(number);
                },
                (None, Some(_)) => anyhow::bail!("unexpected state"),
            }
        }
        if safe {
            result += 1;
        }
    }
    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let mut result = 0;
    for line in data.split("\n") {
        let numbers = line.split_whitespace().flat_map(|s| s.parse::<i64>()).collect::<Vec<_>>();
        let mut previous = Option::<i64>::None;
        let mut is_negative = Option::<bool>::None;
        let mut is_failed = false;
        let mut skip_index = 0;
        let mut index = 0;
        while index < numbers.len() && skip_index < numbers.len()  {
            if is_failed && skip_index == index {
                index +=1;
                continue;
            }
            let number = numbers[index];
            let mut should_reset = false;
            match (previous, is_negative) {
                (None, None) => {
                    let _ = previous.insert(number);
                    index += 1;
                },
                (Some(p), None) => {
                    let diff = p - number;
                    if !(1..=3).contains(&diff.abs()) {
                        should_reset = true;
                    }
                    let _ = previous.insert(number);
                    let _ = is_negative.insert(diff.is_negative());
                    index += 1;
                },
                (Some(p), Some(d)) => {
                    let diff = p - number;
                    if !(1..=3).contains(&diff.abs()) || d != diff.is_negative() {
                        should_reset = true;
                    }
                    let _ = previous.insert(number);
                    index += 1;
                },
                (None, Some(_)) => anyhow::bail!("unexpected state"),
            }

            if should_reset {
                if is_failed {
                    skip_index += 1;
                } else {
                    is_failed = true;
                }
                index = 0;
                previous = None;
                is_negative = None;
                continue;
            }
        }
        if skip_index < numbers.len() {
            result += 1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 2;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 591);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 53);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 621);
    }
}