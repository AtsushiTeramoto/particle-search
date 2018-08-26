extern crate rand;

#[derive(Debug)]
struct Particle {
    id :u64,
    position: [f64; 3],
    kernel_h: f64,
}

#[derive(Debug)]
enum Tree {
    Leaf {
        bound: [(f64, f64); 3],
        particle: Box<Particle>,
    },
    Node {
        bound: [(f64, f64); 3],
        child: [Option<Box<Tree>>; 8],
    },
}

impl Tree {
    fn new(bound: [(f64, f64); 3]) -> Tree {
        Tree::Node {
            bound: bound,
            child: [None, None, None, None, None, None, None, None],
        }
    }
    fn push(&mut self, add_particle: Box<Particle>) {
        match *self {
            Tree::Leaf {bound, particle: _} => {
                if let Tree::Leaf {bound: _, particle} = std::mem::replace(self, Tree::new(bound)) {
                    let center = subdivision_center(&bound);
                    let particle_index = subdivision_index(&center, &particle.position);
                    let new_bound = subdivision_bound(&bound, &center, &particle.position);
                    if let Tree::Node{bound: _, ref mut child} = *self {
                        child[particle_index] = Some(Box::new(Tree::Leaf {bound: new_bound, particle: particle}));
                    }
                }
                self.push(add_particle)
            },
            Tree::Node {ref bound, ref mut child} => {
                let center = subdivision_center(bound);
                let add_particle_index = subdivision_index(&center, &add_particle.position);
                if let Some(ref mut node) = child[add_particle_index] {
                    node.as_mut().push(add_particle)
                } else {
                    child[add_particle_index] = Some(Box::new(Tree::Leaf {
                        bound: subdivision_bound(&bound, &center, &add_particle.position),
                        particle: add_particle,
                    }))
                }
            },
        }
    }
    fn search(&self, search_bound: &[(f64, f64); 3]) -> Vec<&Particle> {
        match *self {
            Tree::Leaf {bound: _, ref particle}
            if search_bound[0].0 < particle.position[0] && particle.position[0] < search_bound[0].1
            && search_bound[1].0 < particle.position[1] && particle.position[1] < search_bound[1].1
            && search_bound[2].0 < particle.position[2] && particle.position[2] < search_bound[2].1
            => vec![particle.as_ref()],
            Tree::Node {ref bound, ref child}
            if bound[0].0 < search_bound[0].1 && search_bound[0].0 < bound[0].1
            && bound[1].0 < search_bound[1].1 && search_bound[1].0 < bound[1].1
            && bound[2].0 < search_bound[2].1 && search_bound[2].0 < bound[2].1
            => {
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

fn subdivision_center(bound: &[(f64, f64); 3]) -> [f64; 3] {
    [
        (bound[0].1 + bound[0].0) / 2.0,
        (bound[1].1 + bound[1].0) / 2.0,
        (bound[2].1 + bound[2].0) / 2.0,
    ]
}

fn subdivision_index(center: &[f64; 3], position: &[f64; 3]) -> usize {
    let index_x = if position[0] < center[0] {0} else {1};
    let index_y = if position[1] < center[1] {0} else {2};
    let index_z = if position[2] < center[2] {0} else {4};
    index_x + index_y + index_z
}

fn subdivision_bound(bound: &[(f64, f64); 3], center: &[f64; 3], position: &[f64; 3]) -> [(f64, f64); 3] {
    let bound_x = if position[0] < center[0] {(bound[0].0, center[0])} else {(center[0], bound[0].1)};
    let bound_y = if position[1] < center[1] {(bound[1].0, center[1])} else {(center[1], bound[1].1)};
    let bound_z = if position[2] < center[2] {(bound[2].0, center[2])} else {(center[2], bound[2].1)};
    [bound_x, bound_y, bound_z]
}

fn main() {
    let mut particle_list = Vec::new();
    for i in 0..1024 {
        let p = Particle {
            id: i,
            position: [rand::random(), rand::random(), rand::random()],
            kernel_h: 0.1,
        };
        particle_list.push(p);
    }
    let mut bound = [
        (particle_list[0].position[0], particle_list[0].position[0]),
        (particle_list[0].position[1], particle_list[0].position[1]),
        (particle_list[0].position[2], particle_list[0].position[2]),
    ];
    for p in &particle_list {
        if bound[0].0 > p.position[0] { bound[0].0 = p.position[0] }
        if bound[1].0 > p.position[1] { bound[1].0 = p.position[1] }
        if bound[2].0 > p.position[2] { bound[2].0 = p.position[2] }
        if bound[0].1 < p.position[0] { bound[0].1 = p.position[0] }
        if bound[1].1 < p.position[1] { bound[1].1 = p.position[1] }
        if bound[2].1 < p.position[2] { bound[2].1 = p.position[2] }
    }
    let mut size = bound[0].1 - bound[0].0;
    if size < bound[1].1 - bound[1].0 {size = bound[1].1 - bound[1].0};
    if size < bound[2].1 - bound[2].0 {size = bound[2].1 - bound[2].0};

    size *= 1.125;

    let center = subdivision_center(&bound);
    bound = [
        ((center[0] - size / 2.0), (center[0] + size / 2.0)),
        ((center[1] - size / 2.0), (center[1] + size / 2.0)),
        ((center[2] - size / 2.0), (center[2] + size / 2.0)),
    ];

    let mut particle_tree = Box::new(Tree::Node {
        bound: bound,
        child: [None, None, None, None, None, None, None, None],
    });
    for p in particle_list {
        particle_tree.as_mut().push(Box::new(p));
    }
    //println!("{:?}", particle_tree);
    let neighbor_list = particle_tree.search(&[(0.0,0.2),(0.0,0.2),(0.0,0.2)]);
    println!("{:?}", neighbor_list);
}
