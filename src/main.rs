mod cli_io;
mod errors;
mod header_map;

use bpaf::Bpaf;

use self::cli_io::{Input, Output};
use self::header_map::HeaderMappings;

/// Convert a JSON file (with top-level array) to a CSV table.
#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, generate(parse_cli_env_args))]
struct Command {
    #[bpaf(positional("INPUT_JSON"))]
    input: Input,
    #[bpaf(positional("MAPPING_JSON"))]
    header_map: Input,
    #[bpaf(positional("OUTPUT_CSV"), fallback(Output::Stdout))]
    output: Output,
}

type JsonRecords = Vec<serde_json::Value>;

fn main() -> anyhow::Result<()> {
    let command = parse_cli_env_args().run();

    let header_map: HeaderMappings = {
        let deserializer = &mut serde_json::Deserializer::from_reader(command.header_map.open()?);
        serde_path_to_error::deserialize(deserializer)?
    };

    let json_records: JsonRecords = {
        let deserializer = &mut serde_json::Deserializer::from_reader(command.input.open()?);
        serde_path_to_error::deserialize(deserializer)?
    };

    let mut csv_writer = csv::Writer::from_writer(command.output.create(true)?);

    dbg!(json_records.len());

    csv_writer.write_record(header_map.keys())?;
    let mut fields = Vec::with_capacity(header_map.keys().len());

    for record in &json_records {
        for mapper in header_map.values() {
            fields.push(
                mapper
                    .resolve(record)
                    .and_then(|cell_val| serde_json::to_string(&cell_val))?,
            );
        }
        csv_writer.write_record(&fields)?;
        fields.clear();
    }

    Ok(())
}
