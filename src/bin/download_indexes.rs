use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use flate2::bufread::GzDecoder;

fn main() {
    if let Ok(lines) = read_lines("cc-index.paths") {
        let line_iterator = lines.map_while(Result::ok);
        for download_path in line_iterator {
            if download_path.ends_with("gz") {
                println!("https://data.commoncrawl.org/{download_path}");

                // download the file


                // unzip and expose a bufreader

                // try this?
                // let mut gz = bufread::GzDecoder::new(&bytes[..]);

              
                // pass the bufreader to process_index
                // delete the file
            }
        }
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
