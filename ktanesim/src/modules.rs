use crate::prelude::*;
use lazy_format::prelude::*;
use std::fmt;
use strum_macros::Display;

use phf_macros::phf_map;
use ModuleGroup::Single;

/// A perfect hash map of all modules and module groups
pub static MODULE_GROUPS: phf::Map<&'static str, ModuleGroup> = phf_map! {
    "wires"       => Single(&wires::DESCRIPTOR),
    "simplewires" => Single(&wires::DESCRIPTOR),

    "vanilla"      => VANILLA_MODULES,
    "base"         => VANILLA_MODULES,
    "unmodded"     => VANILLA_MODULES,
    "solvable"     => SOLVABLE_MODULES,
    "regular"      => SOLVABLE_MODULES,
    "normal"       => SOLVABLE_MODULES,
    "ruleseed"     => ModuleGroup::Ruleseed,
    "ruleseedable" => ModuleGroup::Ruleseed,
    "all"          => ModuleGroup::All,
    "any"          => ModuleGroup::All,
};

const VANILLA_MODULES: ModuleGroup = ModuleGroup::Origin(ModuleOrigin::Vanilla);
const SOLVABLE_MODULES: ModuleGroup = ModuleGroup::Category(ModuleCategory::Solvable);

pub mod wires;

#[cfg(test)]
#[test]
fn module_identifiers_consistent() {
    for (&key, group) in MODULE_GROUPS.entries() {
        if let Single(module) = group {
            assert_eq!(Some(group), MODULE_GROUPS.get(module.identifier));
            assert!(module.identifier == key || module.aliases.contains(&key));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ModuleOrigin {
    Vanilla,
    Modded,
    Novelty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ModuleCategory {
    Solvable,
    Needy,
    Boss,
}

pub struct ModuleDescriptor {
    pub identifier: &'static str,
    pub aliases: &'static [&'static str],
    pub constructor: ModuleNew,
    pub origin: ModuleOrigin,
    pub category: ModuleCategory,
    pub rule_seed: bool,
}

// Comparing constructor is enough, the rest is just metadata. This could be derived, but ModuleNew
// does not implement PartialEq because of higher-ranked trait bounds caused by the references.
// Compare rust-lang/rust#46989
impl Eq for ModuleDescriptor {}
impl PartialEq for ModuleDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.constructor as usize == other.constructor as usize
    }
}

use std::hash::{Hash, Hasher};
impl Hash for ModuleDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.constructor as usize).hash(state);
    }
}

impl fmt::Debug for ModuleDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<module {:?}>", self.identifier)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModuleGroup {
    Single(&'static ModuleDescriptor),
    Category(ModuleCategory),
    Origin(ModuleOrigin),
    Ruleseed,
    All,
}

use std::collections::HashSet;

impl ModuleGroup {
    /// Add all modules in the module group to the `set`.
    pub fn add_to_set(&self, set: &mut HashSet<&'static ModuleDescriptor>) {
        /// Helper function. Add all registered modules that match a predicate.
        fn add_matching<F>(set: &mut HashSet<&'static ModuleDescriptor>, f: F)
        where
            F: Fn(&'static ModuleDescriptor) -> bool,
        {
            for module in MODULE_GROUPS
                .values()
                .filter_map(|&group| {
                    if let ModuleGroup::Single(module) = group {
                        Some(module)
                    } else {
                        None
                    }
                })
                .filter(|&m| f(m))
            {
                set.insert(module);
            }
        }

        match *self {
            ModuleGroup::Single(module) => {
                set.insert(module);
            }
            ModuleGroup::Category(cat) => add_matching(set, |m| m.category == cat),
            ModuleGroup::Origin(origin) => add_matching(set, |m| m.origin == origin),
            ModuleGroup::Ruleseed => add_matching(set, |m| m.rule_seed),
            ModuleGroup::All => add_matching(set, |_| true),
        }
    }

    /// Remove all modules in the module group from the `set`.
    pub fn remove_from_set(&self, set: &mut HashSet<&'static ModuleDescriptor>) {
        match *self {
            ModuleGroup::Single(module) => {
                set.remove(module);
            }
            ModuleGroup::Category(cat) => set.retain(|m| m.category != cat),
            ModuleGroup::Origin(origin) => set.retain(|m| m.origin != origin),
            ModuleGroup::Ruleseed => set.retain(|m| !m.rule_seed),
            ModuleGroup::All => set.clear(),
        }
    }
}

impl fmt::Display for ModuleGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleGroup::Single(descriptor) => f.write_str(descriptor.identifier),
            ModuleGroup::Category(category) => category.fmt(f),
            ModuleGroup::Origin(origin) => origin.fmt(f),
            ModuleGroup::Ruleseed => f.write_str("ruleseedable"),
            ModuleGroup::All => f.write_str("all"),
        }
    }
}

