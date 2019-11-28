mod triangle;
use triangle::Solution;

fn main() {
    let data = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    println!("{}", Solution::minimum_total(data));
}
