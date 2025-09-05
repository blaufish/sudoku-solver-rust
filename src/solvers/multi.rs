use crate::sudoku;
//use crate::sudoku::MAX_DIMENSIONS;
//use crate::sudoku::MAX_GRID_DIMENSIONS;
use crate::table::Table;

pub fn solve(sudoku: &mut sudoku::Sudoku) -> bool {
    let mut vec: Vec<sudoku::Sudoku> = Vec::new();
    vec.push(sudoku.clone());
    let success = solve_inner(&mut vec);
    if success {
        let sudoku2 = &vec[vec.len() - 1];
        for row in 0..sudoku.dimensions {
            for col in 0..sudoku.dimensions {
                sudoku.board[row][col] = sudoku2.board[row][col];
            }
        }
    }
    success
}

fn solve_inner(vec: &mut Vec<sudoku::Sudoku>) -> bool {
    println!("solve_inner( len {} )", vec.len());
    if vec.len() == 0 {
        return false;
    }
    let mut sudoku = vec[vec.len() - 1].clone();
    let check = pre(&mut sudoku);
    match check {
        PreCheckValue::Failed => return false,
        PreCheckValue::Completed => return true,
        PreCheckValue::NotCompleted => (),
    }
    println!("solve_inner...");
    let Some((lookup_row_col, attacks)) = gimme_attacks(&sudoku) else {
        return false;
    };
    println!("solve_inner... 2");
    println!("lookup_row_col.len = {}", lookup_row_col.len());
    println!("attacks.len = {}", attacks.len());
    for attack in attacks {
        for i in 0..attack.len() {
            let (row, col) = lookup_row_col[i];
            sudoku.board[row][col] = attack[i];
            println!("solve_inner {} s[{}][{}] = {}", i, row, col, attack[i]);
        }
        vec.push(sudoku.clone());
        if solve_inner(vec) {
            return true;
        }
        _ = vec.pop();
    }
    false
}

#[derive(Clone)]
enum AttackType {
    Row,
    Col,
    //Grid,
}

struct Attack {
    attack_type: AttackType,
    attack_position: usize,
    complexity: u64,
}

fn generate_attack_list(sudoku: &sudoku::Sudoku, table: &Table, attack: &Attack) -> Vec<Vec<u32>> {
    let tmp: Vec<u32> = Vec::new();

    let debug;
    match attack.attack_type {
        AttackType::Row => debug = "row",
        AttackType::Col => debug = "col",
    }
    println!(
        "Generate attack list for: {} {}",
        debug, attack.attack_position
    );
    generate_attack_list_inner(sudoku, table, attack, tmp, 0)
}

fn generate_attack_list_inner(
    sudoku: &sudoku::Sudoku,
    table: &Table,
    attack: &Attack,
    tmp: Vec<u32>,
    position: usize,
) -> Vec<Vec<u32>> {
    //println!("generate_attack_list_inner position {} tmp.len {}", position, tmp.len());
    let mut vec: Vec<Vec<u32>> = Vec::new();
    if position >= sudoku.dimensions {
        //println!("Push: {}", tmp.len());
        vec.push(tmp);
        return vec;
    }
    let row;
    let col;
    match attack.attack_type {
        AttackType::Row => {
            row = attack.attack_position;
            col = position;
        }
        AttackType::Col => {
            row = position;
            col = attack.attack_position;
        }
        /*AttackType::Grid => todo!(),*/
    }
    if sudoku.board[row][col] != 0 {
        return generate_attack_list_inner(sudoku, table, attack, tmp, position + 1);
    }
    //println!(" Calculate options for {} {}", row, col);
    let utilized = calculate_utilized(sudoku, table, row, col);
    //println!(" utilized: {}", utilized);
    let mut binary: u32 = 1;
    for _i in 0..31 {
        //println!("  {} & {} = {}", binary, utilized, binary & utilized);
        let check = binary & utilized;
        binary = binary << 1;
        if check == 0 {
            continue;
        }
        //println!("  ...{}", _i);
        let mut list = tmp.clone();
        //println!("  Append binary {} to list", binary);
        list.push(binary);
        //println!("  list.len(): {}", list.len());

        let mut vec2 = generate_attack_list_inner(sudoku, table, attack, list, position + 1);
        vec.append(&mut vec2);
    }
    //println!("generate_attack_list_inner position {} default return, len {}", position, vec.len());
    vec
}

fn gimme_attacks(sudoku: &sudoku::Sudoku) -> Option<(Vec<(usize, usize)>, Vec<Vec<u32>>)> {
    let mut table = Table::new();
    table.populate(sudoku);
    let mut lookup_table: Vec<(usize, usize)> = Vec::new();
    let attack_outer;
    match select_attack(sudoku, &table) {
        None => return None,
        Some(attack) => {
            match attack.attack_type {
                AttackType::Row => {
                    for col in 0..sudoku.dimensions {
                        if sudoku.board[attack.attack_position][col] == 0 {
                            lookup_table.push((attack.attack_position, col));
                        }
                    }
                }
                AttackType::Col => {
                    for row in 0..sudoku.dimensions {
                        if sudoku.board[row][attack.attack_position] == 0 {
                            lookup_table.push((row, attack.attack_position));
                        }
                    }
                }
            }
            attack_outer = attack;
        }
    }
    let attacks = generate_attack_list(&sudoku, &table, &attack_outer);
    Some((lookup_table, attacks))
}

