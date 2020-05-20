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
use assert_approx_eq::assert_approx_eq;
use recommend::{record::Record, table::Table, Distance};

fn load(file: &str) -> Result<Table<String, String, f64>, Error> {
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

    let mut table: Table<String, String, f64> = Table::with_keys(&keys);

    for i in 1..raw_table[0].len() {
        let mut record = Record::new();
        for row in raw_table.iter().skip(1) {
            let key = table.hash_key(&row[0]);
            let val = &row[i];

            if !val.is_empty() {
                let val: f64 = val.parse()?;
                record.values_mut().insert(key, val);
            }
        }

        table.insert(user[i - 1].clone(), record);
    }

    Ok(table)
}

#[test]
fn music() -> Result<(), Error> {
    let table = load("test_data/music.csv")?;

    assert_approx_eq!(
        table
            .distance_between("Angelica", "Chan", Distance::Euclidean)
            .unwrap(),
        table
            .distance_between("Angelica", "Chan", Distance::Minkowski(2))
            .unwrap()
    );

    Ok(())
}

#[test]
fn movies() -> Result<(), Error> {
    let table = load("test_data/movies.csv")?;

    assert_approx_eq!(
        table
            .distance_between("Patrick C", "Jeff", Distance::Manhattan)
            .unwrap(),
        table
            .distance_between("Patrick C", "Jeff", Distance::Minkowski(1))
            .unwrap()
    );

    Ok(())
}
