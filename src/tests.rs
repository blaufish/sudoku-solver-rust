#[cfg(test)]
mod tests {
    use crate::generator;
    use crate::helpers;
    use crate::solvers;

    const UNICODE_V: &str = "__💩 ___ __🍅
_🌈🍅 _💩_ ___
___ 🍅__ 🍕💩🌈

🎆🎌🌈 🐡_🐙 💩_🍕
🍅🏠🍕 💩🎌🎆 __🐙
_💩_ 🌈_🍕 🎌__

___ 🎌__ 🏠_💩
_🍕_ _🐙_ ___
__🎆 ___ __🐡";

    const UNICODE_E: &str = "🎌🐡💩 🐙🍕🌈 🎆🏠🍅
🍕🌈🍅 🎆💩🏠 🐡🐙🎌
🐙🎆🏠 🍅🐡🎌 🍕💩🌈

🎆🎌🌈 🐡🏠🐙 💩🍅🍕
🍅🏠🍕 💩🎌🎆 🌈🐡🐙
🐡💩🐙 🌈🍅🍕 🎌🎆🏠

🌈🐙🐡 🎌🎆🍅 🏠🍕💩
💩🍕🎌 🏠🐙🐡 🍅🌈🎆
🏠🍅🎆 🍕🌈💩 🐙🎌🐡";

    const SUDOKU6_V: &str = "_13 ___
__6 ___

___ _5_
___ _64

4__ ___
_32 5__";

    const SUDOKU6_E: &str = "513 426
246 315

164 253
325 164

451 632
632 541";

    const SUDOKU9_V: &str = "_7_ ___ 342
46_ _2_ 7__
32_ _7_ 068

__4 208 __6
6__ 4_1 8__
5__ __6 421

_8_ 34_ ___
___ _1_ 6__
053 ___ _74";

    const SUDOKU9_E: &str = "871 065 342
460 823 715
325 174 068

714 208 536
632 451 807
508 736 421

186 347 250
247 510 683
053 682 174";

    const SUDOKU9_V2: &str = "___ __6 7__
1_5 __8 ___
36_ ___ 42_

_2_ ___ 13_
459 ___ 68_
6__ ___ __9

___ __1 ___
_31 _5_ ___
8__ 3__ _5_";

    const SUDOKU9_E2: &str = "982 436 715
145 728 396
367 519 428

728 695 134
459 173 682
613 842 579

576 281 943
231 954 867
894 367 251";

    const SUDOKU9_V3: &str = "6_1 __2 __4
_8_ ___ ___
_7_ 8_5 _3_

7_9 _3_ ___
___ ___ 9__
4__ ___ _58

__8 5__ 2__
_43 9__ _17
___ ___ _4_";

    const SUDOKU9_E3: &str = "691 372 584
385 614 729
274 895 136

759 238 461
836 451 972
412 769 358

168 547 293
543 926 817
927 183 645";

    const SUDOKU25_V: &str = "_9_c_ __f_g mh__e _jak_ 6lb__
ln_4e __d__ 1_983 2____ ____k
__2mo jl___ gk_p6 3____ 8feah
3h_ja _k87o 5f_40 ___d_ __g1n
8f6_k __pa_ n____ ___m7 ___3d

_l___ o85kj bgp_7 _m4__ __0d3
_d7__ _f09_ e__j_ _1_2_ n___4
_e__0 ___h_ __m_a d___j l_fkb
mg_f3 ___ln 6__5h __k__ j____
kj_2_ _d___ _n_l1 5____ __h__

b_0__ 1ea_4 82lhn _opj_ d53_m
__k__ __90_ _pb_5 4_7l8 __a_o
ao__4 k_jb_ 7d6_g hc_51 298_0
5_j_m co7_f _1e09 n_2_a ___lg
__cd_ h_2__ a3_o_ 9g_bm e_4n7

__91j _he__ p_n__ _0f3d _clb8
c___p n9b__ jl43_ _8ma_ _k1e2
_ke_6 d1__7 h_o_8 ___p_ _3m_9
____2 __ofp ce_a_ k__9_ g__h6
gb_a8 _63__ 09_k_ 7e_c_ p__4_

6_4__ ___n8 lm_9o ab__2 __p_f
___o_ _____ _0___ j5dg_ 4___l
__g_n _a___ _6___ _____ ke_9_
0cm8l f_h__ 2j_n_ p__e9 3___1
215_7 _p4__ d_aef 0____ ___8c";

