struct Particle {
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

fn main() {
    println!("Hello, world!");
}
