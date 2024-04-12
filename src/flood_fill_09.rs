// link: https://leetcode.com/problems/flood-fill

use crate::Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image.clone();
        let row = image.len();
        let column = image[0].len();
        let start_color: i32 = image[sr as usize][sc as usize];
        if start_color == color {
            return image;
        }
        Self::dfs(sr, sc, row, column, start_color, color, &mut image);
        return image;
    }

    fn dfs(
        m: i32,
        n: i32,
        row: usize,
        column: usize,
        start_color: i32,
        color: i32,
        image: &mut Vec<Vec<i32>>,
    ) {
        if m < 0
            || n < 0
            || m >= row as i32
            || n >= column as i32
            || image[m as usize][n as usize] != start_color
        {
            return;
        }
        image[m as usize][n as usize] = color;
        Self::dfs(m - 1, n, row, column, start_color, color, image);
        Self::dfs(m + 1, n, row, column, start_color, color, image);
        Self::dfs(m, n - 1, row, column, start_color, color, image);
        Self::dfs(m, n + 1, row, column, start_color, color, image);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill01() {
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn test_flood_fill02() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }
}
