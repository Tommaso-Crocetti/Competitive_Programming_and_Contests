pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
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

    pub fn max(&self) -> u32 {
        if let Some(id) = self.rec_max(Some(0)) {
            return self.nodes[id].key;
        }
        0
    }

    fn rec_max (&self, node_id: Option<usize>) -> Option<usize> {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let mut result: usize = 0;
            let max_l = self.rec_max(node.id_left);
            if !max_l.is_none() {
                result = max_l.unwrap();
            }
            let max_r = self.rec_max(node.id_right);
            if !max_r.is_none() && self.nodes[result].key < self.nodes[max_r.unwrap()].key {
                result = max_r.unwrap();
            }
            if self.nodes[result].key < self.nodes[id].key {
                result = id;
            }
            return Some(result);
        }
        None
    }

    pub fn min(&self) -> u32 {
        if let Some(id) = self.rec_min(Some(0)) {
            return self.nodes[id].key;
        }
        0
    }

    fn rec_min (&self, node_id: Option<usize>) -> Option<usize> {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let mut result: usize = 0;
            let min_l = self.rec_min(node.id_left);
            if !min_l.is_none() {
                result = min_l.unwrap();
            }
            let min_r = self.rec_min(node.id_right);
            if !min_r.is_none() && self.nodes[result].key > self.nodes[min_r.unwrap()].key {
                result = min_r.unwrap();
            }
            if self.nodes[result].key > self.nodes[id].key {
                result = id;
            }
            return Some(result);
        }
        None
    }

    pub fn check_bst(&self) -> bool {
        let (result, _, _) = self.rec_check_bst(Some(0));
        result
    }

    fn rec_check_bst(&self, node_id: Option<usize>) -> (bool, u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (bst_l, min_l, max_l) = self.rec_check_bst(node.id_left);
            let (bst_r, min_r, max_r) = self.rec_check_bst(node.id_right);
            let result: bool = bst_l && bst_r && node.key >= max_l && node.key < min_r;
            return (result, node.key.min(min_l).min(min_r), node.key.max(max_l).max(max_r));
        }
        (true, 0, u32::MAX)
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
        let (result, _) = self.rec_equals_sum(Some(0), 0);
        result
    }

    fn rec_equals_sum(&self, node_id: Option<usize>, path_sum: u32) -> (u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (lcount, lsum) = self.rec_equals_sum(node.id_left, path_sum + node.key);
            let (rcount, rsum) = self.rec_equals_sum(node.id_right, path_sum + node.key);
            let count = if path_sum == lsum + rsum { 1 } else { 0 };
            return (lcount + rcount + count, lsum + rsum + node.key);
        };
        (0, 0)
    }

    pub fn max_path_sum(&self) -> u32 {
        let (result, _) = self.rec_max_path_sum(Some(0));
        result
    }

    fn rec_max_path_sum(&self, node_id: Option<usize>) -> (u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let (lres, lpath) = self.rec_max_path_sum(node.id_left);
            let (rres, rpath) = self.rec_max_path_sum(node.id_right);
            return (lres.max(rres).max(lpath + rpath + node.key), lpath.max(rpath) + node.key);
        };
        (0, 0)
    }

    pub fn check_max_heap(&self) -> bool {
        let (result, _, _, _) = self.rec_check_max_heap(Some(0));
        result
    }

    fn rec_check_max_heap(&self, node_id: Option<usize>) -> (bool, u32, u32, bool) {
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
            return (result && lres && rres && node.key >= lkey && node.key >= rkey, node.key, ldepth + 1, cond);
        };
        (true, 0, 0, true)
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
}
