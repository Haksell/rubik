fn main() -> std::io::Result<()> {
    rubik::tables::generate_table(
        rubik::tables::FILE_ZZ_RIGHT,
        rubik::solvers::NUM_ZZ_RIGHT,
        rubik::Cube::zz_right_index,
        &rubik::puzzles::MOVES_RU,
    )
}
