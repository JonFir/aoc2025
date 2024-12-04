use std::mem;

const X: i8 = 1 << 0;
const M: i8 = 1 << 1;
const A: i8 = 1 << 2;
const S: i8 = 1 << 3;

fn char_to_i8(c: char) -> i8 {
    match c {
        'X' => X,
        'M' => M,
        'A' => A,
        'S' => S,
        _ => panic!(),
    }
}

fn first(data: &str) -> anyhow::Result<i64> {
    let mut matrix = Vec::<Vec<i8>>::new();
    for (row, line) in data.split("\n").enumerate() {
        matrix.push(Vec::<i8>::new());
        for char in line.chars() {
            matrix[row].push(char_to_i8(char));
        }
    }
    let n = matrix.len();
    let xmasb: i32 = unsafe { mem::transmute([X, M, A, S]) };
    let samxb: i32 = unsafe { mem::transmute([S, A, M, X]) };
    let mut res = 0;
    let xmas_len = 3;
    for i in 0..n {
        for j in 0..n {
            if j + xmas_len < n {
                let b: i32 = unsafe {
                    unsafe {
                        mem::transmute([
                            matrix[i][j],
                            matrix[i][j + 1],
                            matrix[i][j + 2],
                            matrix[i][j + 3],
                        ])
                    }
                };
                if xmasb ^ b == 0 || samxb ^ b == 0 {
                    res += 1;
                }
            }
            if i + xmas_len < n {
                let b: i32 = unsafe {
                    unsafe {
                        mem::transmute([
                            matrix[i][j],
                            matrix[i + 1][j],
                            matrix[i + 2][j],
                            matrix[i + 3][j],
                        ])
                    }
                };
                if xmasb ^ b == 0 || samxb ^ b == 0 {
                    res += 1;
                }
            }

            if i + xmas_len < n && j + xmas_len < n {
                let b: i32 = unsafe {
                    unsafe {
                        mem::transmute([
                            matrix[i][j],
                            matrix[i + 1][j + 1],
                            matrix[i + 2][j + 2],
                            matrix[i + 3][j + 3],
                        ])
                    }
                };
                if xmasb ^ b == 0 || samxb ^ b == 0 {
                    res += 1;
                }
            }

            if i as i64 - xmas_len as i64 >= 0 && j + xmas_len < n {
                let b: i32 = unsafe {
                    unsafe {
                        mem::transmute([
                            matrix[i][j],
                            matrix[i - 1][j + 1],
                            matrix[i - 2][j + 2],
                            matrix[i - 3][j + 3],
                        ])
                    }
                };
                if xmasb ^ b == 0 || samxb ^ b == 0 {
                    res += 1;
                }
            }
        }
    }
    Ok(res)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let mut matrix = Vec::<Vec<i8>>::new();
    for (row, line) in data.split("\n").enumerate() {
        matrix.push(Vec::<i8>::new());
        for char in line.chars() {
            matrix[row].push(char_to_i8(char));
        }
    }
    let n = matrix.len();
    let mut res = 0;
    for i in 1..n-1 {
        for j in 1..n-1 {
            if (matrix[i][j] == A) && matrix[i-1][j-1]|matrix[i+1][j+1] == S|M && matrix[i-1][j+1]|matrix[i+1][j-1] == S|M {
                res += 1; 
            }
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::data::data;

    use super::*;

    const DAY: i8 = 4;

    #[test]
    fn t1() {
        let data = data(DAY, "t1");
        let result = first(&data).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn i1() {
        let data = data(DAY, "i1");
        let result = first(&data).unwrap();
        assert_eq!(result, 2462);
    }

    #[test]
    fn t2() {
        let data = data(DAY, "t2");
        let result = second(&data).unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn i2() {
        let data = data(DAY, "i2");
        let result = second(&data).unwrap();
        assert_eq!(result, 1877);
    }

}
