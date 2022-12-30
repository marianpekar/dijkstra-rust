use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Copy, Eq)]
struct Vertex<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    id: usize,
    value: T,
}

impl<T> Ord for Vertex<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> PartialOrd for Vertex<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Clone for Vertex<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn clone(&self) -> Self {
        Vertex {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

impl<T> PartialEq for Vertex<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Hash for Vertex<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

struct Edge<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    to: Vertex<T>,
    cost: f64,
}

impl<T> Edge<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn new(to: Vertex<T>, cost: f64) -> Self {
        Self { to, cost }
    }
}

struct Graph<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    graph: HashMap<Vertex<T>, Vec<Edge<T>>>,
    head: usize,
}

impl<T> Graph<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            head: 0,
        }
    }

    fn add_vertex(&mut self, value: T) -> usize {
        let vertex = Vertex::<T> {
            id: self.head,
            value,
        };
        self.graph.insert(vertex, Vec::new());
        self.head += 1;
        self.head - 1
    }

    fn get_vertex(&self, id: usize) -> &Vertex<T> {
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

    fn get_shortest_path(&self, from: usize, to: usize) -> (Vec<Vertex<T>>, f64) {
        let start = self.get_vertex(from);
        let end = self.get_vertex(to);

        let (prev, distance, start, end) = self.dijkstra(*start, *end);

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

    fn dijkstra(
        &self,
        start: Vertex<T>,
        end: Vertex<T>,
    ) -> (Vec<Option<Vertex<T>>>, f64, Vertex<T>, Vertex<T>) {
        let mut dist = vec![f64::INFINITY; self.graph.len()];
        dist[start.id] = 0.0;

        let mut queue = BinaryHeap::new();
        queue.push((MinNonNan(0.0), start));

        let mut visited = vec![false; self.graph.len()];
        let mut prev: Vec<Option<Vertex<T>>> = vec![None; self.graph.len()];

        while let Some((current_cost, current)) = queue.pop() {
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
                return (prev, dist[end.id], start, end);
            }
        }

        (prev, f64::INFINITY, start, end)
    }
}

impl<T: Display> Display for Graph<T>
where
    T: Ord + PartialOrd + Eq + PartialEq + Clone + Copy,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut graph_string: String = String::new();
        for (key, value) in &self.graph {
            graph_string += format!("[{}] -> ", key.value).as_str();
            for edge in value {
                graph_string += format!("[{} ({})]", edge.to.value, edge.cost).as_str();
            }
            graph_string += "\n"
        }

        write!(f, "{}", graph_string)
    }
}

fn main() {
    let mut graph = Graph::<&'static str>::new();

    let a = graph.add_vertex(&"A");
    let b = graph.add_vertex(&"B");
    let c = graph.add_vertex(&"C");
    let d = graph.add_vertex(&"D");
    let e = graph.add_vertex(&"E");

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

    /*
    Output:    
    [D] -> [E (4)]
    [C] -> [D (1)][E (3)]
    [A] -> [B (1)][C (3)]
    [B] -> [D (2)][E (8)][C (1)]
    [E] -> [C (3)]
    The shortest path has value of 5 and leads via [Vertex { id: 0, value: "A" }, Vertex { id: 1, value: "B" }, Vertex { id: 2, value: "C" }, Vertex { id: 4, value: "E" }]
    */
}
