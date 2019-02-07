use crate::prelude::*;
use rand::prelude::*;
use smallbitvec::SmallBitVec;
use ktane_utils::modules::wires::{generate, Color, RuleSet, MAX_WIRES};
use std::collections::HashMap;
use std::sync::{Arc, Weak};

pub struct Wires {
    rules: Arc<RuleSet>,
    wires: [Option<Color>; MAX_WIRES],
    cut_state: SmallBitVec,
}

pub struct RuleCacheKey();
impl typemap::Key for RuleCacheKey {
    type Value = HashMap<u32, Weak<RuleSet>>;
}

fn get_rules(seed: u32, mut rules_cache: MutexGuard<ShareMap>) -> Arc<RuleSet> {
    let rule_map = rules_cache
        .entry::<RuleCacheKey>()
        .or_insert_with(HashMap::new);

    use std::collections::hash_map::Entry::*;
    fn cache_miss<F: FnOnce(Weak<RuleSet>) -> X, X>(seed: u32, insert: F) -> Arc<RuleSet> {
        let rules = Arc::new(RuleSet::new(seed));
        insert(Arc::downgrade(&rules));
        rules
    }

    match rule_map.entry(seed) {
        Occupied(mut occupied) => match occupied.get().upgrade() {
            Some(rules) => rules,
            None => cache_miss(seed, |weak| occupied.insert(weak)),
        },
        Vacant(vacant) => cache_miss(seed, |weak| vacant.insert(weak)),
    }
}

pub fn init(bomb: &mut Bomb, rules_cache: MutexGuard<ShareMap>) -> Box<dyn Module> {
    let rules = get_rules(bomb.rule_seed, rules_cache);
    let (wires, wire_count) = generate(&mut rand::thread_rng());

    Box::new(Wires {
        rules,
        wires,
        cut_state: SmallBitVec::from_elem(wire_count as usize, false),
    })
}

impl Module for Wires {
    fn handle_command(&mut self, bomb: &mut Bomb, user: UserId, command: &str) -> EventResponse {
        unimplemented!();
    }

    fn view(&self, light: SolveLight) -> Render {
        let wires = self.wires;
        let cut_state = self.cut_state.clone();
        Box::new(move || {
            let (surface, ctx) = module_canvas(light);

            // Draw the boxes at the end of the wires
            ctx.set_source_rgb(0.0, 0.0, 0.0);
            ctx.rectangle(47.0, 62.0, 30.0, 226.0);
            ctx.stroke();

            ctx.rectangle(258.0, 107.0, 30.0, 178.0);
            ctx.stroke();

            ctx.set_line_cap(cairo::LineCap::Square);
            let mut cut_state = cut_state.into_iter();
            for ((wire, path), path_cut) in wires.into_iter().zip(PATHS).zip(PATHS_CUT) {
                if let Some(wire) = wire {
                    if cut_state.next().unwrap() {
                        path_cut(&ctx);
                    } else {
                        path(&ctx);
                    }

                    ctx.set_source_rgb(0.0, 0.0, 0.0);
                    ctx.set_line_width(8.0);
                    ctx.stroke_preserve();

                    match wire {
                        Color::Black => ctx.set_source_rgb(0.0, 0.0, 0.0),
                        Color::Blue => ctx.set_source_rgb(0.0, 0.0, 1.0),
                        Color::Red => ctx.set_source_rgb(1.0, 0.0, 0.0),
                        Color::White => ctx.set_source_rgb(1.0, 1.0, 1.0),
                        Color::Yellow => ctx.set_source_rgb(1.0, 1.0, 0.0),
                    }

                    ctx.set_line_width(4.0);
                    ctx.stroke();
                }
            }

            output_png(surface)
        })
    }
}

