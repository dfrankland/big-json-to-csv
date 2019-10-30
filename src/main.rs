use csv::WriterBuilder;
use serde::Serialize;
use serde_json::{Deserializer, Value};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "big-json-to-csv", about = "What big file?")]
struct CliOpt {
    /// Input JSON file
    #[structopt(short, long, parse(from_os_str))]
    input_json: PathBuf,

    /// Output CSV file
    #[structopt(short, long, parse(from_os_str))]
    output_csv: PathBuf,
}

#[derive(Serialize)]
struct Row {
    file: String,
    start: u64,
    end: u64,
    word: String,
    r#type: String,
}

fn main() {
    let opt = CliOpt::from_args();

    let reader = BufReader::new(File::open(opt.input_json).unwrap());
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut writer = WriterBuilder::new()
        .from_path(opt.output_csv)
        .expect("Couldn't create csv file");

    for top_level_possible_value in stream {
        let top_level_value = top_level_possible_value.expect("No top-level value found");
        let top_level_object = top_level_value
            .as_object()
            .expect("Top-level value is not an object");
        for (file, nested_value) in top_level_object.iter() {
            let array = nested_value
                .as_array()
                .expect("Nested value is not an array");
            for array_nested_value in array.iter() {
                let array_nested_object = array_nested_value
                    .as_object()
                    .expect("Nested array value is not an object");
                let row = Row {
                    file: file.to_string(),
                    start: array_nested_object
                        .get("start")
                        .expect("Nested array object has no property \"start\"")
                        .as_u64()
                        .expect("\"start\" is not a number"),
                    end: array_nested_object
                        .get("end")
                        .expect("Nested array object has no property \"end\"")
                        .as_u64()
                        .expect("\"end\" is not a number"),
                    word: array_nested_object
                        .get("word")
                        .expect("Nested array object has no property \"word\"")
                        .as_str()
                        .expect("\"word\" is not a string")
                        .to_string(),
                    r#type: array_nested_object
                        .get("type")
                        .expect("Nested array object has no property \"type\"")
                        .as_str()
                        .expect("\"type\" is not a string")
                        .to_string(),
                };
                writer
                    .serialize(row)
                    .expect("Could not write a row to csv file");
            }
        }
    }
}
