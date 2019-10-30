//# structopt = "0.3.3"
//# rand = "0.7.2"

use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};
use structopt::StructOpt;
use rand::{Rng, distributions::Alphanumeric, seq::SliceRandom};

#[derive(StructOpt, Debug)]
#[structopt(name = "generate-big-input-json")]
struct CliOpt {
    /// Output file to add JSON object records to
    #[structopt(short, long, parse(from_os_str))]
    output_file: PathBuf,

    /// Number of JSON object records to generate
    #[structopt(short, long)]
    records: usize,
}

const MIN_ARRAY_LENGTH: usize = 1;
const MAX_ARRAY_LENGTH: usize = 5;

fn main() {
    let opt = CliOpt::from_args();

    let mut writer = BufWriter::new(File::create(opt.output_file).unwrap());

    let mut rng = rand::thread_rng();

    let types = vec![
        "ID",
        "LOCATION",
        "DATE",
        "NAME",
    ];

    for _ in 0..opt.records {
        let rand_file: String = rng
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();

        let rand_array_length = rng.gen_range(MIN_ARRAY_LENGTH, MAX_ARRAY_LENGTH);

        let mut rand_array = vec![];

        for _ in 0..rand_array_length {
            let (rand_start, rand_end) = rng.gen::<(u64, u64)>();

            let rand_word: String = rng
                .sample_iter(&Alphanumeric)
                .take(30)
                .collect();

            let rand_type = types.choose(&mut rng).unwrap();

            rand_array.push(format!(r##"
        {{
          "start": {},
          "end": {},
          "word": "{}",
          "type": "{}"
        }}"##, rand_start, rand_end, rand_word, rand_type));
        }

        let record = format!(r##"
{{
    "{}": [
        {}
    ]
}}
"##, rand_file, rand_array.join(",\n"));

        writer.write_all(record.as_bytes()).unwrap();
    }
}
