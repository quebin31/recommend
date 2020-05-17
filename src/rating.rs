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

use crate::score::Score;
use num_traits::real::Real;
use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

#[derive(Debug, Clone)]
pub struct Rating<T: Real> {
    data: Vec<Score<T>>,
}

impl<'a, T> Rating<T>
where
    T: Default + Real + 'a,
    &'a T: Add<Output = T> + Sub<Output = T>,
{
    pub fn manhattan(&'a self, rhs: &'a Self) -> Score<T> {
        self.data
            .iter()
            .zip(&rhs.data)
            .filter_map(|(l, r)| {
                if let Score::Some(s) = l - r {
                    let s = s.abs();
                    Some(Score::Some(s))
                } else {
                    None
                }
            })
            .fold(Score::Some(T::zero()), |acc, x| acc + x)
    }

    pub fn euclidean(&'a self, rhs: &'a Self) -> Score<T> {
        self.data
            .iter()
            .zip(&rhs.data)
            .filter_map(|(l, r)| {
                if let Score::Some(s) = l - r {
                    let s = s.powi(2);
                    Some(Score::Some(s))
                } else {
                    None
                }
            })
            .fold(Score::Some(T::zero()), |acc, x| acc + x)
            .map(T::sqrt)
    }

    pub fn minkowski(&'a self, rhs: &'a Self, p: usize) -> Score<T> {
        let d = self
            .data
            .iter()
            .zip(&rhs.data)
            .filter_map(|(l, r)| {
                if let Score::Some(s) = l - r {
                    let s = s.abs().powi(p as i32);
                    Some(Score::Some(s))
                } else {
                    None
                }
            })
            .fold(Score::Some(T::zero()), |acc, x| acc + x);

        if let Some(p) = T::from(p) {
            d.map(|v| v.powf(T::one() / p))
        } else {
            Score::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let r1 = Rating {
            data: vec![Score::Some(1.), Score::None, Score::Some(1.)],
        };

        let r2 = Rating {
            data: vec![Score::Some(1.), Score::Some(1.), Score::Some(2.)],
        };

        println!("{:?}", r1.manhattan(&r2));
    }
}
