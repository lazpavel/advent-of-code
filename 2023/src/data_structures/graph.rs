use fnv::FnvHashMap;

#[derive(Debug, Clone)]
pub struct Graph<Vid, E = (), V = ()> {
    pub nodes: FnvHashMap<Vid, V>,
    pub adjacency: FnvHashMap<Vid, Vec<(Vid, E)>>,
    pub edges: Vec<(Vid, Vid, E)>,
}

impl<Vid, E, V> Graph<Vid, E, V>
where
    Vid: std::hash::Hash + std::cmp::Eq + std::clone::Clone,
    V: std::hash::Hash + std::clone::Clone,
    E: std::hash::Hash + std::clone::Clone,
{
    pub fn new() -> Graph<Vid, E, V> {
        Graph {
            nodes: FnvHashMap::default(),
            adjacency: FnvHashMap::default(),
            edges: Vec::new(),
        }
    }

    pub fn neighbors(&self, vid: Vid) -> Option<&Vec<(Vid, E)>> {
        self.adjacency.get(&vid)
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_undirected_edges(&self) -> Vec<(Vid, Vid, E)> {
        let mut edges = Vec::new();
        for (from, to, edge) in &self.edges {
            if let Some(_) = edges.iter().find(|(f, t, _)| f == to && t == from) {
                continue;
            }
            edges.push((from.clone(), to.clone(), edge.clone()));
            
        }
        edges
    }

    pub fn add_node(&mut self, vid: Vid, vertex: V) {
        self.nodes.insert(vid, vertex);
    }

    pub fn add_edge(&mut self, from: Vid, to: Vid, edge: E) {
        if let Some(_) = self.edges.iter().find(|(f, t, _)| f == &from && t == &to) {
            return;
        }
        
        self.adjacency
            .entry(from.clone())
            .or_default()
            .push((to.clone(), edge.clone()));
        self.edges.push((from, to, edge));
    }

    pub fn add_undirected_edge(&mut self, from: Vid, to: Vid, edge: E) {
        self.add_edge(from.clone(), to.clone(), edge.clone());
        self.add_edge(to, from, edge);
    }

    pub fn remove_undirected_edge(&mut self, from: Vid, to: Vid) {
        self.remove_edge(from.clone(), to.clone());
        self.remove_edge(to, from);
    }

    pub fn remove_edge(&mut self, from: Vid, to: Vid) {
        self.edges.retain(|(f, t, _)| f != &from || t != &to);
        self.adjacency
            .get_mut(&from)
            .map(|neighbors| neighbors.retain(|(t, _)| t != &to));
    }
}