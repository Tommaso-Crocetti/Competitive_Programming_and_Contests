pub fn holiday_planning(n: usize, d: usize, iters: Vec<Vec<u32>>) -> u32 {
    let mut prefixes = Vec::with_capacity(n);
    for iter in iters {
        let mut pref = vec![0; d + 1];
        pref[0] = 0;
        for i in 1..d + 1 {
            pref[i] = pref[i - 1] + iter[i - 1];
        }
        prefixes.push(pref);
    }
    let mut dp = vec![vec![0; d + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..d + 1 {
            for k in 0..j + 1 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - k] + prefixes[i - 1][k]);
            }
        }
    }
    dp[n][d]
}

pub struct Node {
    key: u32,
    value: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
    id_father: Option<usize>,
}

impl Node {
    fn new(key: u32, value: u32, id_father: Option<usize>) -> Self {
        Self {
            key,
            value,
            id_left: None,
            id_right: None,
            id_father,
        }
    }
}

pub struct BSTree {
    nodes: Vec<Node>,
}

impl BSTree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key, 1, None)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: u32, value: u32, is_left: bool) -> usize {
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
        self.nodes.push(Node::new(key, value, Some(parent_id)));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    pub fn insert(&mut self, key: u32, value: u32) {
        self.rec_insert(Some(0), key, value);
    }

    fn rec_insert(&mut self, node_id: Option<usize>, key: u32, value: u32) {
        if let Some(id) = node_id {
            if key <= self.nodes[id].key {
                if self.nodes[id].id_left.is_none() {
                    self.add_node(id, key, value, true);
                } else {
                    self.rec_insert(self.nodes[id].id_left, key, value);
                }
            } else if self.nodes[id].id_right.is_none() {
                self.add_node(id, key, value, false);
            } else {
                self.rec_insert(self.nodes[id].id_right, key, value);
            }
        }
    }

    pub fn predecessor(&self, key: u32) -> Option<usize> {
        self.rec_predecessor(Some(0), key, None)
    }

    fn rec_predecessor(
        &self,
        node_id: Option<usize>,
        key: u32,
        current_pred: Option<usize>,
    ) -> Option<usize> {
        if let Some(id) = node_id {
            if key > self.nodes[id].key {
                return self.rec_predecessor(self.nodes[id].id_right, key, Some(id));
            } else {
                return self.rec_predecessor(self.nodes[id].id_left, key, current_pred);
            }
        }
        current_pred
    }

    pub fn get_max_value(&self, node_id: Option<usize>, key: u32) -> u32 {
        if let Some(_id) = node_id {
            return self.rec_get_max_value(node_id, key, 0);
        }
        0
    }

    fn rec_get_max_value(&self, node_id: Option<usize>, key: u32, max: u32) -> u32 {
        if let Some(id) = node_id {
            if key > self.nodes[id].key {
                return self.rec_get_max_value(
                    self.nodes[id].id_father,
                    key,
                    max.max(self.nodes[id].value),
                );
            }
            return self.rec_get_max_value(self.nodes[id].id_father, key, max);
        }
        max
    }

    pub fn update_max_value(&mut self) {
        let key = self.nodes[self.nodes.len() - 1].key;
        let value = self.nodes[self.nodes.len() - 1].value;
        self.rec_update_max_value(Some(self.nodes.len() - 1), key, value);
    }

    fn rec_update_max_value(&mut self, node_id: Option<usize>, key: u32, value: u32) {
        if let Some(id) = node_id {
            if key <= self.nodes[id].key {
                self.nodes[id].value = (self.nodes[id].value).max(value);
            }
            self.rec_update_max_value(self.nodes[id].id_father, key, value)
        }
    }
}

pub fn design_a_course(_n: usize, mut topics: Vec<Vec<u32>>) -> u32 {
    topics.sort_by(|a, b| a[1].cmp(&b[1]));
    topics.dedup_by(|a, b| a[1] == b[1]);
    let mut lis = vec![1; topics.len()];
    let mut result = 0;
    for i in 0..topics.len() {
        for j in 0..i {
            if topics[i][0] > topics[j][0] {
                lis[i] = lis[i].max(1 + lis[j])
            }
        }
        result = result.max(lis[i])
    }
    result
}

pub fn design_a_course_v2(_n: usize, mut topics: Vec<Vec<u32>>) -> u32 {
    topics.sort_by(|a, b| a[1].cmp(&b[1]).then_with(|| b[0].cmp(&a[0])));
    let mut tree = BSTree::with_root(topics[0][0]);
    let mut result = 1;
    for topic in &topics[1..] {
        let id_pred = tree.predecessor(topic[0]);
        let max_value = tree.get_max_value(id_pred, topic[0]);
        tree.insert(topic[0], max_value + 1);
        result = result.max(max_value + 1);
        tree.update_max_value();
    }
    result
}