use cairo_svgpath::svgpath;
const PATHS: &[&Fn(&cairo::Context)] = &[
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m61.6063 94.05481c10.847099 -2.326851 19.528023 12.062485 30.566925 13.165352c24.493225 2.4470444 49.259254 -2.853485 73.83202 -1.4094467c11.687805 0.68683624 20.134766 12.74498 31.509186 15.519684c25.011337 6.1013184 50.6969 9.060364 76.183716 12.695541");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m65.367455 130.26443c24.469223 -11.123505 52.597984 13.950012 79.47506 13.637787c23.982635 -0.27859497 49.307022 -7.900284 71.95276 0c18.927505 6.603134 43.66768 30.634155 57.842514 16.45932");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m62.076115 163.18391c13.594257 -1.7824554 28.332623 -9.683853 40.913387 -4.233597c8.545235 3.7019806 7.230156 21.33052 16.45932 22.574814c30.325989 4.0885925 61.216156 -1.6049805 91.703415 -4.233597c21.870361 -1.8856659 43.41594 7.994751 65.36745 7.994751");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m61.133858 190.4596c14.410118 -0.9737854 29.280548 -5.961746 43.26509 -2.3516998c11.184219 2.8871613 21.443314 9.520508 32.921257 10.816269c22.413116 2.5302582 45.28653 2.3772125 67.24672 7.524933c11.842621 2.7760468 21.626816 5.8235016 29.157486 6.1128693c13.792221 0.529953 29.040268 -7.58667 41.383194 -1.4094543");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m61.133858 262.41208c9.028687 12.403412 31.701561 12.572601 45.6168 6.112854c10.123177 -4.699402 17.996696 -16.45932 29.157486 -16.45932c9.0716095 0 18.350845 1.4131927 26.80577 4.7008057c6.3217316 2.4580994 4.523987 16.401917 11.286087 16.931732c32.745956 2.565796 66.42221 0.9182739 98.2861 -7.0551147");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m66.30708 227.61273c46.126076 -8.542572 94.69835 5.857666 139.2021 20.690277c8.912521 2.9704437 16.024826 -11.53244 25.393707 -12.225723c14.25708 -1.0549774 28.499207 2.8215332 42.795258 2.8215332");
    },
];

const PATHS_CUT: &[&Fn(&cairo::Context)] = &[
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m 61.6063,94.05481 c 10.847099,-2.326851 19.528024,12.06248 30.566925,13.16535 16.662945,1.66475 33.452145,-0.25623 50.220865,-1.19995 m 23.61115,-0.20949 c 11.68781,0.68683 20.13477,12.74498 31.50919,15.51968 25.01134,6.10132 50.6969,9.06036 76.18372,12.69554");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m 65.367455,130.26443 c 24.469223,-11.1235 52.597985,13.95001 79.475055,13.63779 m 21.32517,-1.56094 c 17.2676,-1.85142 34.61706,-4.02457 50.62759,1.56094 18.92751,6.60313 43.66769,30.63415 57.84252,16.45932");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m 62.076115,163.18391 c 13.594257,-1.78246 28.332623,-9.68385 40.913385,-4.2336 8.54524,3.70198 7.23016,21.33052 16.45932,22.57482 6.72046,0.90606 13.46862,1.33172 20.23396,1.4168 m 26.51707,-1.09345 c 15.00505,-1.24863 30.027,-3.27008 44.95239,-4.55695 21.87036,-1.88567 43.41594,7.99475 65.36745,7.99475");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m 61.133858,190.4596 c 14.410118,-0.97379 29.280548,-5.96175 43.265092,-2.3517 11.18422,2.88716 21.44331,9.5205 32.92125,10.81627 m 29.20299,2.42125 c 12.80805,0.96657 25.57314,2.18043 38.04373,5.10368 11.84263,2.77605 21.62682,5.8235 29.15749,6.11287 13.79222,0.52995 29.04027,-7.58667 41.38319,-1.40945");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m 61.133858,262.41208 c 9.028687,12.40341 31.701561,12.5726 45.616802,6.11285 10.12317,-4.6994 17.99669,-16.45932 29.15748,-16.45932 m 26.80577,4.70081 c 6.32174,2.4581 4.52399,16.40192 11.28609,16.93173 32.74596,2.5658 66.42221,0.91828 98.2861,-7.05511");
    },
    &|ctx: &cairo::Context| {
        svgpath!(ctx, "m 66.30708,227.61273 c 24.321074,-4.50427 49.32223,-2.63004 74.04847,2.32286 m 23.62109,5.59233 c 14.09816,3.79679 28.00335,8.26596 41.53254,12.77509 8.91252,2.97044 16.02483,-11.53244 25.39371,-12.22573 14.25708,-1.05497 28.4992,2.82154 42.79525,2.82154");
    },
];
