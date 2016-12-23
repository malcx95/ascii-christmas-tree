extern crate nalgebra as na;
const NUM_TWIGS_ON_BRANCH: i32 = 6;
const NUM_BRANCHES_PER_LEVEL: i32 = 6;

const ANGLE: f32 = 0.1;

pub struct TreeNode {

    branches: Vec<TreeNode>,

    depth: i32,

    pub pos: na::Vector3<f32>,

}

impl TreeNode {
    pub fn new(depth: i32, pos: na::Vector3<f32>) -> TreeNode {
        TreeNode {
            branches: Vec::new(),
            depth: depth,
            pos: pos,
        }
    }

    fn build_help(depth: i32, pos: na::Vector3<f32>, direction: na::Vector3<f32>) -> TreeNode {
        let mut branch = TreeNode::new(depth, pos);
        if depth == 0 {
            return branch;
        } else {
            let mut left = 1;
            for i in 0..NUM_TWIGS_ON_BRANCH {
                let mut v = pos + direction;
                for _ in 0..i {
                    v = v + direction;
                }
                let t = na::Rotation3::new(
                    na::Vector3::new(0.0f32, 0.0, (left as f32) * ANGLE * 3.14)
                );
                if left == -1 {
                    left = 1;
                } else {
                    left = -1;
                }
                let new_dir = na::rotate(&t, &v);
                branch.branches.push(TreeNode::build_help(depth - 1, v, new_dir));
            }
            return branch;
        }
    }

    pub fn build(&mut self) {
        let v = na::Vector3::new(1.0, 1.0, -1.0);
        for i in 0..NUM_BRANCHES_PER_LEVEL {
            let t = na::Rotation3::new(na::Vector3::new(0.0f32, 0.0, (i as f32) *
                                                        (2.0 * 3.14)/(NUM_BRANCHES_PER_LEVEL as f32)));
            let new_dir = na::rotate(&t, &v);
            self.branches.push(TreeNode::build_help(self.depth, self.pos, new_dir));
        }
    }
}

