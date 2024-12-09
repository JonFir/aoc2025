use std::collections::{HashMap, HashSet};
use std::{char, result};
use std::str::FromStr;

fn first(data: &str) -> anyhow::Result<i64> {
    let data = data.lines().collect::<Vec<_>>();
    let m = data.len() as i64;
    let symbols = data
        .iter()
        .flat_map(|line|line.chars())
        .enumerate()
        .filter(|(_, char)| char != &'.');
    let mut chars = HashMap::<char, Vec<i64>>::new();
    for (index, char) in symbols {
        chars.entry(char).or_default().push(index as i64);
    }
    let mut all_points = HashSet::<i64>::new();
    
    for positions in chars.values() {
        for i in 0..positions.len() - 1 {
            let ir = positions[i] / m;
            let ic = positions[i] % m;
            for j in i+1..positions.len() {    
                let jr = positions[j] / m;
                let jc = positions[j] % m;

                let dr = jr - ir;
                let dc = jc - ic;

                let t1r = ir-dr;
                let t1c = ic-dc;

                if t1r >= 0 && t1r < m && t1c >= 0 && t1c < m {
                    let t1 = t1r * m + t1c;
                    all_points.insert(t1);
                }

                let t2r = jr+dr;
                let t2c = jc+dc;

                if t2r >= 0 && t2r < m && t2c >= 0 && t2c < m {
                    let t2 = t2r * m + t2c;
                    all_points.insert(t2);
                }
            }
        }
    }
    let result = all_points.len() as i64;
    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let data = data.lines().collect::<Vec<_>>();
    let m = data.len() as i64;
    let symbols = data
        .iter()
        .flat_map(|line|line.chars())
        .enumerate()
        .filter(|(_, char)| char != &'.');
    let mut chars = HashMap::<char, Vec<i64>>::new();
    for (index, char) in symbols {
        chars.entry(char).or_default().push(index as i64);
    }
    let mut all_points = HashSet::<i64>::new();
    
    for positions in chars.values() {
        for i in 0..positions.len() - 1 {
            let ir = positions[i] / m;
            let ic = positions[i] % m;
            all_points.insert(positions[i]);
            for j in i+1..positions.len() {    
                let jr = positions[j] / m;
                let jc = positions[j] % m;
                all_points.insert(positions[j]);

                let dr = jr - ir;
                let dc = jc - ic;

                let mut t1r = ir;
                let mut t1c = ic;
                loop {
                    t1r -= dr;
                    t1c -= dc;
                    if t1r >= 0 && t1r < m && t1c >= 0 && t1c < m {
                        let t1 = t1r * m + t1c;
                        all_points.insert(t1);
                    } else {
                        break;
                    }
                }

                let mut t2r = jr;
                let mut t2c = jc;
                loop {
                    t2r += dr;
                    t2c += dc;
                    if t2r >= 0 && t2r < m && t2c >= 0 && t2c < m {
                        let t2 = t2r * m + t2c;
                        all_points.insert(t2);
                    } else {
                        break;
                    }    
                }
                
            }
        }
    }
    let result = all_points.len() as i64;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 8;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 323);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 34);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 1077);
    }
}
