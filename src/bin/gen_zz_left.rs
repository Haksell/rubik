#![expect(clippy::absolute_paths)]

fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_ZZ_LEFT,
        rubik::solvers::zz::NUM_ZZ_LEFT,
        rubik::puzzles::Cube::zz_left_index,
        &rubik::r#move::MOVES_RUL,
    )
}
