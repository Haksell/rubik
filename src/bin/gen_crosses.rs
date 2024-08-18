fn main() -> std::io::Result<()> {
    rubik::tables::generate_start_table(
        rubik::tables::FILE_CROSSES,
        rubik::solvers::NUM_CROSSES,
        rubik::Cube::cross_index,
    )
}
