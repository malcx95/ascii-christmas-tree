extern crate nalgebra as na;
const NUM_TWIGS_ON_BRANCH: i32 = 6;
const NUM_BRANCHES_PER_LEVEL: i32 = 6;

pub struct TreeNode {
    
    branches: Vec<TreeNode>,

    depth: i32,

    pos: na::Vector3<f32>,

}

impl TreeNode {
    pub fn new(depth: i32, pos: na::Vector3<f32>) -> TreeNode {
        TreeNode {
            branches: Vec::new(),
            depth: depth,
            pos: pos,
        }
    }

    fn build(&mut self) {
        for (i = 0; i < NUM_BRANCHES_PER_LEVEL; i++) {
            for (j = 0; j < NUM_TWIGS_ON_BRANCH; j++) {
                for (k = 0; k < NUM_BRANCHES_PER_LEVEL; k++) {
                    
                }
            }
        }
    }
}

