use nanoserde::{DeJson};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    // let mut input = String::new();

    // io::stdin().read_line(&mut input).unwrap();

    #[derive(DeJson, Debug)]
    struct CDXIndex {
        url: String,
    }

    if let Ok(lines) = read_lines("cdx-00000") {
        // Consumes the iterator, returns an (Optional) String

        let mut url_list = Vec::new();

        for line in lines.map_while(Result::ok).enumerate() {
            let index_json_line: &str = line.1.splitn(3, " ").nth(2).unwrap();

            let test: CDXIndex = DeJson::deserialize_json(index_json_line).unwrap();

            url_list.push(test.url);

            // now write out to a bufrwiter

            if line.0 == 100 {
                break;
            }
        };

        let duplicated_list = url_list.len();
        url_list.dedup();
        let unduplicated_list_size = url_list.len();

        println!("duplicated {duplicated_list} unduplicated {unduplicated_list_size}");

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
