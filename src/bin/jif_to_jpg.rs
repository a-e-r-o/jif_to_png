use jif_to_jpg::OutputFormat;

fn main() {
    jif_to_jpg::convert_all(OutputFormat::Jpg(95));
}
