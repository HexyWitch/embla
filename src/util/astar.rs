use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};
use std::ops::Add;

use failure::Error;

struct Node<Q, F> {
    n: Q,
    prev: Option<Q>,
    cost: F,
}

impl<Q, F> Node<Q, F> {
    pub fn new(n: Q, prev: Option<Q>, cost: F) -> Self {
        Node { n, prev, cost }
    }
}

impl<Q: PartialEq, F> PartialEq for Node<Q, F> {
    fn eq(&self, other: &Node<Q, F>) -> bool {
        self.n == other.n
    }
}

impl<Q: PartialEq, F> Eq for Node<Q, F> {}

impl<Q: PartialEq + Ord, F: Ord> PartialOrd for Node<Q, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Q: PartialEq + Ord, F: Ord> Ord for Node<Q, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.cost, &self.n).cmp(&(&other.cost, &other.n))
    }
}

impl<Q: Hash, F> Hash for Node<Q, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.n.hash(state);
    }
}

impl<Q: Clone, F: Clone> Clone for Node<Q, F> {
    fn clone(&self) -> Self {
        Node {
            n: self.n.clone(),
            prev: self.prev.clone(),
            cost: self.cost.clone(),
        }
    }
}

struct NodeSet<Q, F> {
    q_map: HashMap<Q, Node<Q, F>>,
    f_set: BTreeSet<Node<Q, F>>,
}
impl<Q: Clone + Hash + Eq + Ord, F: Clone + Ord + Eq> NodeSet<Q, F> {
    pub fn new() -> NodeSet<Q, F> {
        NodeSet {
            q_map: HashMap::new(),
            f_set: BTreeSet::new(),
        }
    }

    fn get(&self, q: &Q) -> Option<&Node<Q, F>> {
        self.q_map.get(q)
    }

    fn insert(&mut self, node: Node<Q, F>) {
        self.q_map.insert(node.n.clone(), node.clone());
        self.f_set.insert(node);
    }

    fn take_next(&mut self) -> Option<Node<Q, F>> {
        self.f_set.iter().next().cloned().map(|n| {
            self.f_set.remove(&n);
            self.q_map.remove(&n.n);
            n
        })
    }
}

pub fn astar<'a, N, A, H, C>(start: N, end: N, adjacent: A, heuristic: H) -> Option<Vec<N>>
where
    N: Clone + Hash + Eq + Ord,
    A: Fn(N) -> Box<Iterator<Item = (N, C)> + 'a>,
    H: Fn(&N, &N) -> C,
    C: Clone + Copy + Ord + Default + Add<Output = C>,
{
    let mut closed_set = NodeSet::new();
    let mut open_set = NodeSet::new();
    open_set.insert(Node::new(start.clone(), None, C::default()));

    while let Some(parent) = open_set.take_next() {
        for (node, cost) in adjacent(parent.n.clone()) {
            if node == end {
                let mut path = vec![node, parent.n];
                let mut prev = parent.prev;
                while let Some(p) = prev {
                    path.push(p.clone());
                    prev = closed_set.get(&p).and_then(|n| n.prev.clone());
                }
                return Some(path.into_iter().rev().collect());
            }

            let g = parent.cost + cost;
            let h = heuristic(&node, &end);
            let f = g + h;

            if open_set.get(&node).map(|n| n.cost < f).unwrap_or(false) {
                continue;
            }

            if closed_set.get(&node).map(|n| n.cost > g).unwrap_or(true) {
                open_set.insert(Node::new(node, Some(parent.n.clone()), f));
            }
        }

        closed_set.insert(parent);
    }

    None
}
