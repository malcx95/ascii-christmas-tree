extern crate nalgebra as na;
extern crate rand;
const NUM_TWIGS_ON_BRANCH: i32 = 6;
const NUM_BRANCHES_PER_LEVEL: i32 = 6;

const ANGLE: f32 = 0.1;

use vertex::Vertex;
use rand::distributions::{IndependentSample, Range};

#[derive(Clone)]
pub struct TreeNode {
    pub branches: Vec<TreeNode>,
    depth: i32,
    pub pos: na::Vector3<f32>,
    pub dir: na::Vector3<f32>,
}

impl TreeNode {
    pub fn new(depth: i32, pos: na::Vector3<f32>, dir: na::Vector3<f32>) -> TreeNode {
        TreeNode {
            branches: Vec::new(),
            depth: depth,
            pos: pos,
            dir: dir,
        }
    }

    fn build_help(depth: i32, pos: na::Vector3<f32>, direction: na::Vector3<f32>) -> TreeNode {
        let mut branch = TreeNode::new(depth, pos, direction);
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
                left *= -1;
                let new_dir = na::rotate(&t, &v);
                branch.branches.push(TreeNode::build_help(depth - 1, v, new_dir));
            }
            return branch;
        }
    }

    pub fn build(&mut self) {
        let v = na::Vector3::new(1.0, 1.0, -1.0);
        for i in 0..NUM_BRANCHES_PER_LEVEL {
            let t = na::Rotation3::new(
                na::Vector3::new(0.0f32, 0.0, (i as f32) *
                                 (2.0 * 3.14)/(NUM_BRANCHES_PER_LEVEL as f32))
            );
            let new_dir = na::rotate(&t, &v);
            self.branches.push(TreeNode::build_help(self.depth, self.pos, new_dir));
        }
    }
}

#[allow(dead_code)]
pub fn print_tree(tree: &TreeNode) {
    use std::collections::VecDeque;
    let mut last_depth = None;
    let mut tree_queue = VecDeque::new();
    tree_queue.push_back(tree.clone());

    while let Some(top) = tree_queue.pop_front() {
        if last_depth != Some(top.depth) {
            println!("\nAt depth {}", top.depth);
        }
        last_depth = Some(top.depth);

        println!("Node {} pointing {}", top.pos, top.dir);
        for branch in top.branches {
            tree_queue.push_back(branch.clone());
        }
    }
}

pub fn make_triangles(tree: &TreeNode, scaling: f32, offset: f32, rot: f32) -> Vec<Vertex> {
    use std::collections::VecDeque;
    let mut tree_queue = VecDeque::new();
    tree_queue.push_back(tree.clone());

    let between = Range::new(0.0, 0.2);
    let mut rng = rand::thread_rng();

    let mut triangles = Vec::new();

    while let Some(top) = tree_queue.pop_front() {
        let base_pos = top.pos;
        let tip_pos = base_pos + top.dir;
        let green = [0.0 + between.ind_sample(&mut rng),
                     0.3 + between.ind_sample(&mut rng),
                     0.0 + between.ind_sample(&mut rng)];

        let base_x = rot.cos() * base_pos.x + rot.sin() * base_pos.y;
        let tip_x = rot.cos() * tip_pos.x + rot.sin() * tip_pos.y;

        let a = [base_x * scaling / 120.0 - 0.03,
                 base_pos.z * scaling / 100.0 + offset];
        let b = [tip_x * scaling / 120.0,
                 tip_pos.z * scaling / 100.0 + offset];
        let c = [base_x * scaling / 120.0 + 0.03,
                 base_pos.z * scaling  / 100.0 + offset];

        triangles.push(Vertex { pos: a, color: green });
        triangles.push(Vertex { pos: b, color: green });
        triangles.push(Vertex { pos: c, color: green });

        for branch in top.branches {
            tree_queue.push_back(branch.clone());
        }
    }

    triangles
}
