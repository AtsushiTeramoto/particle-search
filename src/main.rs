extern crate rand;

mod search;

fn main() {
    let mut particle_list = Vec::new();
    for i in 0..1024 {
        let p = search::Particle {
            id: i,
            position: search::Vec3(rand::random(), rand::random(), rand::random()),
            kernel_h: 0.1,
        };
        particle_list.push(p);
    }
    let mut bound = search::Bound(
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
    bound = search::Bound(
        search::Vec3(center.0 - size*0.5, center.1 - size*0.5, center.2 - size*0.5),
        search::Vec3(center.0 + size*0.5, center.1 + size*0.5, center.2 + size*0.5)
    );

    let mut particle_tree = search::Tree::new(bound);
    for p in particle_list {
        particle_tree.push(p);
    }
    //println!("{:?}", particle_tree);
    let neighbor_list = particle_tree.search(&search::Bound(search::Vec3(0.9,0.9,0.9),search::Vec3(0.1,0.1,0.1)));
    println!("{:?}", neighbor_list);
}
