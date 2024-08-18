# rubik

## todo

- [ ] big cubes moves / visualisation (vegret)
- [ ] `solve_cross`, `solve_eo_line`... = `solve_step_from_file` (axbrisse)
- [ ] generate or find optimal ZBLL (axbrisse)
- [ ] pyramin visualisation

## features

- [x] 3x3x3 cfop solver
- [ ] 3x3x3 zb solver (axbrisse)
- [ ] 3x3x3 zz solver (axbrisse)
- [ ] 3x3x3 petrus solver (axbrisse)
- [ ] 3x3x3 thistlethwaite/kloosterman/pochmann solver (axbrisse)
- [ ] 3x3x3 reid solver (axbrisse)
- [ ] 3x3x3 kociemba solver (axbrisse)
- [ ] 3x3x3 korf solver (vegret)
- [x] 2x2x2 iddfs solver
- [ ] pyraminx iddfs solver (vegret)
- [ ] big cube solver
- [ ] megaminx solver
- [ ] scrambler (random state/random moves)
- [ ] 3d visualization (vegret)

## before push

- [ ] remove `#[allow(dead_code)]`
- [ ] remove useless `pub` and use `pub(crate)`/`pub(super)` where appropriate

## resources

- https://kociemba.org/cube.htm
- https://github.com/sebcrozet/kiss3d
- https://www.youtube.com/watch?v=9PGfL4t-uqE
- Kloosterman explanation : https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/michael_reid__an_upper_bound_on_god%27s_number.html
- Reid explanation : https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/michael_reid__an_upper_bound_on_god%27s_number.html
- Kociemba symmetry : https://stackoverflow.com/a/70159792
- Korf paper : https://www.cs.princeton.edu/courses/archive/fall06/cos402/papers/korfrubik.pdf
- Korf explanation : https://github.com/benbotto/rubiks-cube-cracker
- https://en.wikipedia.org/wiki/Optimal_solutions_for_the_Rubik%27s_Cube
- Kociemba explanation : https://www.jaapsch.net/puzzles/compcube.htm#kocal
- https://medium.com/@benjamin.botto/implementing-an-optimal-rubiks-cube-solver-using-korf-s-algorithm-bf750b332cf9
