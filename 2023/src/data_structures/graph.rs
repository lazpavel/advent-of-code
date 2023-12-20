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

    pub fn get_edge(&self, current: usize) -> Option<&Edge> {
        for edge in &self.edges {
            if edge.from == current {
                return Some(edge);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: u32,

    pub pos_x: usize,
    pub pos_y: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: u32, (pos_x, pos_y): (usize, usize)) -> Self {
        Edge { from, to, weight, pos_x, pos_y }
    }
}