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
    /// Constructs new State
    fn new() -> Self {
        Self {
            mode: GameMode::Menu
        }
    }

    /// Executes game behavior for `Playing` game mode
    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub later
        self.mode = GameMode::End;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
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
