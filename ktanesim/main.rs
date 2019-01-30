use rand::prelude::*;
use ktane_utils::edgework::Edgework;
use ktane_utils::modules::wires;

fn main() {
    let edgework: Edgework = random();
    let wires = wires::generate(&mut thread_rng());
    println!("{:?}", edgework);
    println!("{:?}", wires);
}
