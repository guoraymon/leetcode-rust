mod house_robber;
use house_robber::Solution;

fn main() {
    let data = vec![0, 1];
    println!("{}", Solution::rob(data));
}
