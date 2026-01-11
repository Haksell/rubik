#![expect(clippy::absolute_paths)]

fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_CROSSES,
        rubik::solvers::cfop::NUM_CROSSES,
        rubik::puzzles::Cube::cross_index,
        &rubik::r#move::MOVES,
    )
}
