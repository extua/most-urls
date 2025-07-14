use idna::uts46::{self, Uts46};
use nanoserde::DeJson;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use url::Url;

fn internationalised_domain_length(parsed_url: &Url) -> usize {
    let url = parsed_url.as_str();

    let i18n_domain_length_difference = match parsed_url.domain() {
        Some(raw_url_domain) => {
            let i18n_domain = Uts46::to_unicode(
                &Uts46::new(),
                raw_url_domain.as_bytes(),
                idna::AsciiDenyList::EMPTY,
                uts46::Hyphens::Allow,
            )
            .0;

            raw_url_domain.chars().count() - i18n_domain.chars().count()
        }
        None => 0,
    };

    let decoded_length_difference: usize = url.chars().count()
        - percent_encoding::percent_decode_str(url)
            .decode_utf8_lossy()
            .chars()
            .count();

    let total_difference: usize = i18n_domain_length_difference + decoded_length_difference;

    url.chars().count() - total_difference
}

fn main() {
    #[derive(DeJson, Debug)]
    struct CDXIndex {
        url: String,
        status: String,
    }

    if let Ok(lines) = read_lines("urls.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut url_list = Vec::new();

        let mut invalid_urls: usize = 0;

        let line_iterator = lines.map_while(Result::ok).enumerate();

        for line in line_iterator {
            // extract the json object from the cdx(j) line
            let index_json_line: &str = line.1.splitn(3, ' ').nth(2).unwrap();

            // Deserialise json
            let index: CDXIndex = DeJson::deserialize_json(index_json_line).unwrap();

            let parsed_url = match Url::parse(&index.url) {
                Ok(parsed_url) => parsed_url,
                Err(_) => {
                    invalid_urls = invalid_urls.wrapping_add(1);
                    continue;
                }
            };

            let url_length: usize = index.url.len();

            let i18_url_length: usize = internationalised_domain_length(&parsed_url);

            let status: char = index.status.chars().next().unwrap();

            let record = (index.url, url_length, i18_url_length, status);
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

        println!(
            "invalid {invalid_urls}\nduplicated {duplicated_list_size}\nunduplicated {unduplicated_list_size}"
        );

        let stringified_list: String = url_list
            .iter()
            .map(|f| format!("{},{},{}", f.1, f.2, f.3))
            .collect::<Vec<String>>()
            .join("\n");

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
