use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in (0..triangle.len()).rev() {
            for j in (0..triangle[i].len()).rev() {
                let t1 = triangle.get(i+1);
                triangle[i][j] += t1.map(|t1| min(t1[j], t1[j+1])).unwrap_or(0);
            }
        }
        return triangle[0][0];
    }
}
