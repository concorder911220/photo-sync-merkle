use std::collections::HashSet;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: String,
    content: String,
    is_copied: bool,
    height: usize,
    leave_count: usize,
}

impl Node {
    pub fn new(
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        value: String,
        content: String,
        is_copied: bool,
        height: usize,
    ) -> Self {
        let leave_count = match (&left, &right) {
            (Some(l), Some(r)) => l.leave_count + r.leave_count,
            (Some(l), None) | (None, Some(l)) => l.leave_count,
            _ => 1,
        };

        Node {
            left,
            right,
            value,
            content,
            is_copied,
            height,
            leave_count,
        }
    }

    pub fn hash(value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value);
        format!("{:x}", hasher.finalize())
    }

    pub fn copy(&self) -> Node {
        Node {
            left: self.left.clone(),
            right: self.right.clone(),
            value: self.value.clone(),
            content: self.content.clone(),
            is_copied: true,
            height: self.height,
            leave_count: self.leave_count,
        }
    }
}

pub struct MerkleTree {
    pub root: Option<Node>,
    pub arr: Vec<String>,
}

impl MerkleTree {
    pub fn new(values: Vec<String>) -> Self {
        let mut sorted_values = values;
        sorted_values.sort();
        let root = MerkleTree::build_tree(&sorted_values);
        MerkleTree {
            root,
            arr: sorted_values,
        }
    }

    fn build_tree(values: &[String]) -> Option<Node> {
        let mut leaves: Vec<Node> = values
            .iter()
            .map(|v| Node::new(None, None, Node::hash(v), v.clone(), false, 0))
            .collect();

        if leaves.len() % 2 == 1 {
            if let Some(last) = leaves.last().cloned() {
                leaves.push(last.copy());
            }
        }

        MerkleTree::build_tree_rec(leaves)
    }

    fn build_tree_rec(mut nodes: Vec<Node>) -> Option<Node> {
        if nodes.len() % 2 == 1 {
            if let Some(last) = nodes.last().cloned() {
                nodes.push(last.copy());
            }
        }

        if nodes.len() == 2 {
            let left = Box::new(nodes[0].clone());
            let right = Box::new(nodes[1].clone());
            return Some(Node::new(
                Some(left.clone()),
                Some(right.clone()),
                Node::hash(&(left.value.clone() + &right.value)),
                left.content.clone() + "+" + &right.content,
                false,
                left.height + 1,
            ));
        }

        let mid = nodes.len() / 2;
        let left = MerkleTree::build_tree_rec(nodes[..mid].to_vec());
        let right = MerkleTree::build_tree_rec(nodes[mid..].to_vec());

        if let (Some(l), Some(r)) = (left, right) {
            Some(Node::new(
                Some(Box::new(l.clone())),
                Some(Box::new(r.clone())),
                Node::hash(&(l.value.clone() + &r.value)),
                l.content + "+" + &r.content,
                false,
                l.height + 1,
            ))
        } else {
            None
        }
    }

    pub fn sync(&mut self, other: &MerkleTree) {
        if let (Some(my_root), Some(other_root)) = (&self.root, &other.root) {
            let min_height = std::cmp::min(my_root.height, other_root.height);

            let my_leftmost = self.get_leftmost_node_with_height(my_root, min_height);
            let other_leftmost = self.get_leftmost_node_with_height(other_root, min_height);

            let same_val_count = MerkleTree::get_same_val_count(my_leftmost, other_leftmost);

            println!("Same val count: {:?}", same_val_count);
            let common_arr = self.arr[..same_val_count].to_vec();
            let my_diff_arr = self.arr[same_val_count..].to_vec();
            let other_diff_arr = other.arr[same_val_count..].to_vec();

            let merged_arr = MerkleTree::merge_sorted_arrays(&my_diff_arr, &other_diff_arr);
            self.arr = [common_arr, merged_arr].concat();
            self.root = MerkleTree::build_tree(&self.arr);
        }
    }

    fn get_leftmost_node_with_height<'a>(
        &'a self,
        node: &'a Node,
        height: usize,
    ) -> Option<&'a Node> {
        if node.height == height {
            return Some(node);
        }
        if let Some(ref left) = node.left {
            return self.get_leftmost_node_with_height(left, height);
        }
        None
    }

    pub fn get_same_val_count(node_a: Option<&Node>, node_b: Option<&Node>) -> usize {
        let mut heights = Vec::new();
        Self::get_height_for_same_val_rec(node_a, node_b, &mut heights);
    
        heights.iter().sum()
    }

    fn get_height_for_same_val_rec(
        node_a: Option<&Node>,
        node_b: Option<&Node>,
        heights: &mut Vec<usize>,
    ) {
        // Base case: if either node is None, return
        if node_a.is_none() || node_b.is_none() {
            return;
        }

        let node_a = node_a.unwrap();
        let node_b = node_b.unwrap();

        // If values are equal, push the leave_count
        if node_a.value == node_b.value {
            heights.push(node_a.leave_count);
            return;
        }

        // Recursively check left and right children
        Self::get_height_for_same_val_rec(node_a.left.as_ref().map(|n| &**n), node_b.left.as_ref().map(|n| &**n), heights);
        Self::get_height_for_same_val_rec(node_a.right.as_ref().map(|n| &**n), node_b.right.as_ref().map(|n| &**n), heights);
    }
    
    fn merge_sorted_arrays(arr1: &[String], arr2: &[String]) -> Vec<String> {
        let mut merged: Vec<String> = Vec::new();
        let mut i = 0;
        let mut j = 0;
    
        while i < arr1.len() && j < arr2.len() {
            if arr1[i] <= arr2[j] {
                merged.push(arr1[i].clone());
                i += 1;
            } else {
                merged.push(arr2[j].clone());
                j += 1;
            }
        }
    
        merged.extend_from_slice(&arr1[i..]);
        merged.extend_from_slice(&arr2[j..]);
    
        let mut unique = Vec::new();
        let mut seen = HashSet::new();
        for value in merged {
            if seen.insert(value.clone()) {
                unique.push(value);
            }
        }
        unique
    }
}
