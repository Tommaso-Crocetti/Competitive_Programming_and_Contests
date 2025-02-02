pub struct Node {
    key: u32,
    left: usize,
    right: usize,
    id_left: Option<usize>,
    id_right: Option<usize>,
    lazy: u32,
}

impl Node {
    fn new(key: u32, l: usize, r: usize) -> Self {
        Self {
            key,
            left: l,
            right: r,
            id_left: None,
            id_right: None,
            lazy: u32::MAX,
        }
    }
}

pub struct Minandmax {
    nodes: Vec<Node>,
}

impl Minandmax {
    pub fn new(arr: &[u32]) -> Self {
        let mut tree = Minandmax { nodes: Vec::new() };
        tree.build(0, arr.len() - 1, arr);
        tree
    }

    /// Costruisce ricorsivamente il Segment Tree.
    /// Restituisce l'ID del nodo radice.
    fn build(&mut self, l: usize, r: usize, arr: &[u32]) -> usize {
        if l == r {
            // Nodo foglia: rappresenta un singolo elemento dell'array
            let node = Node::new(arr[l], l, r);
            self.nodes.push(node);
            return self.nodes.len() - 1;
        }

        let mid = (l + r) / 2;

        // Costruisce i sottoalberi
        let id_left = self.build(l, mid, arr);
        let id_right = self.build(mid + 1, r, arr);

        // Valore del nodo corrente è la somma (o massimo/minimo) dei figli
        let key = self.nodes[id_left].key.max(self.nodes[id_right].key);
        let mut node = Node::new(key, l, r);

        // Collega i figli
        node.id_left = Some(id_left);
        node.id_right = Some(id_right);

        self.nodes.push(node);
        self.nodes.len() - 1 // Restituisce l'ID del nodo creato
    }

    fn propagate(&mut self, id: usize) {
        // Applica l'aggiornamento al nodo corrente
        self.nodes[id].key = self.nodes[id].key.min(self.nodes[id].lazy);

        // Propaga l'aggiornamento ai figli
        if let Some(left) = self.nodes[id].id_left {
            self.nodes[left].lazy = self.nodes[left].lazy.min(self.nodes[id].lazy);
        }
        if let Some(right) = self.nodes[id].id_right {
            self.nodes[right].lazy = self.nodes[right].lazy.min(self.nodes[id].lazy);
        }

        // Resetta l'aggiornamento in sospeso
        self.nodes[id].lazy = u32::MAX;
    }

    fn range_update(&mut self, node_id: usize, ql: usize, qr: usize, value: u32) {
        
        self.propagate(node_id); //Assicurati che l'aggiornamento in sospeso sia applicato

        // Caso 1: Intervallo completamente fuori
        if qr < self.nodes[node_id].left || ql > self.nodes[node_id].right {
            return;
        }

        // Caso 2: Intervallo completamente dentro
        if ql <= self.nodes[node_id].left && self.nodes[node_id].right <= qr {
            self.nodes[node_id].lazy = value; //Aggiungi l'aggiornamento in sospeso
            self.propagate(node_id); // Applica immediatamente
            return;
        }

        // Caso 3: Intervallo parzialmente dentro
        if let Some(left) = self.nodes[node_id].id_left {
            self.range_update(left, ql, qr, value);
        }
        if let Some(right) = self.nodes[node_id].id_right {
            self.range_update(right, ql, qr, value);
        }

        // Aggiorna il valore corrente dopo aver aggiornato i figli
        self.nodes[node_id].key = self.nodes[self.nodes[node_id].id_left.unwrap()]
            .key
            .max(self.nodes[self.nodes[node_id].id_right.unwrap()].key);
    }

    /// Esegue una query per sommare i valori nell'intervallo [ql, qr].
    fn query(&mut self, node_id: usize, ql: usize, qr: usize) -> Option<u32> {
        self.propagate(node_id);

        // Caso 1: Intervallo completamente fuori
        if qr < self.nodes[node_id].left || ql > self.nodes[node_id].right {
            return None;
        }

        // Caso 2: Intervallo completamente dentro
        if ql <= self.nodes[node_id].left && self.nodes[node_id].right <= qr {
            return Some(self.nodes[node_id].key);
        }

        // Caso 3: Intervallo parzialmente dentro
        let mut max = 0;
        if let Some(left) = self.nodes[node_id].id_left {
            if let Some(lres) = self.query(left, ql, qr) {
                max = max.max(lres);
            }
        }
        if let Some(right) = self.nodes[node_id].id_right {
            if let Some(rres) = self.query(right, ql, qr) {
                max = max.max(rres);
            }
        }
        Some(max)
    }

    pub fn update(&mut self, l: usize, r: usize, value: u32) {
        let root_id = self.nodes.len() - 1;
        self.range_update(root_id, l, r, value);
    }

    pub fn max(&mut self, l: usize, r: usize) -> Option<u32> {
        let root_id = self.nodes.len() - 1;
        self.query(root_id, l, r)
    }
}
