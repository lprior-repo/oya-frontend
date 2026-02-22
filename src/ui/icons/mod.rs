#![allow(non_snake_case)]

mod registry;
mod set_a;
mod set_b;
mod set_c;

pub use registry::icon_by_name;
pub use set_b::{
    BoxIcon, MaximizeIcon, PlayIcon, RedoIcon, SaveIcon, SearchIcon, UndoIcon, ZoomInIcon,
    ZoomOutIcon,
};
pub use set_c::{
    ChevronDownIcon, CopyIcon, HelpCircleIcon, LayersIcon, SettingsIcon, TrashIcon, XIcon,
};
