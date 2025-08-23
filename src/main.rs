use macroquad::prelude::*; //using prelude module in macroquad
use macroquad::prelude::KeyCode::*; //For giving access to keys like Up, Down, Left, Right
use macroquad::rand::gen_range; //For food placement in the board

const GRID_W: i32 = 20; //Width of the grid
const GRID_H: i32 = 20; //Height of the grid
const CELL: f32 = 24.0; //Size of each grid cell
const STEP_TIME: f32 = 0.24; //Time interva b/w the snake movements in sec

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up    => (0, -1),
            Dir::Down  => (0,  1),
            Dir::Left  => (-1, 0),
            Dir::Right => (1,  0),
        }
    }
}

fn is_opposite(a: Dir, b: Dir) -> bool {
    matches!(
        (a, b),
        (Dir::Up, Dir::Down) | (Dir::Down, Dir::Up) |
        (Dir::Left, Dir::Right) | (Dir::Right, Dir::Left)
    )
}

struct Game {
    snake: Vec<(i32, i32)>, 
    dir: Dir,               // direction currently moving this step
    next_dir: Dir,          // desired direction (latest user input)
    timer: f32,
    food: (i32, i32),
    score: u32,
    alive: bool,
}

impl Game {
    fn new() -> Self {
        let mut g = Self {
            snake: vec![(GRID_W / 2, GRID_H / 2)],
            dir: Dir::Right,
            next_dir: Dir::Right,
            timer: 0.0,
            food: (0, 0),
            score: 0,
            alive: true,
        };
        g.spawn_food();
        g
    }

    fn spawn_food(&mut self) {
        loop {
            let x = gen_range(0, GRID_W);
            let y = gen_range(0, GRID_H);
            if !self.snake.contains(&(x, y)) {
                self.food = (x, y);
                break;
            }
        }
    }

    // Always take the MOST RECENT key the user pressed; if none, optionally steer by held keys.
    fn handle_input(&mut self) {
        // 1) Latest key pressed wins
        if let Some(k) = get_last_key_pressed() {
            if let Some(d) = key_to_dir(k) {
                self.next_dir = d;
            }
        } else {
            // 2) If no new press this frame, optionally steer by held key
            if is_key_down(Up)    { self.next_dir = Dir::Up; }
            if is_key_down(Down)  { self.next_dir = Dir::Down; }
            if is_key_down(Left)  { self.next_dir = Dir::Left; }
            if is_key_down(Right) { self.next_dir = Dir::Right; }
        }
    }

    fn step(&mut self) {
        if !self.alive { return; }
        if !is_opposite(self.next_dir, self.dir) {
            self.dir = self.next_dir;
        }

        let (dx, dy) = self.dir.delta();
        let (hx, hy) = self.snake[0];
        let nx = hx + dx;
        let ny = hy + dy;

        if nx < 0 || nx >= GRID_W || ny < 0 || ny >= GRID_H {
            self.alive = false;
            return;
        }

        let new_head = (nx, ny);

        if self.snake.contains(&new_head) {
            self.alive = false;
            return;
        }

        if new_head == self.food {
            self.snake.insert(0, new_head); 
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.insert(0, new_head);
            self.snake.pop();
        }
    }
}

fn key_to_dir(k: KeyCode) -> Option<Dir> {
    match k {
        Up    => Some(Dir::Up),
        Down  => Some(Dir::Down),
        Left  => Some(Dir::Left),
        Right => Some(Dir::Right),
        _ => None,
    }
}

fn draw_cell((x, y): (i32, i32), color: Color) {
    let px = x as f32 * CELL;
    let py = y as f32 * CELL;
    draw_rectangle(px + 1.0, py + 1.0, CELL - 2.0, CELL - 2.0, color);
}

#[macroquad::main("Snake")]
async fn main() {
    request_new_screen_size(GRID_W as f32 * CELL, GRID_H as f32 * CELL);

    let mut game = Game::new();

    loop {
    
        game.handle_input();

        // UPDATE
        game.timer += get_frame_time();
        while game.timer >= STEP_TIME {
            game.timer -= STEP_TIME;
            game.step();
        }

        // DRAW
        clear_background(BLACK);
        draw_rectangle(
            0.0, 0.0,
            GRID_W as f32 * CELL, GRID_H as f32 * CELL,
            Color::from_rgba(25, 25, 25, 255),
        );

        draw_cell(game.food, RED);

        for (i, seg) in game.snake.iter().enumerate() {
            let color = if i == 0 { GREEN } else { Color::from_rgba(0, 180, 0, 255) };
            draw_cell(*seg, color);
        }

        draw_text(&format!("Score: {}", game.score), 6.0, 18.0, 22.0, WHITE);

        if !game.alive {
            draw_text("Game Over - Press R", 6.0, 42.0, 24.0, YELLOW);
            if is_key_pressed(KeyCode::R) {
                game = Game::new();
            }
        }

        next_frame().await;
    }
}
