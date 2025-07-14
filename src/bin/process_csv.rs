use polars::prelude::*;
use std::fs::File;

fn main() {
    let csv_schema = Schema::from_iter(vec![
        Field::new("raw_characters".into(), DataType::UInt64),
        Field::new("i18n_characters".into(), DataType::UInt64),
        Field::new("status_code".into(), DataType::UInt32),
    ]);

    let lazy_frame = LazyCsvReader::new("values.csv")
        .with_has_header(false)
        .with_schema(Some(csv_schema.into()))
        .finish()
        .unwrap();

    let mut grouped_df = lazy_frame
        .group_by(["raw_characters"])
        .agg([
            col("raw_characters")
                .filter(col("status_code").eq(lit(1u8)))
                .count()
                .alias("informational"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(2u8)))
                .count()
                .alias("successful"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(3u8)))
                .count()
                .alias("redirection"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(4u8)))
                .count()
                .alias("client_error"),
            col("raw_characters")
                .filter(col("status_code").eq(lit(5u8)))
                .count()
                .alias("server_error"),
            col("raw_characters").count().alias("total"),
        ])
        .sort(["raw_characters"], Default::default())
        .collect()
        .unwrap();

    let mut file = File::create("frequency.csv").expect("could not create file");

    CsvWriter::new(&mut file)
        .finish(&mut grouped_df)
        .expect("could not write to file");

    println!("{:?}", grouped_df);
}