    const SUDOKU25_E: &str = "791c5 30f4g mh2de 8jakn 6lbop
lnp4e 6bdmh 1a983 2fgo0 7jc5k
d02mo jln15 gk7p6 39b4c 8feah
3hbja 2k87o 5fc40 e6ldp 9mg1n
8f6gk 9cpae nojbl 1h5m7 0423d

9lah1 o85kj bgpf7 6m4ne c20d3
pd76c bf09a e83jk l1h2o ng5m4
4en50 p3gh6 ocm2a d798j l1fkb
mgof3 721ln 64d5h cak0b j89pe
kj82b 4dmec 9n0l1 5p3fg o7h6a

b709g 1ea64 82lhn fopjk d53cm
e2knh mg90d fpbc5 437l8 16ajo
aolp4 knjb3 7d6mg hce51 298f0
58j3m co7pf 41e09 nd26a bhklg
16cdf h528l a3koj 9g0bm ep4n7

o491j ahe2k p7n6m g0f3d 5clb8
c5h7p n9bg0 jl43d o8ma6 fk1e2
fkel6 d1cj7 h5og8 b2np4 a3m09
nm302 84ofp ce1ab klj95 gd7h6
gbda8 l635m 09fk2 7e1ch pno4j

634ed 5jkn8 lmg9o abc12 h0p7f
hafo9 em6c1 k087p j5dg3 4bn2l
jpgbn 0alo2 36h1c m487f ked95
0cm8l f7hdb 2j5n4 pkoe9 3a6g1
215k7 gp439 dbaef 0n6hl moj8c";

    fn process(testvector: String, strat: Option<&str>) -> String {
        let mut sudoku;
        match helpers::parse(testvector.clone()) {
            Ok(s) => sudoku = s,
            Err(_) => return String::from(""),
        }
        let solved = solvers::solve(&mut sudoku, strat);
        assert_eq!(true, solved, "error solving: {}", testvector);
        let (valid, errs) = sudoku.validate();
        if !valid {
            eprintln!("Failed to solve: {}", testvector);
            eprintln!("Invalid solution: {}", sudoku.to_string());
        }
        for (row, col, reason) in errs {
            eprintln!("Error! row:{} col:{} reason:{}", row, col, reason);
        }
        assert!(valid);
        sudoku.to_string()
    }

    #[test]
    fn test_all_solvers() {
        for solver in solvers::list_solvers() {
            let vector = SUDOKU9_V.to_string();
            let expected = SUDOKU9_E;
            let strategy: Option<&str> = Some(&solver);
            let actual = process(vector, strategy);
            assert_eq!(expected, actual, "failed solving with: {}", solver);
        }
    }

    #[test]
    fn test_solve_6x6() {
        let vector = SUDOKU6_V;
        let expected = SUDOKU6_E;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_9x9() {
        let vector = SUDOKU9_V;
        let expected = SUDOKU9_E;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_9x9_2() {
        let vector = SUDOKU9_V2;
        let expected = SUDOKU9_E2;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_9x9_3() {
        let vector = SUDOKU9_V3;
        let expected = SUDOKU9_E3;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_9x9_unicode() {
        let vector = UNICODE_V;
        let expected = UNICODE_E;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_25x25() {
        let vector = SUDOKU25_V;
        let expected = SUDOKU25_E;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    fn generator_test(generator: &generator::Generator) {
        let valid = generator.validate_generator();
        assert!(valid);

        println!("Generated golden soluton:");
        let some_golden = generator::generate_golden(generator);
        let golden;
        if let Some(g) = some_golden {
            golden = g.clone();
            let (valid_golden, errors) = g.validate();
            for (row, col, s) in errors {
                println!("Error: {} {} {}", row, col, s);
            }
            assert!(valid_golden);
        } else {
            assert!(false, "Did not generate any initial solution!");
            return;
        }
        println!("{}", golden.to_string());

        let some_challenge = generator::generate_challenge(generator, &golden);
        let challenge;
        if let Some(c) = some_challenge {
            challenge = c;
        } else {
            assert!(false, "Did not generate any challenge sudoku!");
            return;
        }
        println!("Generated challenge:");
        println!("{}", challenge.to_string());

        let mut sudoku = challenge.clone();
        let solved = solvers::solve(&mut sudoku, None);
        assert!(solved);
        println!("Generated Solution:");
        println!("{}", sudoku.to_string());
        {
            let (valid_solution, errors) = &sudoku.validate();
            for (row, col, s) in errors {
                println!("Error: {} {} {}", row, col, s);
            }
            assert!(valid_solution);
        }
        assert_eq!(golden.to_string(), sudoku.to_string());
    }

    #[test]
    fn test_gen_6x6() {
        for _i in 0..10 {
            println!("Test iteration: {} ", _i);
            let generator = generator::Generator {
                dimensions: 6,
                grid_width: 3,
                grid_height: 2,
                charset: "123456".to_string(),
                max_prune_seconds: 10,
            };
            generator_test(&generator);
        }
    }

    #[test]
    fn test_gen_9x9() {
        for _i in 0..10 {
            println!("Test iteration: {} ", _i);
            let generator = generator::Generator {
                dimensions: 9,
                grid_width: 3,
                grid_height: 3,
                charset: "123456789".to_string(),
                max_prune_seconds: 10,
            };
            generator_test(&generator);
        }
    }
}
