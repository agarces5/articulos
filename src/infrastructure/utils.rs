use tiberius::{FromSql, Row};

pub fn get_row_value<'a, T: ToString + FromSql<'a>>(row: &'a Row, idx: &'a str) -> String {
    row.try_get::<T, &'a str>(idx).unwrap().unwrap().to_string()
}
