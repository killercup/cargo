use std::fmt;
use std::hash::Hash;
use std::collections::hash_set::{HashSet, Iter};
use std::collections::hash_map::{HashMap, Keys};

pub struct Graph<N> {
    nodes: HashMap<N, HashSet<N>>,
}

enum Mark {
    InProgress,
    Done,
}

pub type Nodes<'a, N> = Keys<'a, N, HashSet<N>>;
pub type Edges<'a, N> = Iter<'a, N>;

impl<N: Eq + Hash + Clone> Graph<N> {
    pub fn new() -> Graph<N> {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add(&mut self, node: N, children: &[N]) {
        self.nodes
            .entry(node)
            .or_insert_with(HashSet::new)
            .extend(children.iter().cloned());
    }

    pub fn link(&mut self, node: N, child: N) {
        self.nodes
            .entry(node)
            .or_insert_with(HashSet::new)
            .insert(child);
    }

    pub fn get_nodes(&self) -> &HashMap<N, HashSet<N>> {
        &self.nodes
    }

    pub fn edges(&self, node: &N) -> Option<Edges<N>> {
        self.nodes.get(node).map(|set| set.iter())
    }

    pub fn sort(&self) -> Option<Vec<N>> {
        let mut ret = Vec::new();
        let mut marks = HashMap::new();

        for node in self.nodes.keys() {
            self.visit(node, &mut ret, &mut marks);
        }

        Some(ret)
    }

    fn visit(&self, node: &N, dst: &mut Vec<N>, marks: &mut HashMap<N, Mark>) {
        if marks.contains_key(node) {
            return;
        }

        marks.insert(node.clone(), Mark::InProgress);

        for child in &self.nodes[node] {
            self.visit(child, dst, marks);
        }

        dst.push(node.clone());
        marks.insert(node.clone(), Mark::Done);
    }

    pub fn iter(&self) -> Nodes<N> {
        self.nodes.keys()
    }

    /// Resolves one of the paths from the given dependent package up to
    /// the root.
    pub fn path_to_top<'a>(&'a self, mut pkg: &'a N) -> Vec<&'a N> {
        // Note that this implementation isn't the most robust per se, we'll
        // likely have to tweak this over time. For now though it works for what
        // it's used for!
        let mut result = vec![pkg];
        let first_pkg_depending_on = |pkg: &N, res: &[&N]| {
            self.get_nodes()
                .iter()
                .filter(|&(_node, adjacent)| adjacent.contains(pkg))
                // Note that we can have "cycles" introduced through dev-dependency
                // edges, so make sure we don't loop infinitely.
                .filter(|&(_node, _)| !res.contains(&_node))
                .next()
                .map(|p| p.0)
        };
        while let Some(p) = first_pkg_depending_on(pkg, &result) {
            result.push(p);
            pkg = p;
        }
        result
    }
}

impl<N: Eq + Hash + Clone> Default for Graph<N> {
    fn default() -> Graph<N> {
        Graph::new()
    }
}

impl<N: fmt::Display + Eq + Hash> fmt::Debug for Graph<N> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Graph {{")?;

        for (n, e) in &self.nodes {
            writeln!(fmt, "  - {}", n)?;

            for n in e.iter() {
                writeln!(fmt, "    - {}", n)?;
            }
        }

        write!(fmt, "}}")?;

        Ok(())
    }
}

impl<N: Eq + Hash> PartialEq for Graph<N> {
    fn eq(&self, other: &Graph<N>) -> bool {
        self.nodes.eq(&other.nodes)
    }
}
impl<N: Eq + Hash> Eq for Graph<N> {}

impl<N: Eq + Hash + Clone> Clone for Graph<N> {
    fn clone(&self) -> Graph<N> {
        Graph {
            nodes: self.nodes.clone(),
        }
    }
}
