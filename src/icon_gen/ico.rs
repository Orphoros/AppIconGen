use ico_builder::IcoBuilder;

use crate::error_exit;

use super::definition::AppIconGenerator;

impl AppIconGenerator<'_> {
    pub fn build_ico(&self) {
        IcoBuilder::default()
            .sizes(self.ico_resolutions)
            .add_source_file(self.input_file)
            .build_file(&format!("{}.ico", self.out)).unwrap_or_else(|_| error_exit!("failed to build ICO file"));
    }
}
