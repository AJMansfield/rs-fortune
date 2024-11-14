use state::Board;

mod state;
mod text;

fn main() {
    println!("Hello, world!");

    let mut b = Board::default();
    b.apply_forced();
}
