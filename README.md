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
|    1x1 |     1x1 | Some silly base cases...                 |
|    3x3 |     3x3 | Some silly base cases...                 |
|    6x6 |     3x2 | Asymetric, wide (more columns than rows) |
|  12x12 |     4x3 | Asymetric, wide (more columns than rows) |


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
