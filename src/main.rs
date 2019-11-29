mod best_time_to_buy_and_sell_stock;
use best_time_to_buy_and_sell_stock::Solution;

fn main() {
    let data = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(data));
}
