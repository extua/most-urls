use std::fs::File;
use polars::prelude::*;

fn main() {
    let csv_schema = Schema::from_iter(vec![
        Field::new("raw_characters".into(), DataType::UInt64),
        Field::new("i18n_characters".into(), DataType::UInt64),
        Field::new("status_code".into(), DataType::String),
    ]);

    let lazy_frame = LazyCsvReader::new("values.csv")
        .with_has_header(false)
        .with_schema(Some(csv_schema.into()))
        .finish()
        .unwrap();

    let mut grouped_df = lazy_frame
        .group_by(["raw_characters"])
        .agg([col("raw_characters").count().alias("frequency")])
        .sort(["raw_characters"], Default::default())
        .collect()
        .unwrap();

    let mut file = File::create("example.csv").expect("could not create file");

    let _ = CsvWriter::new(&mut file)
        .finish(&mut grouped_df);

    println!("{:?}", grouped_df);
}
