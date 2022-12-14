#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use slint::{Model, VecModel, SharedString};
use std::rc::Rc;

slint::include_modules!();

const SIZE: i32 = 81;
const NUM_ROWS: i32 = 9;
const NUM_COLS: i32 = 9;
const BOMB: u8 = b"*"[0];

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let ui = App::new();
    let mut num_to_win: i32 = SIZE;

    ui.set_bombfield_row(NUM_ROWS);
    ui.set_bombfield_col(NUM_COLS);

    // get a new random bomb field
    let field_rows: usize = ui.get_bombfield_row().try_into().unwrap();
    let field_cols: usize = ui.get_bombfield_col().try_into().unwrap();
    let answers: Vec<u8> = create(field_rows, field_cols);

    let mut bomb_field: Vec<BombData> = ui.get_bombfield_list().iter().collect();

    for tile in answers.iter() {
        let icon: String;
        let is_bomb: bool;

        if tile.eq(&BOMB) {
            icon    = "💣".to_string();
            is_bomb = true;
            num_to_win -= 1;
        }
        else {
            icon    = tile.escape_ascii().to_string();
            is_bomb = false;
        }

        bomb_field.push( BombData { 
            icon: SharedString::from(icon),
            is_bomb: is_bomb, 
        });
    }

    let field = Rc::new(VecModel::from(bomb_field));
    ui.set_bombfield_list(field.clone().into());
    ui.set_num_to_win(num_to_win);

    ui.run();
}

fn create(row: usize, col: usize) -> Vec<u8> {
    let space: u8 = b" "[0];
    let one: u8 = b"1"[0];
    
    let mut field: Vec<u8> = Vec::new();
    let mut ans: Vec<u8> = vec![space; SIZE as usize];
    
    // create new random field
    for i in 0..SIZE {
        let mut seeds = [0u8; SIZE as usize];
        getrandom::getrandom(&mut seeds).unwrap();
        let mut rng = oorandom::Rand32::new(seeds[i as usize] as u64);

        match rng.rand_range(0..10) {
            0 => field.push(BOMB),
            _ => field.push(space),
        }
    }

    let col_size: i32 = col.try_into().unwrap();
    let field_size: i32 = SIZE.try_into().unwrap();
    
    for (i, field_pt) in field.iter().enumerate() {
        if field_pt.eq(&BOMB) {
            ans[i] = BOMB;
            
            let index: i32 = i.try_into().unwrap();
            let mut affected_pts: Vec<i32> = Vec::new();
            
            // always try to add directly above and below in same col
            affected_pts.push(index - col_size); // below
            affected_pts.push(index + col_size); // above
            
            // is there col to left?
            if index % col_size != 0 {
                affected_pts.push(index-1 - col_size); // below col to left
                affected_pts.push(index-1 + col_size); // above col to left
                affected_pts.push(index-1); // above col to left
            }
            
            // is there col to right?
            if index % col_size != (col_size-1) {
                affected_pts.push(index+1 - col_size); // below col to left
                affected_pts.push(index+1 + col_size); // above col to left
                affected_pts.push(index+1); // above col to left
            }
            
            for pt in affected_pts {
                if pt >= 0 && pt < field_size {
                    let upt: usize = pt.try_into().unwrap();
                    
                    if ans[upt] == space {
                        ans[upt] = one;
                    }
                    else if ans[upt] >= one {
                        ans[upt] += 1;
                    }
                }
            }
        }
    }
        
    // println!("{:?}", field);
    // println!("{:?}", ans);

    let mut index = 0;
    for _ in 0..row {
        for _ in 0..col {
            print!("{:?} ", &ans[index].escape_ascii().to_string());
            index += 1;
        }
        println!();
    }

    ans
}