#[cfg(test)]
mod tests {
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
        match helpers::parse(testvector) {
            Ok(s) => sudoku = s,
            Err(_) => return String::from(""),
        }
        let _ = solvers::solve(&mut sudoku, strat);
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
    fn test_unicode() {
        let vector = UNICODE_V;
        let expected = UNICODE_E;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_25x25() {
        let vector = SUDOKU25_V;
        let expected = SUDOKU25_E;
        let actual = process(vector.to_string(), None);
        assert_eq!(expected, actual);
    }
}
