#![expect(clippy::absolute_paths)]

fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_EO_LINES,
        rubik::solvers::zz::NUM_EO_LINES,
        rubik::puzzles::Cube::eo_line_index,
        &rubik::r#move::MOVES,
    )
}
