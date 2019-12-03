mod house_robber;
use chrono::prelude::*;
use house_robber::Solution;

fn main() {
    let start = Local::now().timestamp_nanos();
    Solution::test();
    let end = Local::now().timestamp_nanos();

    println!(
        "done! start: {:?}, end: {:?}, duration: {:?}",
        start,
        end,
        end - start
    );
}
