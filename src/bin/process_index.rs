use nanoserde::DeJson;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    #[derive(DeJson, Debug)]
    struct CDXIndex {
        url: String,
        status: String,
    }

    if let Ok(lines) = read_lines("urls.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut url_list = Vec::new();

        let line_iterator = lines.map_while(Result::ok).enumerate();

        for line in line_iterator {
            // extract the json object from the cdx(j) line
            let index_json_line: &str = line.1.splitn(3, " ").nth(2).unwrap();

            // Deserialse json to extract just the url
            let index: CDXIndex = DeJson::deserialize_json(index_json_line).unwrap();

            let url_length = index.url.len();

            let status = index.status.chars().nth(0).unwrap();
            println!("{status}");

            let record = (index.url, url_length, status);

            // push to url list a tuple of strings
            // including the digest

            url_list.push(record);
        }

        // filter the list and deduplicate by key
        // should be 225 when testing with test dataset
        let duplicated_list_size = url_list.len();
        // deduplicate by url
        url_list.dedup_by(|a, b| a.0 == b.0);
        let unduplicated_list_size = url_list.len();

        println!("duplicated {duplicated_list_size} unduplicated {unduplicated_list_size}");

        let list_string_tuples: Vec<String> = url_list
            .iter()
            .map(|f| format!("{},{}", f.1.to_string(), f.2))
            .collect();

        println!("{:?}", list_string_tuples);
        let stringified_list = list_string_tuples.join("\n");

        fs::write("values.csv", stringified_list).unwrap();
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