pub type ModuleNew = fn(bomb: &mut BombData, state: ModuleState) -> Box<dyn Module>;

pub const MODULE_SIZE: i32 = 348;

pub trait Module: Send + Sync {
    fn descriptor(&self) -> &'static ModuleDescriptor;
    fn name(&self) -> &'static str;
    fn help_message(&self) -> &'static str;

    fn state(&self) -> &ModuleState;
    fn state_mut(&mut self) -> &mut ModuleState;

    fn view(&self, light: SolveLight) -> Render;
    fn handle_command(
        &mut self,
        bomb: &mut BombData,
        user: UserId,
        command: &str,
        params: Parameters<'_>,
    ) -> Result<EventResponse, ErrorMessage>;

    // Return the module number for this instance of the module
    fn number(&self) -> ModuleNumber {
        self.state().number
    }

    fn solve(&mut self, bomb: &mut BombData, user: UserId) -> EventResponse {
        assert!(!self.state().solved);
        assert!(self.descriptor().category != ModuleCategory::Needy);
        // TODO: leaderboard
        self.state_mut().solved = true;
        self.state_mut().user = Some(user);
        bomb.solved_count += 1;

        EventResponse {
            render: Some(self.view(SolveLight::Solved)),
            message: Some((
                "Module solved".to_owned(),
                "TODO: This is where you learn how much leaderboard points you just got."
                    .to_owned(),
            )),
        }
    }

    fn strike(&self, bomb: &mut BombData, user: UserId) -> EventResponse {
        // TODO: leaderboard
        bomb.timer.strike();
        EventResponse {
            render: Some(self.view(SolveLight::Strike)),
            message: Some((
                "Strike!".to_owned(),
                "TODO: This is where you learn how badly you fucked up.".to_owned(),
            )),
        }
    }
}

/// Get the manual URL for a module and rule seed combination. Returns `impl Display` to take
/// advantage of lazy formatting.
pub fn manual_url(module: &dyn Module, rule_seed: u32) -> impl fmt::Display {
    // TODO: manual mirror? (might be necessary for novelty modules)
    // OTOH, if choosing the stored preferred manual gets implemented...

    use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
    // RIGHT SINGLE QUOTATION MARK, used in the URLs of the manual repository
    let name = module.name().replace("\'", "\u{2019}");
    let seed = if module.descriptor().rule_seed {
        rule_seed
    } else {
        ktane_utils::random::VANILLA_SEED
    };

    lazy_format!(
        "https://ktane.timwi.de/HTML/{}.html#{}",
        utf8_percent_encode(&name, DEFAULT_ENCODE_SET),
        seed
    )
}

/// A struct used to store all the state common between modules.
#[derive(Debug)]
pub struct ModuleState {
    // private because of the need to trigger events on module solve.
    solved: bool,
    number: ModuleNumber,
    // claimed_by or solved_by
    pub user: Option<UserId>,
}

impl ModuleState {
    pub fn new(number: ModuleNumber) -> Self {
        ModuleState {
            solved: false,
            number,
            user: None,
        }
    }

    pub fn solved(&self) -> bool {
        self.solved
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SolveLight {
    Normal,
    Solved,
    Strike,
}

impl SolveLight {
    fn get_color(self) -> (f64, f64, f64) {
        use SolveLight::*;
        match self {
            Normal => (1.0, 1.0, 1.0),
            Solved => (0.0, 1.0, 0.0),
            Strike => (1.0, 0.0, 0.0),
        }
    }
}

/// Return an `ImageSurface` with a blank module drawn on it, along with a `CairoContext` that can
/// be used to draw any further graphics.
pub fn module_canvas(status: SolveLight) -> (ImageSurface, CairoContext) {
    let surface = ImageSurface::create(cairo::Format::ARgb32, MODULE_SIZE, MODULE_SIZE).unwrap();
    let ctx = CairoContext::new(&surface);

    ctx.set_line_join(cairo::LineJoin::Round);

    ctx.rectangle(5.0, 5.0, 338.0, 338.0);
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.fill_preserve();
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.stroke();

    let (r, g, b) = status.get_color();
    ctx.arc(298.0, 40.5, 15.0, 0.0, 2.0 * std::f64::consts::PI);
    ctx.set_source_rgb(r, g, b);
    ctx.fill_preserve();
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.stroke();

    (surface, ctx)
}

/// Given an `ImageSurface`, return what an implementation of `Render` should.
pub fn output_png(surface: ImageSurface) -> (Vec<u8>, RenderType) {
    let mut png = std::io::Cursor::new(vec![]);
    surface.write_to_png(&mut png).unwrap();
    (png.into_inner(), RenderType::PNG)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_type_is_lowercase() {
        assert_eq!(RenderType::PNG.to_string(), "png");
    }
}
