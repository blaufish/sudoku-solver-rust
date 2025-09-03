A simple sudoku solver, ultra beta quality...

Supports arbitrary character sets, e.g. 1-9, 0-9, 0-9A-F.
Or whatever, parser attempts to understand anything.
Text files must represent unknown/unset as either `_` or `.`.

Supports 9x9 mode and less common modes.
Supported modes:

| Sudoku | Subgrid | Note                                     |
| -----: | ------: | ----                                     |
|    4x4 |     2x2 | Symetric                                 |
|    9x9 |     3x3 | Symetric                                 |
|  16x16 |     4x4 | Symetric                                 |
|  25x25 |     5x5 | Symetric                                 |
|    1x1 |     1x1 | Some silly base cases...                 |
|    3x3 |     3x3 | Some silly base cases...                 |
|    6x6 |     3x2 | Asymetric, wide (more columns than rows) |
|  12x12 |     4x3 | Asymetric, wide (more columns than rows) |
|  20x20 |     5x4 | Asymetric, wide (more columns than rows) |

## Usage

### Usage: Help

`sudoku-solver-rust --help`

``` plain
./target/release/sudoku-solver-rust --help
Usage: sudoku-solver-rust [OPTIONS]

Options:
  -f, --file <FILE>
      --solve
      --generate
      --generate-size <GENERATE_SIZE>                            [default: 9]
      --generate-grid-width <GENERATE_GRID_WIDTH>                [default: 3]
      --generate-grid-height <GENERATE_GRID_HEIGHT>              [default: 3]
      --generate-charset <GENERATE_CHARSET>                      [default: 123456789]
      --generate-max-prune-seconds <GENERATE_MAX_PRUNE_SECONDS>  [default: 20]
      --generate-count <GENERATE_COUNT>                          [default: 1]
      --solve-strategy <SOLVE_STRATEGY>
  -h, --help                                                     Print help
  -V, --version                                                  Print version
```

### Usage: Solving a 9x9 sudoku with unicode

`sudoku-solver-rust --file samples/challenge9x9_3.txt --solve`

``` plain
File contents:
__💩_____🍅
_🌈🍅_💩____
___🍅__🍕💩🌈
🎆🎌🌈🐡_🐙💩_🍕
🍅🏠🍕💩🎌🎆__🐙
_💩_🌈_🍕🎌__
___🎌__🏠_💩
_🍕__🐙____
__🎆_____🐡

H/W: 9 9
subsquare_height: 3
subsquare_width: 3
charset: 💩🍅🌈🍕🎆🎌🐡🐙🏠
__💩 ___ __🍅
_🌈🍅 _💩_ ___
___ 🍅__ 🍕💩🌈

🎆🎌🌈 🐡_🐙 💩_🍕
🍅🏠🍕 💩🎌🎆 __🐙
_💩_ 🌈_🍕 🎌__

___ 🎌__ 🏠_💩
_🍕_ _🐙_ ___
__🎆 ___ __🐡
Time elapsed: 540.706µs
Solved: true
🎌🐡💩 🐙🍕🌈 🎆🏠🍅
🍕🌈🍅 🎆💩🏠 🐡🐙🎌
🐙🎆🏠 🍅🐡🎌 🍕💩🌈

🎆🎌🌈 🐡🏠🐙 💩🍅🍕
🍅🏠🍕 💩🎌🎆 🌈🐡🐙
🐡💩🐙 🌈🍅🍕 🎌🎆🏠

🌈🐙🐡 🎌🎆🍅 🏠🍕💩
💩🍕🎌 🏠🐙🐡 🍅🌈🎆
🏠🍅🎆 🍕🌈💩 🐙🎌🐡
```

### Usage: Solving a 25x25 sudoku

`sudoku-solver-rust --file samples/challenge25x25_3.txt --solve`

