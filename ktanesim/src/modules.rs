use crate::prelude::*;
use strum_macros::Display;

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

use std::fmt;
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
            ModuleGroup::Category(cat) => cat.fmt(f),
            ModuleGroup::Origin(origin) => origin.fmt(f),
            ModuleGroup::Ruleseed => f.write_str("ruleseedable"),
            ModuleGroup::All => f.write_str("all"),
        }
    }
}

pub type ModuleNew = fn(bomb: &mut Bomb) -> Box<dyn Module>;

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

const VANILLA_MODULES: ModuleGroup = ModuleGroup::Origin(ModuleOrigin::Vanilla);
const SOLVABLE_MODULES: ModuleGroup = ModuleGroup::Category(ModuleCategory::Solvable);

pub mod wires;

pub const MODULE_SIZE: i32 = 348;

pub struct Render(pub Box<dyn FnOnce() -> (Vec<u8>, RenderType)>);

use serenity::builder::CreateMessage;
impl Render {
    /// Helper method for simpler creation of Render objects
    pub fn with(f: impl FnOnce() -> (Vec<u8>, RenderType) + 'static) -> Render {
        Render(Box::new(f))
    }

    pub fn resolve<F>(self, ctx: &Context, channel_id: ChannelId, f: F)
    where
        for<'b> F: FnOnce(&'b mut CreateMessage<'b>, &str) -> &'b mut CreateMessage<'b>,
    {
        let (data, extension) = (self.0)();
        let filename = format!("f.{}", extension);
        if let Err(why) = channel_id.send_files(
            &ctx.http,
            std::iter::once((&data[..], &filename[..])),
            |m| f(m, &format!("attachment://{}", filename)),
        ) {
            error!("Couldn't send message with attachment: {:?}", why);
        }
    }
}

pub trait Module: Send + Sync {
    fn handle_command(
        &mut self,
        bomb: Arc<RwLock<Bomb>>,
        user: UserId,
        command: &str,
    ) -> EventResponse;
    fn view(&self, light: SolveLight) -> Render;
}

pub struct EventResponse {
    render: Option<Render>,
    message: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum RenderType {
    PNG,
    GIF,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
