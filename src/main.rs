extern crate rand;

struct Particle {
    id :u64,
    position: [f64; 3],
    kernel_h: f64,
}

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
    fn push(&mut self, add_particle : &Particle) {
        match *self {
            Tree::Leaf {bound : ref bound, particle: ref particle} => {
                let particle_index = subdivision_index(bound, &particle.position);
                let add_particle_index = subdivision_index(bound, &add_particle.position);
                let mut child : [Option<Box<Tree>>; 8] = [None; 8];
                if particle_index != add_particle_index {
                    *self = Tree::Node {
                        bound: bound,
                        child, child,
                    }
                }
            },
            Tree::Node {bound : ref bound, child: ref mut child} => {

            }
        }
    }
}

fn subdivision_index(bound: &[(f64, f64);3], position: &[f64; 3]) -> usize {
    let index_x = if position[0] < (bound[0].1 - bound[0].0)/2.0 {0} else {1};
    let index_y = if position[1] < (bound[1].1 - bound[1].0)/2.0 {0} else {2};
    let index_z = if position[2] < (bound[2].1 - bound[2].0)/2.0 {0} else {4};
    index_x + index_y + index_z
}

fn main() {
    let mut particle_list = Vec::new();
    for i in 1..64 {
        let p = Particle {
            id: i,
            position: [rand::random(), rand::random(), rand::random()],
            kernel_h: 0.1,
        };
        particle_list.push(p);
    }
}
