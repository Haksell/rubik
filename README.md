# rubik

## todo

- [ ] 3x3 visualisation (vegret)
- [ ] pyraminx mooves + iddfs + visualisation (vegret)
- [ ] big cube moves + visualisation (vegret)
- [ ] `solve_cross`, `solve_eo_line`... = `solve_step_from_file` (axbrisse)
- [ ] generate or find optimal ZBLL (axbrisse)
- [ ] random move scrambler without redundant moves
- [ ] better `reduce_moves` (`R L R L'`-> `R2`)

## features

- [x] 3x3x3 cfop solver
- [ ] 3x3x3 zb solver (axbrisse)
- [ ] 3x3x3 zz solver (axbrisse)
- [ ] 3x3x3 petrus solver (axbrisse)
- [ ] 3x3x3 thistlethwaite/kloosterman/pochmann solver (axbrisse)
- [ ] 3x3x3 reid solver (axbrisse)
- [ ] 3x3x3 kociemba solver
- [ ] 3x3x3 korf solver
- [x] 2x2x2 iddfs solver
- [ ] pyraminx iddfs solver (vegret)
- [ ] big cube solver
- [ ] megaminx solver
- [ ] step explainer
- [ ] random move scrambler (vegret)
- [ ] random state scrambler (vegret)
- [ ] 3d visualization (vegret)

## before push

- [ ] remove `#[allow(dead_code)]`
- [ ] remove useless `pub` and use `pub(crate)`/`pub(super)` where appropriate

## resources

- https://kociemba.org/cube.htm
- https://github.com/sebcrozet/kiss3d
- https://www.youtube.com/watch?v=9PGfL4t-uqE
- https://en.wikipedia.org/wiki/Optimal_solutions_for_the_Rubik%27s_Cube
- Kloosterman explanation : https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/michael_reid__an_upper_bound_on_god%27s_number.html
- Reid explanation : https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/michael_reid__an_upper_bound_on_god%27s_number.html
- Kociemba symmetry : https://stackoverflow.com/a/70159792
- Korf paper : https://www.cs.princeton.edu/courses/archive/fall06/cos402/papers/korfrubik.pdf
- Korf opti pattern databases : https://cdn.aaai.org/AAAI/2005/AAAI05-219.pdf
- Korf explanation : https://github.com/benbotto/rubiks-cube-cracker
- Korf explanation : https://medium.com/@benjamin.botto/implementing-an-optimal-rubiks-cube-solver-using-korf-s-algorithm-bf750b332cf9
- Kociemba explanation : https://www.jaapsch.net/puzzles/compcube.htm#kocal
- *Analyzing the Rubik's Cube Group of Various Sizes and Solutions* : https://math.uchicago.edu/~may/REU2021/REUPapers/Chuang,Alex.pdf#page=17&zoom=100,169,306
- *Group Theory and the Rubik's Cube* : https://people.math.harvard.edu/~jjchen/docs/Group%20Theory%20and%20the%20Rubik's%20Cube.pdf
