use std::collections::{HashMap, HashSet};

fn common(data: &str, blinks: i64) -> anyhow::Result<i64> {
    let mut numbers = data.split_whitespace().map(|s| (s.parse::<i64>().unwrap_or_default(), 1)).collect::<HashMap<_, _>>();
    for _ in 0..blinks {
        let current_numbers = numbers.clone();
        numbers.clear();
        for (number, count) in current_numbers {
            let new_numbers = match number.to_string().as_str() {
                "0" => vec![1],
                value if value.to_string().len() % 2 == 0 => {
                    let value = value.split_at(value.len()/ 2);
                    let left = value.0.parse::<i64>().unwrap();
                    let value = value.1.trim_start_matches("0");
                    let value = if value.is_empty() {
                        "0"
                    } else {
                        value
                    };
                    let right = value.parse::<i64>().unwrap();
                    vec![left, right]
                }
                value => vec![value.parse::<i64>().unwrap() * 2024]
            };
            for new_number in new_numbers {
                numbers.entry(new_number).and_modify(|v| *v += count).or_insert(count);
            }
        }
        
    }
    let result = numbers.values().sum();
    Ok(result)
}


fn first(data: &str) -> anyhow::Result<i64> {
    common(data, 25)
}

fn second(data: &str) -> anyhow::Result<i64> {
    common(data, 75)
}

#[cfg(test)]
mod tests {
    use crate::data::data;
    use super::*;

    const DAY: i8 = 11;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 55312);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 197157);
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
        assert_eq!(result, 234430066982597);
    }
}
