use std::mem;

const X: i8 = 1 << 0;
const M: i8 = 1 << 1;
const A: i8 = 1 << 2;
const S: i8 = 1 << 3;
const BUFFER: usize = 4;

fn char_to_i8(c: char) -> i8 {
    match c {
        'X' => X,
        'M' => M,
        'A' => A,
        'S' => S,
        _ => panic!(),
    }
}

fn make_matrix(data: &str) -> Vec<Vec<i8>> {
    let lines = data.split("\n").collect::<Vec<_>>();
    let m = lines.len() + BUFFER * 2;

    let mut matrix = vec![vec![0i8; m]; m];

    for (row, line) in data.split("\n").enumerate() {
        for (col, char) in line.chars().enumerate() {
            matrix[row+BUFFER][col+BUFFER] = char_to_i8(char);
        }
    }
    matrix
}

fn first(data: &str) -> anyhow::Result<i64> {
    let matrix = make_matrix(data);
    let m= BUFFER..matrix.len()-BUFFER;
    let xmasb: i32 = unsafe { mem::transmute([X, M, A, S]) };
    let samxb: i32 = unsafe { mem::transmute([S, A, M, X]) };
    let mut res = 0;
    for i in m.clone() {
        for j in m.clone() {
            let b: i32 = unsafe { mem::transmute([matrix[i][j], matrix[i][j+1], matrix[i][j+2], matrix[i][j+3]])};
            if xmasb == b || samxb == b { res += 1; }
            
            let b: i32 = unsafe { mem::transmute([matrix[i][j], matrix[i+1][j], matrix[i+2][j], matrix[i+3][j]])};
            if xmasb == b || samxb == b { res += 1; }

            let b: i32 = unsafe { mem::transmute([matrix[i][j], matrix[i+1][j+1], matrix[i+2][j+2], matrix[i+3][j+3]]) };
            if xmasb == b || samxb == b { res += 1; }

            let b: i32 = unsafe { mem::transmute([matrix[i][j], matrix[i-1][j+1], matrix[i-2][j+2], matrix[i-3][j+3]]) };
            if xmasb == b || samxb == b { res += 1; }
        }
    }
    Ok(res)
}

fn second(data: &str) -> anyhow::Result<i64> {
    let matrix = make_matrix(data);
    let m= BUFFER..matrix.len()-BUFFER;
    let mut res = 0;
    for i in m.clone() {
        for j in m.clone() {
            if matrix[i][j] == A && matrix[i-1][j-1]|matrix[i+1][j+1] == S|M && matrix[i-1][j+1]|matrix[i+1][j-1] == S|M {
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
