#[derive(Debug)]
pub struct Graph {
    pub size: usize,
    pub edges: Vec<Edge>,
    pub glyph_groups: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            size,
            edges: Vec::new(),
            glyph_groups: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn neighbors(&self, current: usize) -> Option<Vec<usize>> {
        let mut neighbors = Vec::new();

        for edge in &self.edges {
            if edge.from == current {
                neighbors.push(edge.to);
            }
        }

        Some(neighbors)
    }
}

#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: u32,
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: u32) -> Self {
        Edge { from, to, weight }
    }
}