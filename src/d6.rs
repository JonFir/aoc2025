use std::collections::{HashMap, HashSet};

fn next_direction(direction: i64, m: i64) -> i64 {
    if direction == -m {
        1
    } else if direction == 1 {
        m
    } else if direction == m {
        -1
    } else if direction == -1 {
        -m
    } else {
        panic!()
    }
}

fn first(data: &str) -> anyhow::Result<i64> {
    let lines = data.split("\n").collect::<Vec<_>>();
    let m = lines.len() as i64;
    let mut matrix = data.split("\n").flat_map(|line| line.chars()).collect::<Vec<_>>();
    let mut spteps = 0;
    let mut index = matrix.iter().position(|e| e == &'^').unwrap();
    let mut direction = -m;
    loop {
        let next_index = index as i64 + direction;
        if (next_index >= matrix.len() as i64) || (next_index < 0) || (direction == -1 && next_index % m == m-1) || (direction == 1 && next_index % m == 0) {
            break;
        }
        let next_index = next_index as usize;
        if matrix[next_index] == '#' {
            direction = next_direction(direction, m);
        } else {
            if matrix[index] != 'x' {
                spteps += 1;
                matrix[index] = 'x'
            }
            index = next_index;
        }
    }
    Ok(spteps+1)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let lines = data.split("\n").collect::<Vec<_>>();
    let m = lines.len() as i64;
    let matrix = data.split("\n").flat_map(|line| line.chars()).collect::<Vec<_>>();
    let index = matrix.iter().position(|e| e == &'^').unwrap();
    let mut variants = 0;
    for (test_index, char) in matrix.iter().enumerate() {
        if char == &'#' || char == &'^' {
            continue;
        }
        let mut matrix = matrix.clone();
        matrix[test_index] = '#';
        let mut index = index;
        let mut direction = -m;
        let mut is_loop = false;
        let mut rotations = HashMap::<usize, HashSet<i64>>::new();
        while !is_loop {
            let next_index = index as i64 + direction;
            if (next_index >= matrix.len() as i64) || (next_index < 0) || (direction == -1 && next_index % m == m-1) || (direction == 1 && next_index % m == 0) {
                break;
            }
            let next_index = next_index as usize;
            if matrix[next_index] == '#' {
                if let Some(p) = rotations.get(&index) {
                    is_loop = p.contains(&direction);
                } else {
                    rotations.entry(index).or_default().insert(direction);
                }
                direction = next_direction(direction, m);
            } else {
                index = next_index;
            }
        }
        if is_loop {
            variants += 1;
        }
    }
    Ok(variants)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 6;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 41);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 4982);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 1663);
    }
}
