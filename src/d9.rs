use std::collections::{HashMap, HashSet};
use std::mem::swap;
use std::{char, result};
use std::str::FromStr;

fn first(data: &str) -> anyhow::Result<i64> {
    let mut fs = Vec::<i64>::new();
    for (index, number) in data.chars().flat_map(|c| c.to_string().parse::<i64>()).enumerate() {
        let index = index as i64;
        let number = number as usize;
        if index  % 2 == 0 {
            fs.extend(vec![index / 2; number]);
        } else {
            fs.extend(vec![-1; number]);
        }
    }
    let mut head = 0; 
    let mut tail = fs.len() - 1;
    while head < tail {
        if fs[head] != -1 {
            head += 1;
        }
        if fs[tail] == -1 {
            tail -= 1;
        }

        if fs[head] == -1 && fs[tail] != -1 {
            fs.swap(head, tail);
        }
    }
    let result = fs.iter().take_while(|n| n >= &&0).enumerate().map(|(index, value)| index as i64 * value).sum();
    Ok(result)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let mut fs = Vec::<i64>::new();
    for (index, number) in data.chars().flat_map(|c| c.to_string().parse::<i64>()).enumerate() {
        let index = index as i64;
        let number = number as usize;
        if index  % 2 == 0 {
            fs.extend(vec![index / 2; number]);
        } else {
            fs.extend(vec![-1; number]);
        }
    }
    let mut tail = fs.len() - 1;
    while tail > 0 {
        let mut block_size = 0;
        
        while fs[tail] == -1 {
            tail -= 1;
        }
        let tail_number = fs[tail];

        while fs[tail] == tail_number && tail > 0 {
            block_size += 1;
            tail -= 1;
        }

        let mut head = 0;
        
        while head < fs.len() && head <= tail+1 {
            let mut zero_size = 0;
            
            while head < fs.len() && fs[head] != -1 {
                head += 1;
            }
    
            while head < fs.len() && fs[head] == -1 {
                zero_size += 1;
                head += 1;
            }

            if head >= fs.len() || !(head <= tail+1) {
                break;
            }
            if zero_size >= block_size {
                head -= zero_size;
                tail += block_size;
                while block_size != 0 {
                    fs.swap(head, tail);
                    head += 1;
                    tail -= 1;
                    block_size -= 1;
                }
                break;
            }
        }
    }
    let result = fs.iter().enumerate().filter(|(_, value)| **value>0 ).map(|(index, value)| index as i64 * value).sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::data::data;
    use super::*;

    const DAY: i8 = 9;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 1928);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 6_421_128_769_094);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 2858);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 6448168620520);
    }
}