``` plain
File contents:
7____ a___0 _3d__ 2____ 9n_hj
__cml _9fe_ ____2 h____ _03k5
_9___ 3_jgh 0a7c8 mn_k6 _____
___e_ _b_n4 _9_p_ __8_5 6_c__
___g1 c_po_ jl5mn 0a__e 2_4__

9____ _0_bo 4j__1 _3___ _6_2c
jo1p_ nd83_ ___5b ____c gh_e9
2_fcn 5g64_ hmp__ _8__j b3_0d
8_35_ _j7__ en_2g _6ad0 k_1o_
_g___ __1__ a_f3o b_7nh __5__

ol_b_ _p__6 _dnf_ _0__a 41e_k
0e4__ __g__ 759_j co___ __db_
3dh_a b2n__ __60p e_4fm __9gl
6p7_9 ____c ____k l_5_2 _j__0
k_n28 17__a m__e_ _b_jp o_65h

1__oe __d_5 _b__a _pn__ _ck62
_6d__ ____n __m_5 _____ h_0_g
4_g__ 7o__j __1_6 _e_ck ___dn
p2_n_ 6__c8 dek__ g_b_o 5__l_
b_kl_ g___1 n__4_ 652__ __m_e

njp_h __41_ b___l 8_c03 _52a_
g_ed_ j_2__ 9___m 5f___ 04bn3
l___f o_5h_ p____ d4ea_ _kg_m
____4 ____e 5_g1_ __6__ _p_9_
___a_ 9___g _4_h_ nmp_b ed_16

H/W: 25 25
subsquare_height: 5
subsquare_width: 5
charset: 7a03d29nhjcmlfek5g86b4p1o
7____ a___0 _3d__ 2____ 9n_hj
__cml _9fe_ ____2 h____ _03k5
_9___ 3_jgh 0a7c8 mn_k6 _____
___e_ _b_n4 _9_p_ __8_5 6_c__
___g1 c_po_ jl5mn 0a__e 2_4__

9____ _0_bo 4j__1 _3___ _6_2c
jo1p_ nd83_ ___5b ____c gh_e9
2_fcn 5g64_ hmp__ _8__j b3_0d
8_35_ _j7__ en_2g _6ad0 k_1o_
_g___ __1__ a_f3o b_7nh __5__

ol_b_ _p__6 _dnf_ _0__a 41e_k
0e4__ __g__ 759_j co___ __db_
3dh_a b2n__ __60p e_4fm __9gl
6p7_9 ____c ____k l_5_2 _j__0
k_n28 17__a m__e_ _b_jp o_65h

1__oe __d_5 _b__a _pn__ _ck62
_6d__ ____n __m_5 _____ h_0_g
4_g__ 7o__j __1_6 _e_ck ___dn
p2_n_ 6__c8 dek__ g_b_o 5__l_
b_kl_ g___1 n__4_ 652__ __m_e

njp_h __41_ b___l 8_c03 _52a_
g_ed_ j_2__ 9___m 5f___ 04bn3
l___f o_5h_ p____ d4ea_ _kg_m
____4 ____e 5_g1_ __6__ _p_9_
___a_ 9___g _4_h_ nmp_b ed_16
Time elapsed: 10.590445499s
Solved: true
75b8p a1m60 k3dge 2cfl4 9nohj
ancml 89fe7 614b2 hjopg d03k5
f9o42 35jgh 0a7c8 mndk6 1elpb
d0jek 2bln4 f9oph 17835 6gcma
h36g1 ckpod jl5mn 0a9be 274f8

9hmk7 e0abo 4j8d1 f3g5l n6p2c
jo1p6 nd83f l705b k2m4c ghae9
2afcn 5g64l hmpk9 o81ej b370d
8435b hj7m9 enc2g p6ad0 kl1of
egl0d pc1k2 a6f3o b97nh jm584

ol5bg mp9j6 2dnf3 70h8a 41eck
0e41m fhgl3 7598j cok6n a2dbp
3dhja b2n5k co60p e14fm 789gl
6p7f9 4eo8c 1hbak ld5g2 mjn30
kcn28 170da mgle4 9b3jp of65h

189oe lmd05 gbhja 4pn7f 3ck62
c6d73 kfe2n opm95 alj18 hb04g
4mgh5 7ob9j 821l6 3e0ck pafdn
p2an0 643c8 dek7f ghbmo 59jl1
bfklj gahp1 n034c 6529d 8om7e

njp9h d641m bkeol 8gc03 f52a7
gkedo j827p 9ca6m 5flh1 04bn3
l106f o35hb p82n7 d4ea9 ckgjm
mb834 0ncae 5fg1d jk627 lph9o
572ac 9lkfg 34jh0 nmpob ed816
```

### Usage: Generating a 25x25 sudoku

`sudoku-solver-rust
 --generate
 --generate-size=25
 --generate-grid-width=5
 --generate-grid-height=5
 --generate-charset=0123456789abcdefghjklmnop
 --generate-max-prune-seconds=2`

``` plain
Generate solution...
Time elapsed: 136.645551ms
Solution:
791c5 30f4g mh2de 8jakn 6lbop
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
215k7 gp439 dbaef 0n6hl moj8c
Generating challenge...
Time elapsed: 7.754562299s
Challenge:
_9_c_ __f_g mh__e _jak_ 6lb__
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
215_7 _p4__ d_aef 0____ ___8c
```

## Implementation details

Internally represents the sudoku board as 2x2 array of `u32`.

``` rust
    pub board: [[u32; 16]; 16],
```

Values are represented as follows:

* `b000` represents unknown/unset.
* `b001` represents first value.
* `b010` represents second value.
* `b100` represents third value.
* ...and so on.

Thus to collect all values set in a subsquare, row, column,
we can just `OR` all cells together.
I thought that was nifty.
