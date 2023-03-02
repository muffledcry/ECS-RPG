mod components;
mod entities;
mod game;
mod utils;



fn main() {
    let mut game = game::Game::new();
    game.init();
    for entity in game.entity_manager.entities.iter() {
        println!("{:?}", entity.get_id());
    }
}



