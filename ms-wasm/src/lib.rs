use std::cell::RefCell;

use ms::{board::Board, random};
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
    static MS:RefCell<Board>  = RefCell::new(Board::new(10,10));

}
#[wasm_bindgen]
pub fn main()
{
    MS.with(|instance| {
        let mines = random::random_position_vec(instance.borrow().get_size(), 10);
        instance.borrow_mut().plant_mines(mines);
    })
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String{
    MS.with(|instance| instance.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x:usize,y:usize)
{
    MS.with(|instance| instance .borrow_mut().open((x,y)));
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x:usize,y:usize)
{
    MS.with(|instance| instance .borrow_mut().toggle_flag((x,y)));
}