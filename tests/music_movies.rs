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

use anyhow::Error;
use recommend::{score::Score, table::Table};

fn load(file: &str) -> Result<Table<f64>, Error> {
    let mut csv = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)?;

    let mut raw_table = Vec::new();

    for record in csv.records() {
        if let Ok(record) = record {
            let mut row = Vec::new();
            for val in record.iter() {
                row.push(val.to_string());
            }
            raw_table.push(row);
        }
    }

    let keys: Vec<_> = raw_table.iter().skip(1).map(|r| r[0].clone()).collect();
    let user: Vec<_> = raw_table[0].iter().skip(1).collect();

    let mut table = Table::<f64>::with_keys(&keys);

    for i in 1..raw_table[0].len() {
        let mut scores = Vec::new();
        for row in raw_table.iter().skip(1) {
            let val = &row[i];

            if val.is_empty() {
                scores.push(Score::None);
            } else {
                let val: f64 = val.parse()?;
                scores.push(Score::Some(val));
            }
        }

        table.insert(user[i - 1], scores.into());
    }

    Ok(table)
}

#[test]
fn music() -> Result<(), Error> {
    let table = load("test_data/music.csv")?;
    Ok(())
}

#[test]
fn movies() -> Result<(), Error> {
    let table = load("test_data/movies.csv")?;
    Ok(())
}
