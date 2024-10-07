use num::{Bounded, Integer};
use std::ops::{Add, Sub};

pub struct Node<T>
where
    T: Integer + Bounded + Copy + Add<Output = T> + Sub<Output = T>,
{
    key: T,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl<T> Node<T>
where
    T: Integer + Bounded + Copy + Add<Output = T> + Sub<Output = T>,
{
    fn new(key: T) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree<T>
where
    T: Integer + Bounded + Copy + Add<Output = T> + Sub<Output = T>,
{
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: Integer + Bounded + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn with_root(key: T) -> Self {
        Self {
            nodes: vec![Node::<T>::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: T, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    pub fn max(&self) -> T {
        if let Some(id) = self.rec_max(Some(0)) {
            return self.nodes[id].key;
        }
        T::min_value()
    }

    fn rec_max(&self, node_id: Option<usize>) -> Option<usize> {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let mut result: usize = id;
            let max_l = self.rec_max(node.id_left);
            let max_r = self.rec_max(node.id_right);
            if let Some(id_l) = max_l {
                if self.nodes[result].key < self.nodes[id_l].key {
                    result = id_l;
                }
            }
            if let Some(id_r) = max_r {
                if self.nodes[result].key < self.nodes[id_r].key {
                    result = id_r;
                }
            }
            return Some(result);
        }
        None
    }

    pub fn min(&self) -> T {
        if let Some(id) = self.rec_min(Some(0)) {
            return self.nodes[id].key;
        }
        T::max_value()
    }

    fn rec_min(&self, node_id: Option<usize>) -> Option<usize> {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let mut result: usize = id;
            let min_l = self.rec_min(node.id_left);
            let min_r = self.rec_min(node.id_right);
            if let Some(id_l) = min_l {
                if self.nodes[result].key > self.nodes[id_l].key {
                    result = id_l;
                }
            }
            if let Some(id_r) = min_r {
                if self.nodes[result].key > self.nodes[min_r.unwrap()].key {
                    result = id_r;
                }
            }
            return Some(result);
        }
        None
    }

    pub fn check_bst(&self) -> bool {
        let (result, _, _) = self.rec_check_bst(Some(0));
        result
    }

    fn rec_check_bst(&self, node_id: Option<usize>) -> (bool, T, T) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (bst_l, min_l, max_l) = self.rec_check_bst(node.id_left);
            let (bst_r, min_r, max_r) = self.rec_check_bst(node.id_right);
            let result: bool = bst_l && bst_r && node.key >= max_l && node.key < min_r;
            return (
                result,
                node.key.min(min_l).min(min_r),
                node.key.max(max_l).max(max_r),
            );
        }
        (true, T::min_value(), T::max_value())
    }

    pub fn check_balance(&self) -> bool {
        let (result, _) = self.rec_check_balance(Some(0));
        result
    }

    fn rec_check_balance(&self, node_id: Option<usize>) -> (bool, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (l, l_h) = self.rec_check_balance(node.id_left);
            let (r, r_h) = self.rec_check_balance(node.id_right);
            let counter: u32 = if l_h < r_h { r_h - l_h } else { l_h - r_h };
            return (l && r && counter <= 1, l_h.max(r_h) + 1);
        }
        (true, 0)
    }

    pub fn equals_sum(&self) -> u32 {
        let (result, _) = self.rec_equals_sum(Some(0), T::zero());
        result
    }

    fn rec_equals_sum(&self, node_id: Option<usize>, path_sum: T) -> (u32, T) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (lcount, lsum) = self.rec_equals_sum(node.id_left, path_sum + node.key);
            let (rcount, rsum) = self.rec_equals_sum(node.id_right, path_sum + node.key);
            let count = if path_sum == lsum + rsum { 1 } else { 0 };
            return (lcount + rcount + count, lsum + rsum + node.key);
        };
        (0, T::zero())
    }

    pub fn max_path_sum(&self) -> T {
        let (result, _) = self.rec_max_path_sum(Some(0));
        result
    }

    fn rec_max_path_sum(&self, node_id: Option<usize>) -> (T, T) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (lres, lpath) = self.rec_max_path_sum(node.id_left);
            let (rres, rpath) = self.rec_max_path_sum(node.id_right);
            return (
                lres.max(rres).max(lpath + rpath + node.key),
                lpath.max(rpath) + node.key,
            );
        };
        (T::zero(), T::zero())
    }

    pub fn check_max_heap(&self) -> bool {
        let (result, _, _, _) = self.rec_check_max_heap(Some(0));
        result
    }

    fn rec_check_max_heap(&self, node_id: Option<usize>) -> (bool, T, u32, bool) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (lres, lkey, ldepth, lcond) = self.rec_check_max_heap(node.id_left);
            let (rres, rkey, rdepth, rcond) = self.rec_check_max_heap(node.id_right);
            let result;
            let mut cond = lcond && rcond;
            if cond {
                cond = ldepth == rdepth;
                result = ldepth == rdepth || ldepth == rdepth + 1;
            } else {
                result = ldepth == rdepth + 1;
            }
            return (
                result && lres && rres && node.key >= lkey && node.key >= rkey,
                node.key,
                ldepth + 1,
                cond,
            );
        };
        (true, T::min_value(), 0, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.max(), 22);
        assert_eq!(tree.min(), 5);
        assert_eq!(tree.check_bst(), false);
        assert_eq!(tree.check_balance(), true);
        assert_eq!(tree.equals_sum(), 0);
        assert_eq!(tree.max_path_sum(), 64);
        assert_eq!(tree.check_max_heap(), false)
    }

    #[test]
    fn test_signed() {
        let mut tree = Tree::with_root(-10);

        tree.add_node(0, -5, true); // id 1
        tree.add_node(0, -22, false); // id 2

        tree.add_node(1, -7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.max(), 20);
        assert_eq!(tree.min(), -22);
        assert_eq!(tree.check_bst(), false);
        assert_eq!(tree.check_balance(), true);
        assert_eq!(tree.equals_sum(), 0);
        assert_eq!(tree.max_path_sum(), 20);
        assert_eq!(tree.check_max_heap(), false)
    }
}
