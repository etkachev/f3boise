use crate::shared::common_errors::AppError;

pub mod ao_monthly_leaderboard;
pub mod ao_pax_leaderboard;
pub mod overall_pax_leaderboard;
pub mod pax_bd_overtime;

/// Trait for building graph with common methods like generating chart and declaring file name.
pub trait GraphWrapper {
    const WIDTH: u32;
    const HEIGHT: u32;
    /// this should Generate a Chart::new() and call save() method
    fn generate_chart(&self) -> Result<(), String>;
    /// return custom file name (without .svg) extension.
    fn file_name(&self) -> String;

    /// convert local svg file to png bytes. Can be used to upload to slack channels
    fn convert_svg(&self) -> Result<Vec<u8>, AppError> {
        let file = std::fs::read(self.file_path())?;
        let options = resvg::usvg::Options::default();

        let mut fontdb = resvg::usvg::fontdb::Database::new();
        fontdb.load_system_fonts();
        fontdb.load_fonts_dir("./assets/fonts/");
        let tree = resvg::usvg::Tree::from_data(&file, &options, &fontdb)?;
        // tree.convert_text(&fontdb);

        let mut pixmap = resvg::tiny_skia::Pixmap::new(self.width(), self.height()).unwrap();
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::default(),
            &mut pixmap.as_mut(),
        );
        // pixmap.save_png("test.png").unwrap();
        let result = pixmap.encode_png().unwrap();
        // once converted, then also delete temporary file.
        std::fs::remove_file(self.file_path())?;
        Ok(result)
    }

    fn file_path(&self) -> String {
        format!("{}.svg", self.file_name())
    }

    fn width(&self) -> u32 {
        Self::WIDTH
    }

    fn height(&self) -> u32 {
        Self::HEIGHT
    }
}

/// pass in graph wrapper implementation to generate svg then convert svg to png.
pub fn graph_generator(graph: impl GraphWrapper) -> Result<Vec<u8>, AppError> {
    graph.generate_chart()?;
    graph.convert_svg()
}
