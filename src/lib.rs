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

use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Score<T> {
    Some(T),
    None,
}

impl<T> Score<T> {
    pub fn is_none(&self) -> bool {
        match self {
            Score::Some(_) => false,
            _ => true,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Score::Some(_) => true,
            _ => false,
        }
    }
}

impl<T> Add for Score<T>
where
    T: Add,
{
    type Output = Score<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Score::Some(l), Score::Some(r)) => Score::Some(l + r),
            _ => Score::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_sum() {
        assert_eq!(Score::Some(3), Score::Some(2) + Score::Some(1));
        assert_eq!(Score::None, Score::Some(2) + Score::None);
        assert_eq!(Score::Some(2), Score::Some(1) + Score::Some(2));
    }
}
