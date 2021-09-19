//! Deals with loading custom schemas

use crate::panes::Paneable;
use anyhow::{anyhow, Result};
use libloading::{Library, Symbol};
use std::fmt::Debug;

/// Used to get the panes added on by a schema
pub trait AddonPanes {
    /// Return a list of the names of new panes added by this schema
    /// The positions of each name should stay the same
    fn names(&self) -> Vec<String>;

    /// Create a new instance of a pane, by the position of its name in the vector returned by `names`
    fn create(&self, pos: usize) -> Option<Box<dyn Paneable>>;
}

/// The name of the function you should export with the signature [`self::GetAddonPanes`]
pub const GET_ADDON_PANES: &[u8] = b"get_addon_panes";

/// A function that should be implemented at the top of your schema
pub type GetAddonPanes = extern "C" fn() -> Box<dyn AddonPanes>;

/// Holds information on the schema currently being used, including the panes it adds on
pub struct Schema {
    #[allow(unused)]
    lib: Library,
    addon_panes: Box<dyn AddonPanes>,
    cached_pane_names: Vec<String>,
}

impl Schema {
    /// Load a schema at the given path
    pub fn new(path: &String) -> Result<Self> {
        let (lib, addon_panes) = unsafe {
            let lib = Library::new(path)?;
            let func: Symbol<GetAddonPanes> = lib.get(GET_ADDON_PANES)?;
            let addon_panes = func();

            (lib, addon_panes)
        };

        let cached_pane_names = addon_panes.names();

        Ok(Self {
            lib,
            addon_panes,
            cached_pane_names,
        })
    }

    /// Create a pane given its position in [`Self::pane_names`]
    pub fn create_pane(&self, pos: usize) -> Result<Box<dyn Paneable>> {
        Ok(self
            .addon_panes()
            .create(pos)
            .ok_or(anyhow!("Addon pane type does not exist"))?)
    }

    /// Get a slice into the names of addon panes declared by this schema
    pub fn pane_names(&self) -> &[String] {
        self.cached_pane_names.as_slice()
    }

    /// Get a reference to the addon panes object returned by the schema.
    pub fn addon_panes(&self) -> &dyn AddonPanes {
        self.addon_panes.as_ref()
    }
}

impl Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("cached_pane_names", &self.cached_pane_names)
            .finish()
    }
}
