use std::fmt::Display;

fn main() {
    let mut graph = Graph::new();

    let v1 = graph.add_node();
    let v2 = graph.add_node();
    let v3 = graph.add_node();

    graph.add_edge(v1, v2);
    graph.add_edge(v1, v3);
    graph.add_edge(v2, v3);
    graph.add_edge(v3, v1);

    println!("{}", graph);
}

struct Edge {
    vertex: usize,
    next: Option<Box<Edge>>,
}

struct Graph {
    adj: Vec<Option<Box<Edge>>>,
}

impl Graph {
    fn new() -> Self {
        Graph { adj: Vec::new() }
    }

    /// Time complexity: O(1)
    /// Space complexity: O(1)
    fn add_node(&mut self) -> usize {
        let id = self.adj.len();
        self.adj.push(None);
        id
    }

    /// Time complexity: O(1)
    /// Space complexity: O(1)
    fn add_edge(&mut self, from: usize, to: usize) {
        if from >= self.adj.len() || to >= self.adj.len() {
            panic!("Vertex with ID {} or {} does not exist.", from, to);
        }
        let new_edge = Box::new(Edge {
            vertex: to,
            next: self.adj[from].take(),
        });
        self.adj[from] = Some(new_edge);
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, edge_opt) in self.adj.iter().enumerate() {
            write!(f, "Node {} -> [", i)?;
            let mut current = edge_opt;
            let mut first = true;
            while let Some(edge) = current {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", edge.vertex)?;
                current = &edge.next;
                first = false;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}