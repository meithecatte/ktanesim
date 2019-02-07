use ktanesim::modules::wires;
use ktanesim::prelude::*;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {
    unimplemented!();
    /*let rule_cache = Arc::new(Mutex::new(ShareMap::custom()));
    let mut bomb = Bomb { rule_seed: 1 };
    let mut module = wires::init(&mut bomb, rule_cache.lock().unwrap());
    let (data, filetype) = module.view()();
    let filename = format!("render.{}", filetype);
    let mut file = std::fs::File::create(filename)?;
    file.write_all(&data)?;
    Ok(())*/
}
