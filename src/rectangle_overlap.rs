pub struct Solution {}

/// 836. 矩形重叠
/// https://leetcode-cn.com/problems/rectangle-overlap/
/// 检查区域: 如果两个矩形重叠，那么它们重叠的区域一定也是一个矩形，那么这代表了两个矩形与 xx 轴平行的边（水平边）投影到 xx 轴上时会有交集，与 yy 轴平行的边（竖直边）投影到 yy 轴上时也会有交集。因此，我们可以将问题看作一维线段是否有交集的问题。
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        rec1[2].min(rec2[2]) > rec1[0].max(rec2[0]) && rec1[3].min(rec2[3]) > rec1[1].max(rec2[1])
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
        false
    );
}