fn select_attack(sudoku: &sudoku::Sudoku, table: &Table) -> Option<Attack> {
    let mut attack: Option<Attack> = None;
    static ATTACK_TYPES: [AttackType; 2] =
        [AttackType::Row, AttackType::Col /*, AttackType::Grid*/];
    for attack_type in ATTACK_TYPES.clone() {
        for position in 0..sudoku.dimensions {
            let new_complexity =
                calculate_attack_complexity(attack_type.clone(), position, sudoku, table);
            if new_complexity <= 1 {
                continue;
            }
            let new_attack: Attack = Attack {
                attack_type: attack_type.clone(),
                attack_position: position,
                complexity: new_complexity,
            };
            match attack {
                None => attack = Some(new_attack),
                Some(ref old_attack) => {
                    if new_complexity > old_attack.complexity {
                        attack = Some(new_attack);
                    }
                }
            }
        }
    }
    attack
}

fn calculate_attack_complexity(
    attack_type: AttackType,
    position: usize,
    sudoku: &sudoku::Sudoku,
    table: &Table,
) -> u64 {
    let mut complexity: u64 = 1;
    match attack_type {
        AttackType::Row => {
            for col in 0..sudoku.dimensions {
                if sudoku.board[position][col] != 0 {
                    continue;
                }
                let possible = calculate_number_of_possible_values(sudoku, table, position, col);
                let counted_possible_values_of_cell: u64 = possible.into();
                complexity = complexity * counted_possible_values_of_cell;
            }
        }
        AttackType::Col => {
            for row in 0..sudoku.dimensions {
                if sudoku.board[row][position] != 0 {
                    continue;
                }
                let possible = calculate_number_of_possible_values(sudoku, table, row, position);
                let counted_possible_values_of_cell: u64 = possible.into();
                complexity = complexity * counted_possible_values_of_cell;
            }
        }
        /*
        AttackType::Grid => {
            let grid_row = position / sudoku.grid_width;
            let grid_col = position % sudoku.grid_width;
            for r in 0..sudoku.grid_height {
                for c in 0..sudoku.grid_width {
                    let row = grid_row + r;
                    let col = grid_col + c;
                    if sudoku.board[row][col] != 0 {
                        continue;
                    }
                    let possible = calculate_number_of_possible_values(sudoku, table, row, position);
                    let counted_possible_values_of_cell : u64 = possible.into();
                    complexity = complexity * counted_possible_values_of_cell;
                }
            }
        }
        */
    }
    complexity
}

fn bincnt(bin: u32) -> u32 {
    let mut cnt = 0;
    let mut b = bin;
    for _i in 0..31 {
        if b & 1 == 1 {
            cnt = cnt + 1;
        }
        b = b >> 1;
    }
    cnt
}

enum PreCheckValue {
    Completed,
    Failed,
    NotCompleted,
}

fn calculate_utilized(sudoku: &sudoku::Sudoku, table: &Table, row: usize, col: usize) -> u32 {
    let grid_row = row / sudoku.grid_height;
    let grid_col = col / sudoku.grid_width;
    let utilized_grid = table.grids[grid_row][grid_col];
    let utilized_row = table.rows[row];
    let utilized_col = table.cols[col];
    let utilized = utilized_row | utilized_col | utilized_grid;
    utilized
}

fn calculate_not_utilized(sudoku: &sudoku::Sudoku, table: &Table, row: usize, col: usize) -> u32 {
    let utilized = calculate_utilized(sudoku, table, row, col);
    let xor_pattern: u32 = (1 << sudoku.character_set.chars().count()) - 1;
    let inverted = utilized ^ xor_pattern;
    return inverted;
}

fn calculate_number_of_possible_values(
    sudoku: &sudoku::Sudoku,
    table: &Table,
    row: usize,
    col: usize,
) -> u32 {
    let not_utilized_bits = calculate_not_utilized(sudoku, table, row, col);
    let count = bincnt(not_utilized_bits);
    count
}

fn pre(sudoku: &mut sudoku::Sudoku) -> PreCheckValue {
    let mut table = Table::new();
    let xor_pattern: u32 = (1 << sudoku.character_set.chars().count()) - 1;
    table.populate(sudoku);
    loop {
        let mut done = true;
        let mut all_done = true;
        for row in 0..sudoku.dimensions {
            for col in 0..sudoku.dimensions {
                if sudoku.board[row][col] != 0 {
                    continue;
                }
                let utilized = calculate_utilized(&sudoku, &table, row, col);
                let inverted = utilized ^ xor_pattern;
                match bincnt(inverted) {
                    0 => return PreCheckValue::Failed,
                    1 => {
                        let grid_row = row / sudoku.grid_height;
                        let grid_col = col / sudoku.grid_width;
                        println!("Fill in: {} {}", row, col);
                        sudoku.board[row][col] = inverted;

                        table.rows[row] ^= inverted;
                        table.cols[col] ^= inverted;
                        table.grids[grid_row][grid_col] ^= inverted;
                        done = false;
                    }
                    _ => all_done = false,
                }
            }
        }
        if all_done {
            return PreCheckValue::Completed;
        }
        if done {
            return PreCheckValue::NotCompleted;
        }
    }
}
