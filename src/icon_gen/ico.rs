use ico_builder::IcoBuilder;

use super::definition::AppIconGenerator;

impl AppIconGenerator<'_> {
    pub fn build_ico(&self) {
        IcoBuilder::default()
            .sizes(&[16, 32, 48, 96, 256])
            .add_source_file(self.input_file)
            .build_file(&format!("{}.ico", self.out)).expect("Failed to build ICO file.");
    }
}
