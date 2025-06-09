use polars::prelude::*;

fn main() {
 let df = LazyCsvReader::new("values.csv")
    .with_has_header(true)
    .with_separator(b',')
    .finish().unwrap()

    // let lf: DataFrame = CsvReader::new("values.csv")
    //     .with_has_header(false)
    //     .finish()
    //     .unwrap();

    println!("{df}");
}
