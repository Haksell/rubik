fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_ZZ_LEFT,
        rubik::solvers::NUM_ZZ_LEFT,
        rubik::Cube::zz_left_index,
        &rubik::puzzles::MOVES_RUL,
    )
}
