extern crate rand;

use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
struct Vec3(f64, f64, f64);

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Vec3 {
    fn lt(&self, other: &Vec3) -> bool {
        self.0 < other.0 &&
        self.1 < other.1 &&
        self.2 < other.2
    }
    fn le(&self, other: &Vec3) -> bool {
        self.0 <= other.0 &&
        self.1 <= other.1 &&
        self.2 <= other.2
    }
}

#[derive(Debug, Copy, Clone)]
struct Bound(Vec3, Vec3);

impl Bound {
    fn is_in_bound(&self, pos: &Vec3) -> bool {
        (self.0).lt(pos) && pos.le(&self.1)
    }
    fn is_overlap(&self, other: &Bound) -> bool {
        (self.0).lt(&other.1) && (other.0).le(&self.1)
    }
    fn subdivision_center(&self) -> Vec3 {
        (self.0 + self.1)*0.5
    }
    fn subdivision_bound(&self, center: &Vec3, pos: &Vec3) -> Bound {
        let (x_lo, x_hi) = if pos.0 < center.0 {((self.0).0, center.0)} else {(center.0, (self.1).0)};
        let (y_lo, y_hi) = if pos.1 < center.1 {((self.0).1, center.1)} else {(center.1, (self.1).1)};
        let (z_lo, z_hi) = if pos.2 < center.2 {((self.0).2, center.2)} else {(center.2, (self.1).2)};
        Bound(Vec3(x_lo, y_lo, z_lo), Vec3(x_hi, y_hi, z_hi))
    }
}

#[derive(Debug)]
struct Particle {
    id :u64,
    position: Vec3,
    kernel_h: f64,
}

#[derive(Debug)]
enum Node {
    Node([Option<Box<Tree>>; 8]),
    Leaf(Particle),
}

#[derive(Debug)]
struct Tree {
    bound: Bound,
    data: Node,
}

impl Default for Node {
    fn default() -> Node {Node::Node(Default::default())}
}

impl Tree {
    fn new(bound: Bound) -> Tree {
        Tree {
            bound: bound,
            data: Default::default(),
        }
    }
    fn push(&mut self, add_particle: Particle) {
        let bound = self.bound;
        let center = bound.subdivision_center();
        match self.data {
            Node::Leaf(_) => {
                let leaf = std::mem::replace(&mut self.data, Default::default());
                if let Node::Leaf(particle) = leaf {//必ずマッチする
                    let particle_index = subdivision_index(&center, &particle.position);
                    let new_bound = bound.subdivision_bound(&center, &particle.position);
                    if let Node::Node(ref mut child) = self.data {//必ずマッチする
                        child[particle_index] = Some(Box::new(Tree {bound: new_bound, data: Node::Leaf(particle)}));
                    }
                }
                self.push(add_particle)
            },
            Node::Node(ref mut child) => {
                let add_particle_index = subdivision_index(&center, &add_particle.position);
                if let Some(ref mut node) = child[add_particle_index] {
                    node.as_mut().push(add_particle)
                } else {
                    child[add_particle_index] = Some(Box::new(Tree {
                        bound: bound.subdivision_bound(&center, &add_particle.position),
                        data: Node::Leaf(add_particle),
                    }))
                }
            },
        }
    }
    fn search(&self, search_bound: &Bound) -> Vec<&Particle> {
        match self.data {
            Node::Leaf(ref particle) if search_bound.is_in_bound(&particle.position) => vec![particle],
            Node::Node (ref child) if (self.bound).is_overlap(&search_bound) => {
                let mut particle_list = Vec::new();
                for child_node in child {
                    if let Some(ref node) = child_node {particle_list.append(&mut node.as_ref().search(search_bound))};
                }
                particle_list
            },
            _ => vec![],
        }
    }
}

fn subdivision_index(center: &Vec3, position: &Vec3) -> usize {
    let index_x = if position.0 < center.0 {0} else {1};
    let index_y = if position.1 < center.1 {0} else {2};
    let index_z = if position.2 < center.2 {0} else {4};
    index_x + index_y + index_z
}

fn main() {
    let mut particle_list = Vec::new();
    for i in 0..1024 {
        let p = Particle {
            id: i,
            position: Vec3(rand::random(), rand::random(), rand::random()),
            kernel_h: 0.1,
        };
        particle_list.push(p);
    }
    let mut bound = Bound(
        particle_list[0].position,
        particle_list[0].position,
    );
    for p in &particle_list {
        (bound.0).0 = ((bound.0).0).min(p.position.0);
        (bound.0).1 = ((bound.0).1).min(p.position.1);
        (bound.0).2 = ((bound.0).2).min(p.position.2);
        (bound.1).0 = ((bound.1).0).max(p.position.0);
        (bound.1).1 = ((bound.1).1).max(p.position.1);
        (bound.1).2 = ((bound.1).2).max(p.position.2);
    }
    let mut size = (bound.1).0 - (bound.0).0;
    size = size.max((bound.1).1 - (bound.0).1);
    size = size.max((bound.1).2 - (bound.0).2);

    size *= 1.125;

    let center = bound.subdivision_center();
    bound = Bound(
        Vec3(center.0 - size*0.5, center.1 - size*0.5, center.2 - size*0.5),
        Vec3(center.0 + size*0.5, center.1 + size*0.5, center.2 + size*0.5)
    );

    let mut particle_tree = Tree::new(bound);
    for p in particle_list {
        particle_tree.push(p);
    }
    //println!("{:?}", particle_tree);
    let neighbor_list = particle_tree.search(&Bound(Vec3(0.0,0.0,0.0),Vec3(0.2,0.2,0.2)));
    println!("{:?}", neighbor_list);
}
