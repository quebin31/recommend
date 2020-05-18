// Copyright (C) 2020 Kevin Del Castillo Ramírez
//
// This file is part of recommend.
//
// recommend is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// recommend is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with recommend.  If not, see <http://www.gnu.org/licenses/>.

use crate::record::Record;
use crate::Distance;
use num_traits::real::Real;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Default)]
pub struct Table<T> {
    keys: Vec<String>,
    values: HashMap<String, Record<T>>,
}

impl<T: Default> Table<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_keys(keys: &[impl ToString]) -> Self {
        Self {
            keys: keys.iter().map(ToString::to_string).collect(),
            ..Default::default()
        }
    }
}

impl<T> Table<T> {
    pub fn insert(&mut self, key: &str, record: Record<T>) {
        self.values.insert(key.to_string(), record);
    }
}

impl<'a, T> Table<T>
where
    T: Real + 'a,
    &'a T: Add<Output = T> + Sub<Output = T>,
{
    pub fn distance_between(&'a self, a: &str, b: &str, method: Distance) -> Option<T> {
        let s = match method {
            Distance::Manhattan => self.values[a].manhattan(&self.values[b]),
            Distance::Euclidean => self.values[a].euclidean(&self.values[b]),
            Distance::Minkowski(p) => self.values[a].minkowski(&self.values[b], p),
        };

        s.into_option()
    }
}
