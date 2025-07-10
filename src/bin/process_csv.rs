use polars::prelude::*;

fn main() {

    let csv_schema = Schema::from_iter(vec![
        Field::new("raw characters".into(), DataType::UInt64),
        Field::new("i18n characters".into(), DataType::UInt64),
        Field::new("status code".into(), DataType::String),
    ]);

    let lazy_frame = LazyCsvReader::new("values.csv")
        .with_has_header(false)
        .with_schema(Some(csv_schema.into()))
        .finish()
        .unwrap();

    let grouped_df = lazy_frame
        .group_by([col("raw characters")])
        .agg([col("status code").sum()])
        .sort(["raw characters"], Default::default())
        .collect()
        .unwrap();

    println!("{:?}", grouped_df);
}
