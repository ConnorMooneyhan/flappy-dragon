use bracket_lib::prelude::*;

/// State machine providing the following three game modes:
/// * Menu
/// * Playing
/// * End
enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

/// Player struct
struct Player {
    x: i32,        // World-space position
    y: i32,        // Screen-space position
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
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            // 2.0 is terminal velocity for the player
            self.velocity += 0.2; // 0.2 is gravitational acceleration
        }
        self.y += self.velocity as i32; // Increment height by velocity
        self.x += 1; // Increment world-space position in 'x' direction
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

/// Game state struct equipped with `tick` method for updating state
struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    /// Constructs new State
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    /// Executes game behavior for `Playing` game mode
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // Clears bg with background color NAVY

        self.frame_time += ctx.frame_time_ms; // Add time elapsed since last call to `tick`
        if self.frame_time > FRAME_DURATION { // If FRAME_DURATION has been reached, reset frame_time and run physics
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // If player presses SPACE, flap!
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        // Render player on screen
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap!");

        // End game if player touches ground
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    /// Restarts game
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
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
                _ => (),
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
                _ => (),
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
