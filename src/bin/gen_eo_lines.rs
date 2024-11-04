fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_EO_LINES,
        rubik::solvers::NUM_EO_LINES,
        rubik::Cube::eo_line_index,
        &rubik::puzzles::MOVES,
    )
}
