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
    /// Generates new Player instance with given x and y coordinates
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    /// Renders Player as '@' at left of screen
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    /// Iterates velocity, recalculates y-value accordingly\
    /// Iterates world-space x-value
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            // 2.0 is terminal velocity for the player
            self.velocity += 0.2; // 0.2 is gravitational acceleration
        }
        self.y += self.velocity as i32; // Increment height by velocity
        self.x += 1; // Increment world-space position in 'x' direction
        if self.y < 0 {
            // Checks for collision with top of screen
            self.y = 0;
        }
    }

    /// Sets velocity to -2.0
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

/// Game state struct equipped with `tick` method for updating state
struct State {
    player: Player,     // Player
    frame_time: f32,    // Time elapsed since last frame render
    mode: GameMode,     // GameMode,
    score: i32,         // Score
    obstacle: Obstacle, // Current obstacle
}

impl State {
    /// Constructs new State
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    /// Executes game behavior for `Playing` game mode
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // Clears bg with background color NAVY

        self.frame_time += ctx.frame_time_ms; // Add time elapsed since last call to `tick`
        if self.frame_time > FRAME_DURATION {
            // If FRAME_DURATION has been reached, reset frame_time and run physics
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // If player presses SPACE, flap!
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        // Renders player on screen
        self.player.render(ctx);

        // Renders textual info
        ctx.print(0, 0, "Press SPACE to flap!");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        // Renders obstacle on screen
        self.obstacle.render(ctx, self.player.x);

        // If player has passed obstacle, generate a new one and increment score
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH,
                self.score
            );
        }

        // End game if player touches ground
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
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
    /// Runs next frame based on GameMode
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

struct Obstacle {
    x: i32,     // Position in world-space
    gap_y: i32, // Center point of gap
    size: i32,  // Size of gap
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Renders top part of obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // Renders bottom part of obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
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
