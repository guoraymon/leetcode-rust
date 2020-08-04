/// 课程表
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/course-schedule/)
///
/// ```text
/// 你这个学期必须选修 numCourse 门课程，记为 0 到 numCourse-1 。
/// 在选修某些课程之前需要一些先修课程。 例如，想要学习课程 0 ，你需要先完成课程 1 ，我们用一个匹配来表示他们：[0,1]
/// 给定课程总量以及它们的先决条件，请你判断是否可能完成所有课程的学习？
///
/// 示例 1:
/// 输入: 2, [[1,0]]
/// 输出: true
/// 解释: 总共有 2 门课程。学习课程 1 之前，你需要完成课程 0。所以这是可能的。
///
/// 示例 2:
/// 输入: 2, [[1,0],[0,1]]
/// 输出: false
/// 解释: 总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0；并且学习课程 0 之前，你还应先完成课程 1。这是不可能的。
///
/// 提示：
/// 输入的先决条件是由 边缘列表 表示的图形，而不是 邻接矩阵 。详情请参见图的表示法。
/// 你可以假定输入的先决条件中没有重复的边。
/// 1 <= numCourses <= 10^5
/// ```
pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut edges = vec![vec![]; num_courses as usize]; // 已访问的顶点：已学过的课
        let mut in_degrees = vec![0; num_courses as usize]; // 顶点的入度：

        // 建立边和入度表
        for prerequisite in prerequisites {
            edges[prerequisite[1] as usize].push(prerequisite[0]);
            in_degrees[prerequisite[0] as usize] += 1;
        }

        // 0入度顶点进入队列
        let mut queue: Vec<i32> = (0..num_courses)
            .filter(|&x| in_degrees[x as usize] == 0)
            .collect();

        let mut visited = 0;
        while !queue.is_empty() {
            visited += 1;

            let q = queue.remove(0);
            for i in edges[q as usize].clone() {
                in_degrees[i as usize] -= 1;

                // 0入度顶点进入队列
                if in_degrees[i as usize] == 0 {
                    queue.push(i);
                }
            }
        }

        visited == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dp() {
        assert_eq!(Solution::can_finish(2, vec!(vec![1, 0])), true);
        assert_eq!(Solution::can_finish(2, vec!(vec![1, 0], vec![0, 1])), false);
        assert_eq!(Solution::can_finish(3, vec!(vec![2, 0], vec![2, 1])), true);
    }
}
