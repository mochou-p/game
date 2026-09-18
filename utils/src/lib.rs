// mochou-p/game/utils/src/lib.rs

use crate as utils;

use std::collections::HashMap;
use std::collections::hash_map::{Entry, VacantEntry, OccupiedEntry};
use std::fmt::Debug;
use std::hash::Hash;


#[macro_export]
macro_rules! style {
    ($color:expr, $tag:expr) => {
        concat!(
            "\x1b[10",
            stringify!($color),
            ";30;1m ",
            $tag,
            " @ ",
            file!(),
            ':',
            line!(),
            ':',
            column!(),
            " \x1b[0;3",
            stringify!($color),
            "m {}\x1b[0m"
        )
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! log {
    ($color:expr, $tag:expr, $($arg:expr),+) => {
        println!(utils::style!($color, $tag), format!($($arg),+))
    };
}

#[macro_export]
macro_rules! elog {
    ($color:expr, $tag:expr, $($arg:expr),+) => {
        eprintln!(utils::style!($color, $tag), format!($($arg),+))
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! todo {
    ($($arg:expr),+) => {
        #[cfg(debug_assertions)]
        utils::log!(5, "TODO", $($arg),+)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:expr),+) => {
        #[cfg(debug_assertions)]
        utils::log!(7, "DEBUG", $($arg),+)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:expr),+) => {
        #[cfg(debug_assertions)]
        utils::log!(6, "INFO", $($arg),+)
    };
}

#[macro_export]
macro_rules! ok {
    ($($arg:expr),+) => {
        #[cfg(debug_assertions)]
        utils::log!(2, "OK", $($arg),+)
    };
}

#[macro_export]
macro_rules! important {
    ($($arg:expr),+) => {
        utils::log!(4, "IMPORTANT", $($arg),+)
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:expr),+) => {
        utils::elog!(3, "WARNING", $($arg),+)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),+) => {
        utils::elog!(1, "ERROR", $($arg),+)
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Map<K: Debug + Eq + Hash, V: Debug>(pub HashMap<K, V>);

impl<K: Debug + Eq + Hash, V: Debug> Map<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: K, value: V) {
        debug!("{} + ({key:?}, {value:?})", std::any::type_name::<Self>());
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let value = self.0.remove(key);
        debug!("{} - ({key:?}, {value:?})", std::any::type_name::<Self>());
        value
    }

    pub fn entry<'a>(&'a mut self, key: K) -> MapEntry<'a, K, V> {
        match self.0.entry(key) {
            Entry::Vacant  (entry) => MapEntry::Vacant  (  VacantMapEntry(entry)),
            Entry::Occupied(entry) => MapEntry::Occupied(OccupiedMapEntry(entry))
        }
    }
}

pub enum MapEntry<'a, K: Debug + Eq + Hash, V: Debug> {
    Vacant  (  VacantMapEntry<'a, K, V>),
    Occupied(OccupiedMapEntry<'a, K, V>)
}

pub struct   VacantMapEntry<'a, K: Debug + Eq + Hash, V: Debug>(  VacantEntry<'a, K, V>);
pub struct OccupiedMapEntry<'a, K: Debug + Eq + Hash, V: Debug>(OccupiedEntry<'a, K, V>);

impl<'a, K: Debug + Eq + Hash, V: Debug> OccupiedMapEntry<'a, K, V> {
    pub fn get(&self) -> &V {
        self.0.get()
    }

    pub fn remove(self) {
        let value = self.0.remove();
        debug!("{} - {value:?}", std::any::type_name::<Self>());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn csprng<const N: usize>() -> Option<[u8; N]> {
    let mut buffer = [0; N];

    if let Err(err) = getrandom::fill(&mut buffer) {
        error!("failed to rng: {err}");
        None
    } else {
        Some(buffer)
    }
}

