fn main() -> std::io::Result<()> {
    rubik::generate_table(
        rubik::files::FILE_EO_LINES,
        rubik::solvers::NUM_EO_LINES,
        rubik::Cube::eo_line_index,
    )
}
