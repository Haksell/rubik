mod file_operations;
mod generate_table;

pub use {
    file_operations::{read_moves, write_moves},
    generate_table::generate_table,
};

pub const FILE_CROSSES: &'static str = "tables/cfop/crosses.bin";
pub const FILE_EO_LINES: &'static str = "tables/zz/eo_lines.bin";
pub const FILE_ZZ_LEFT: &'static str = "tables/zz/zz_left.bin";
pub const FILE_ZZ_RIGHT: &'static str = "tables/zz/zz_right.bin";
