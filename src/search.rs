use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

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

#[derive(Debug, Copy, Clone)]
pub struct Bound(pub Vec3, pub Vec3);

impl Bound {
    fn is_in_bound(&self, pos: &Vec3) -> bool {
        let x_cond = if (self.0).0 <= (self.1).0 {
            (self.0).0 <= pos.0 && pos.0 < (self.1).0
        } else {
            (self.0).0 <= pos.0 || pos.0 < (self.1).0
        };
        let y_cond = if (self.0).1 <= (self.1).1 {
            (self.0).1 <= pos.1 && pos.1 < (self.1).1
        } else {
            (self.0).1 <= pos.1 || pos.1 < (self.1).1
        };
        let z_cond = if (self.0).2 <= (self.1).2 {
            (self.0).2 <= pos.2 && pos.2 < (self.1).2
        } else {
            (self.0).2 <= pos.2 || pos.2 < (self.1).2
        };
        x_cond && y_cond && z_cond
    }
    fn is_overlap(&self, other: &Bound) -> bool {
        let x_cond = if (other.0).0 <= (other.1).0 {
            (other.0).0 < (self.1).0 && (self.0).0 < (other.1).0
        } else {
            (other.0).0 < (self.1).0 || (self.0).0 < (other.1).0
        };
        let y_cond = if (other.0).1 <= (other.1).1 {
            (other.0).1 < (self.1).1 && (self.0).1 < (other.1).1
        } else {
            (other.0).1 < (self.1).1 || (self.0).1 < (other.1).1
        };
        let z_cond = if (other.0).2 <= (other.1).2 {
            (other.0).2 < (self.1).2 && (self.0).2 < (other.1).2
        } else {
            (other.0).2 < (self.1).2 || (self.0).2 < (other.1).2
        };
        x_cond && y_cond && z_cond
    }
    pub fn subdivision_center(&self) -> Vec3 {
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
pub struct Particle {
    pub id :u64,
    pub position: Vec3,
    pub kernel_h: f64,
}

#[derive(Debug)]
enum Node {
    Node([Option<Box<Tree>>; 8]),
    Leaf(Particle),
}

#[derive(Debug)]
pub struct Tree {
    bound: Bound,
    data: Node,
}

impl Default for Node {
    fn default() -> Node {Node::Node(Default::default())}
}

impl Tree {
    pub fn new(bound: Bound) -> Tree {
        Tree {
            bound: bound,
            data: Default::default(),
        }
    }
    pub fn push(&mut self, add_particle: Particle) {
        let bound = self.bound;
        let center = bound.subdivision_center();
        match self.data {
            Node::Leaf(_) => {
                let (particle_index, new_bound) = if let Node::Leaf(ref particle) = self.data {
                    (subdivision_index(&center, &particle.position),
                    bound.subdivision_bound(&center, &particle.position))
                } else {unreachable!()};
                self.data = Node::Node({
                    let mut child:[Option<Box<Tree>>; 8] = Default::default();
                    child[particle_index] = Some(Box::new(Tree {bound: new_bound, data: ::std::mem::replace(&mut self.data, Default::default())}));
                    child
                });
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
    pub fn search(&self, search_bound: &Bound) -> Vec<&Particle> {
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