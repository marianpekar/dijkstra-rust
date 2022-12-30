use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
struct Vertex {
    id: usize,
    value: &'static str,
}

struct Edge {
    to: Vertex,
    cost: f64,
}

impl Edge {
    fn new(to: Vertex, cost: f64) -> Self {
        Self { to, cost }
    }
}

struct Graph {
    graph: HashMap<Vertex, Vec<Edge>>,
    head: usize,
}

impl Graph {
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            head: 0,
        }
    }

    fn add_vertex(&mut self, value: &'static str) -> usize {
        let vertex = Vertex {
            id: self.head,
            value,
        };
        self.graph.insert(vertex, Vec::new());
        self.head += 1;
        self.head - 1
    }

    fn get_vertex(&self, id: usize) -> &Vertex {
        self.graph
            .iter()
            .find(|(key, _)| key.id == id)
            .map(|(key, _)| key)
            .unwrap_or_else(|| panic!("There is no vertex with id {}", id))
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: f64) {
        let edge = Edge::new(*self.get_vertex(to), cost);
        self.graph
            .entry(*self.get_vertex(from))
            .or_insert(vec![])
            .push(edge);
    }

    fn get_shortest_path(&self, from: usize, to: usize) -> (Vec<Vertex>, f64) {
        let start = *self.get_vertex(from);
        let end = *self.get_vertex(to);

        let (prev, distance) = self.dijkstra(start, end);

        let mut path = Vec::new();
        let mut at = end;
        while at.id != start.id {
            path.push(at);
            at = prev[at.id].unwrap();
        }
        path.push(at);
        path.reverse();

        (path, distance)
    }

    fn dijkstra(&self, start: Vertex, end: Vertex) -> (Vec<Option<Vertex>>, f64) {
        let mut dist = vec![f64::INFINITY; self.graph.len()];
        dist[start.id] = 0.0;

        let mut queue = BinaryHeap::new();
        queue.push((MinNonNan(0.0), start));

        let mut visited = vec![false; self.graph.len()];
        let mut prev: Vec<Option<Vertex>> = vec![None; self.graph.len()];

        while !queue.is_empty() {
            let (current_cost, current) = queue.pop().unwrap();

            visited[current.id] = true;

            if dist[current.id] < current_cost.0 {
                continue;
            }

            let edges = self.graph.get(&current).unwrap();
            for edge in edges {
                if visited[edge.to.id] {
                    continue;
                }

                let new_dist = dist[current.id] + edge.cost;
                if new_dist < dist[edge.to.id] {
                    prev[edge.to.id] = Some(current);
                    dist[edge.to.id] = new_dist;
                    queue.push((MinNonNan(new_dist), edge.to))
                }
            }
            if current.id == end.id {
                return (prev, dist[end.id]);
            }
        }

        (prev, f64::INFINITY)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut graph_string: String = String::new();
        for (key, value) in &self.graph {
            graph_string += format!("[{}] ->", key.value).as_str();
            for edge in value {
                graph_string += format!("[{} ({})]", edge.to.value, edge.cost).as_str();
            }
            graph_string += "\n"
        }

        write!(f, "{}", graph_string)
    }
}

fn main() {
    let mut graph = Graph::new();

    let a = graph.add_vertex("A");
    let b = graph.add_vertex("B");
    let c = graph.add_vertex("C");
    let d = graph.add_vertex("D");
    let e = graph.add_vertex("E");

    graph.add_edge(a, b, 1.0);
    graph.add_edge(a, c, 3.0);
    graph.add_edge(b, d, 2.0);
    graph.add_edge(b, e, 8.0);
    graph.add_edge(b, c, 1.0);
    graph.add_edge(c, d, 1.0);
    graph.add_edge(d, e, 4.0);
    graph.add_edge(e, c, 3.0);
    graph.add_edge(c, e, 3.0);

    print!("{}", graph);

    let shortest_path = graph.get_shortest_path(a, e);
    print!(
        "The shortest path has value of {} and leads via {:?}",
        shortest_path.1, shortest_path.0
    );
}
