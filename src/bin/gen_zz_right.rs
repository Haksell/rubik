#![expect(clippy::absolute_paths)]

fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_ZZ_RIGHT,
        rubik::solvers::zz::NUM_ZZ_RIGHT,
        rubik::puzzles::Cube::zz_right_index,
        &rubik::r#move::MOVES_RU,
    )
}
