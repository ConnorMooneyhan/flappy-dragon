use bracket_lib::prelude::*;

/// State machine providing the following three game modes:
/// * Menu
/// * Playing
/// * End
enum GameMode {
    Menu,
    Playing,
    End
}

/// Game state struct equipped with `tick` method for updating state
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {
    // Configures display window
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    // Starts game loop; returns BError
    main_loop(context, State::new())
}
