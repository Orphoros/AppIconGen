use ico_builder::IcoBuilder;

pub fn build_ico(input_file: &str, out: &str) {
    IcoBuilder::default()
        .sizes(&[16, 32, 48, 96, 256])
        .add_source_file(input_file)
        .build_file(&format!("{}.ico", out)).unwrap();
}