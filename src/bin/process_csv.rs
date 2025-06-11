use polars::prelude::*;

fn main() {
    let csv_schema = Schema::from_iter(vec![
        Field::new("raw characters".into(), DataType::UInt64),
        Field::new("status code".into(), DataType::UInt32),
    ]);

    let df = LazyCsvReader::new("values.csv")
        .with_has_header(false)
        .with_schema(Some(csv_schema.into()))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    print!("{df}");
}
