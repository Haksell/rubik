# rubik

## todo

- [ ] associated types for `Puzzle` (`Cube<2>` and `Cube<3>` are different)
  - [ ] `Move`
  - [ ] `Sticker`
  - [ ] `Color` (rename to `Face`)
- [ ] moving 3x3 visualization (vegret)
- [ ] mirror blocks visualization (vegret)
- [ ] big cube moves + visualisation (vegret)
- [ ] antialiasing (vegret)
- [ ] `solve_cross`, `solve_eo_line`... = `solve_step_from_file` (axbrisse)
- [ ] generate or find optimal ZBLL/ZBLS (axbrisse)
- move reduction (axbrisse):
  - [ ] random move scrambler without redundant moves
  - [ ] better `reduce_moves` (`R L R L'`-> `R2`)
- [ ] `moves!` -> `&'static [Move]` (vegret)
- [ ] x-cross flag for cfop/zb (axbrisse)

## features

- [x] 3x3x3 cfop solver
- [ ] 3x3x3 zz solver (axbrisse) (zbll remaining)
- [ ] 3x3x3 zb solver (axbrisse) (zbls/zbll remaining)
- [ ] 3x3x3 petrus solver (axbrisse)
- [ ] 3x3x3 thistlethwaite/kloosterman/pochmann solver (axbrisse)
- [ ] 3x3x3 reid solver (axbrisse)
- [ ] 3x3x3 kociemba solver (axbrisse)
- [ ] 3x3x3 korf solver (vegret)
- [x] 2x2x2 iddfs solver
- [x] pyraminx iddfs solver
- [ ] big cube solver (axbrisse)
- [ ] megaminx solver (axbrisse)
- [ ] step explainer (vegret)
- [x] random move scrambler
- [ ] random state scrambler for 2x2/3x3(requires kociemba)/pyra (vegret)
- [ ] 3d visualization with animation (vegret)

## resources

- https://kociemba.org/cube.htm
- https://en.wikipedia.org/wiki/Optimal_solutions_for_the_Rubik%27s_Cube
- Kloosterman explanation : https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/michael_reid__an_upper_bound_on_god%27s_number.html
- Pochmann code : https://www.stefan-pochmann.info/spocc/other_stuff/tools/solver_thistlethwaite/solver_thistlethwaite_cpp.txt
- Reid explanation : https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/michael_reid__an_upper_bound_on_god%27s_number.html
- Kociemba symmetry : https://stackoverflow.com/a/70159792
- Korf paper : https://www.cs.princeton.edu/courses/archive/fall06/cos402/papers/korfrubik.pdf
- Korf opti pattern databases : https://cdn.aaai.org/AAAI/2005/AAAI05-219.pdf
- Korf explanation : https://github.com/benbotto/rubiks-cube-cracker
- Korf explanation : https://medium.com/@benjamin.botto/implementing-an-optimal-rubiks-cube-solver-using-korf-s-algorithm-bf750b332cf9
- Kociemba explanation : https://www.jaapsch.net/puzzles/compcube.htm#kocal
- *Analyzing the Rubik's Cube Group of Various Sizes and Solutions* : https://math.uchicago.edu/~may/REU2021/REUPapers/Chuang,Alex.pdf#page=17&zoom=100,169,306
- *Group Theory and the Rubik's Cube* : https://people.math.harvard.edu/~jjchen/docs/Group%20Theory%20and%20the%20Rubik's%20Cube.pdf
- Θ(n²/log(n)) god's number : https://arxiv.org/pdf/1106.5736
