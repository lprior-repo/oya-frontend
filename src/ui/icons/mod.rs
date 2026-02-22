#![allow(non_snake_case)]
#![allow(unused_imports)]

mod registry;
mod set_a;
mod set_b;
mod set_c;

pub use registry::icon_by_name;
pub use set_b::{
    BoxIcon, CheckCircleIcon, CheckIcon, ClockIcon, MaximizeIcon, PlayIcon, RedoIcon, SaveIcon,
    SearchIcon, UndoIcon, XCircleIcon, ZoomInIcon, ZoomOutIcon,
};
pub use set_c::{
    ChevronDownIcon, CopyIcon, HelpCircleIcon, LayersIcon, SettingsIcon, TrashIcon, XIcon,
};
