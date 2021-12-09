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

/// Player struct
struct Player {
    x: i32, // World-space position
    y: i32, // Screen-space position
    velocity: f32, // Vertical velocity
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }
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

    /// Restarts game
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
    
    /// Displays main menu and responds to input
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => ()
            }
        }
    }

    /// Displays game over menu and responds to input
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead.");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => ()
            }
        }
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
