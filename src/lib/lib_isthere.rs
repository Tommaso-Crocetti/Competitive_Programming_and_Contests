use std::collections::HashMap;

fn get_prefix(arr: &[(u32, u32)]) -> Vec<i32> {
    let n = arr.len();
    let mut prefix: Vec<i32> = vec![0; n];
    for i in arr {
        prefix[i.0 as usize] += 1;
        if ((i.1 + 1) as usize) < n {
            prefix[(i.1 + 1) as usize] -= 1;
        }
    }
    for i in 1..n {
        prefix[i] += prefix[i - 1];
    }
    prefix
}

pub struct Node {
    key: HashMap<i32, u32>,
    left: usize,
    right: usize,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: HashMap<i32, u32>, l: usize, r: usize) -> Self {
        Self {
            key,
            left: l,
            right: r,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Isthere {
    nodes: Vec<Node>,
}

impl Isthere {
    pub fn new(arr: &[(u32, u32)]) -> Self {
        let mut tree = Isthere { nodes: Vec::new() };
        let prefix = get_prefix(arr);
        tree.build(0, arr.len() - 1, &prefix);
        tree
    }

    /// Costruisce ricorsivamente il Segment Tree.
    /// Restituisce l'ID del nodo radice.
    fn build(&mut self, l: usize, r: usize, arr: &[i32]) -> usize {
        if l == r {
            // Nodo foglia: rappresenta un singolo elemento dell'array
            let mut map = HashMap::new();
            map.insert(arr[l], 1);
            let node = Node::new(map, l, r);
            self.nodes.push(node);
            return self.nodes.len() - 1;
        }

        let mid = (l + r) / 2;

        // Costruisce i sottoalberi
        let id_left = self.build(l, mid, arr);
        let id_right = self.build(mid + 1, r, arr);

        // Valore del nodo corrente è la somma (o massimo/minimo) dei figli
        let mut map = HashMap::new();
        for (&k, &v) in &self.nodes[id_left].key {
            *map.entry(k).or_insert(0) += v;
        }
        for (&k, &v) in &self.nodes[id_right].key {
            *map.entry(k).or_insert(0) += v;
        }

        let mut node = Node::new(map, l, r);

        // Collega i figli
        node.id_left = Some(id_left);
        node.id_right = Some(id_right);

        self.nodes.push(node);
        self.nodes.len() - 1 // Restituisce l'ID del nodo creato
    }

    fn query(&self, node_id: usize, i: usize, j: usize, k: i32) -> bool {
        let node = &self.nodes[node_id];

        // Caso base: l'intervallo del nodo non interseca [i, j]
        if node.right < i || node.left > j {
            return false; // Non contribuisce
        }

        // Caso base: l'intervallo del nodo è completamente dentro [i, j]
        if i <= node.left && node.right <= j {
            // Controlla se il valore `k` è presente nel nodo corrente
            return node.key.get(&k).copied().unwrap_or(0) > 0;
        }

        // Caso generale: dividi tra figlio sinistro e destro
        let mut left_result = false;
        let mut right_result = false;

        if let Some(left_id) = node.id_left {
            left_result = self.query(left_id, i, j, k);
        }

        if let Some(right_id) = node.id_right {
            right_result = self.query(right_id, i, j, k);
        }

        left_result || right_result // Ritorna `true` se almeno un figlio restituisce `true`
    }

    pub fn isthere(&self, i: usize, j: usize, k: i32) -> u32 {
        let root_id = self.nodes.len() - 1;
        self.query(root_id, i, j, k) as u32
    }
}
