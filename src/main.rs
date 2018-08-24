extern crate rand;

#[derive(Debug)]
struct Particle {
    id :u64,
    position: [f64; 3],
    kernel_h: f64,
}

#[derive(Debug)]
enum Tree<'a> {
    Leaf {
        bound: [(f64, f64); 3],
        particle: &'a Particle,
    },
    Node {
        bound: [(f64, f64); 3],
        child: [Option<Box<Tree<'a>>>; 8],
    },
}

impl<'a> Tree<'a> {
    fn push(&mut self, add_particle : &'a Particle) {
        match *self {
            Tree::Leaf {bound, particle} => {
                let center = subdivision_center(&bound);
                let particle_index = subdivision_index(&center, &particle.position);
                let mut child : [Option<Box<Tree>>; 8] = [None ,None, None, None, None, None, None, None];
                let new_bound = subdivision_bound(&bound, &center, &particle.position);
                child[particle_index] = Some(Box::new(Tree::Leaf {bound: new_bound, particle: particle}));
                *self = Tree::Node {
                    bound: bound,
                    child: child,
                };
                self.push(add_particle)     //can't tail call elimination.
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
    for i in 0..16 {
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
    let mut particle_tree = Box::new(Tree::Node {
        bound: bound,
        child: [None, None, None, None, None, None, None, None],
    });
    for p in &particle_list {
        particle_tree.as_mut().push(p);
    }
    println!("{:?}", particle_tree);
}
