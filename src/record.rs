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

use num_traits::real::Real;
use std::{
    collections::{hash_map::RandomState, HashMap},
    default::Default,
    fmt::Debug,
    hash::BuildHasher,
    ops::{AddAssign, Sub},
};

#[derive(Debug, Clone, Default)]
pub struct Record<V, S = RandomState>
where
    S: BuildHasher,
{
    values: HashMap<u64, V, S>,
}

impl<V> From<HashMap<u64, V>> for Record<V> {
    fn from(map: HashMap<u64, V>) -> Self {
        Self { values: map }
    }
}

impl<V, S> Record<V, S>
where
    V: Default,
    S: BuildHasher + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<V, S> Record<V, S>
where
    S: BuildHasher,
{
    pub fn values(&self) -> &HashMap<u64, V, S> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut HashMap<u64, V, S> {
        &mut self.values
    }
}

impl<'a, V, S> Record<V, S>
where
    S: BuildHasher,
    V: Real + AddAssign + 'a,
    &'a V: Sub<Output = V>,
{
    pub fn manhattan(&'a self, rhs: &'a Self) -> Option<V> {
        let mut dist = None;
        for (key, val_a) in &self.values {
            if let Some(val_b) = rhs.values.get(key) {
                *dist.get_or_insert(V::zero()) += (val_b - val_a).abs();
            }
        }

        dist
    }

    pub fn euclidean(&'a self, rhs: &'a Self) -> Option<V> {
        let mut dist = None;
        for (key, val_a) in &self.values {
            if let Some(val_b) = rhs.values.get(key) {
                *dist.get_or_insert(V::zero()) += (val_b - val_a).powi(2);
            }
        }

        dist.map(V::sqrt)
    }

    pub fn minkowski(&'a self, rhs: &'a Self, p: usize) -> Option<V> {
        let mut dist = None;
        for (key, val_a) in &self.values {
            if let Some(val_b) = rhs.values.get(key) {
                *dist.get_or_insert(V::zero()) += (val_b - val_a).abs().powi(p as i32);
            }
        }

        V::from(p)
            .map(|p| dist.map(|v| v.powf(V::one() / p)))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    #[test]
    fn manhattan_distance() {
        let a = Record {
            values: [(0, 1.), (2, 2.)]
                .iter()
                .cloned()
                .collect::<HashMap<u64, f64>>(),
        };

        let b = Record {
            values: [(0, 1.), (1, 3.), (2, 3.)].iter().cloned().collect(),
        };

        let d = b.manhattan(&a);

        assert_approx_eq!(1., d.unwrap());
    }

    #[test]
    fn euclidean_distance() {
        let a = Record {
            values: [(0, 0.), (2, 0.)]
                .iter()
                .cloned()
                .collect::<HashMap<u64, f64>>(),
        };

        let b = Record {
            values: [(0, 2.), (1, 1.), (2, 2.)].iter().cloned().collect(),
        };

        let d = b.euclidean(&a);

        assert_approx_eq!(8f64.sqrt(), d.unwrap());
    }

    #[test]
    fn minkowski3_distance() {
        let a = Record {
            values: [(0, 0.), (2, 0.)]
                .iter()
                .cloned()
                .collect::<HashMap<u64, f64>>(),
        };

        let b = Record {
            values: [(0, 2.), (1, 1.), (2, 2.)].iter().cloned().collect(),
        };

        let d = b.minkowski(&a, 3);

        assert_approx_eq!(16f64.powf(1. / 3.), d.unwrap());
    }
}
