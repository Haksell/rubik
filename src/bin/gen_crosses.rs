fn main() -> std::io::Result<()> {
    rubik::generate_table(
        rubik::files::FILE_CROSSES,
        rubik::solvers::NUM_CROSSES,
        rubik::Cube::cross_index,
    )
}
