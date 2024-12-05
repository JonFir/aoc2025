use std::collections::{HashMap, HashSet};

fn first(data: &str) -> anyhow::Result<i64> {
    let mut rules = HashMap::<i64, HashSet<i64>>::new();
    let mut mode = 0;
    let mut result = 0;
    for line in data.split("\n") {
        if line.is_empty() {
            mode = 1;
            continue;
        }
        if mode == 0 {
            let mut numbers = line.split("|").map(|c| c.parse::<i64>().unwrap());
            let number= numbers.next().unwrap_or_default();
            let rule= numbers.next().unwrap_or_default();
            rules.entry(number).or_default().insert(rule);
        } else {
            let mut oldest = HashSet::<i64>::new();
            let mut ok: bool = true;
            let line = line.split(",").flat_map(|c| c.parse::<i64>()).collect::<Vec<_>>();
            for number in line.iter() {
                if rules.get(number).map(|rules| oldest.intersection(rules).next().is_none() ).unwrap_or(true) {
                    oldest.insert(*number);
                } else {
                    ok = false;
                    break;
                }
            }
            if ok {
                result += line[line.len() / 2];
            }
        }
    }

    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let mut rules = HashMap::<i64, HashSet<i64>>::new();
    let mut mode = 0;
    let mut result = 0;
    for line in data.split("\n") {
        if line.is_empty() {
            mode = 1;
            continue;
        }
        if mode == 0 {
            let mut numbers = line.split("|").map(|c| c.parse::<i64>().unwrap());
            let number= numbers.next().unwrap_or_default();
            let rule= numbers.next().unwrap_or_default();
            rules.entry(number).or_default().insert(rule);
        } else {
            let mut oldest = HashSet::<i64>::new();
            let mut line = line.split(",").flat_map(|c| c.parse::<i64>()).collect::<Vec<_>>();
            let mut index = 0;
            let mut ok = false;
            while index < line.len() {
                let number = line[index];
                let intersect= rules.get(&number).map(|rules| oldest.intersection(rules).collect::<Vec<_>>() ).unwrap_or_default();
                if intersect.is_empty() {
                    oldest.insert(number);
                    index += 1;
                } else {
                    let number = line.remove(index);
                    let number_index = line.iter().position(|e|intersect.contains(&e)).unwrap_or_default();
                    line.insert(number_index, number);
                    index = 0;
                    oldest.clear();
                    ok = true;
                }
            }
            if ok {
                result += line[line.len() / 2];
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 5;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 4905);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 6204);
    }
}
