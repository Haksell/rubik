mod file_operations;
mod generate_start_table;

pub use file_operations::read_moves;
pub use file_operations::write_moves;
pub use generate_start_table::generate_start_table;

pub const FILE_CROSSES: &'static str = "tables/cfop/crosses.bin";
pub const FILE_EO_LINES: &'static str = "tables/zz/eo_lines.bin";
pub const FILE_ZZ_LEFT: &'static str = "tables/zz/zz_left.bin";
