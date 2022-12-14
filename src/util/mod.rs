use std::hash::BuildHasherDefault;

use dashmap::DashMap;

pub use self::block_stuff::*;
pub use self::cli::*;
pub use self::serde::*;
pub use self::transaction::*;

mod block_stuff;
mod cli;
mod serde;
pub mod system;
mod transaction;

pub type FxDashMap<K, V> = DashMap<K, V, BuildHasherDefault<rustc_hash::FxHasher>>;
