mod cli_io;
mod effects;
mod errors;
mod header_mappings;

use bpaf::Bpaf;

use self::cli_io::{Input, Output};
use self::header_mappings::HeaderMappings;

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
    let Command {
        input,
        header_map,
        mut output,
    } = parse_cli_env_args().run();

    if let (Output::Directory(path), Some(input_file_name)) = (&output, input.file_name()) {
        output = Output::File(path.join(input_file_name).with_extension("csv"))
    }

    let header_map: HeaderMappings = {
        let deserializer = &mut serde_json::Deserializer::from_reader(header_map.open()?);
        serde_path_to_error::deserialize(deserializer)?
    };

    let json_records: JsonRecords = {
        let deserializer = &mut serde_json::Deserializer::from_reader(input.open()?);
        serde_path_to_error::deserialize(deserializer)?
    };

    let mut csv_writer = csv::Writer::from_writer(output.create(true)?);

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
