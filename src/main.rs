use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 20.0;
const ENEMY_RADIUS: f32 = 15.0;
const LASER_WIDTH: f32 = 3.0;
const LASER_LENGTH: f32 = 15.0;
const PLAYER_SPEED: f32 = 200.0;
const ENEMY_SPEED: f32 = 50.0;
const LASER_SPEED: f32 = 400.0;

#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    health: i32,
}

#[derive(Clone)]
struct Enemy {
    x: f32,
    y: f32,
    health: i32,
}

#[derive(Clone)]
struct Laser {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    lasers: Vec<Laser>,
    score: i32,
    enemy_spawn_timer: f32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        Game {
            player: Player {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
                health: 100,
            },
            enemies: Vec::new(),
            lasers: Vec::new(),
            score: 0,
            enemy_spawn_timer: 0.0,
            game_over: false,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        self.update_player(dt);
        self.update_lasers(dt);
        self.update_enemies(dt);
        self.spawn_enemies(dt);
        self.check_collisions();
        
        if self.player.health <= 0 {
            self.game_over = true;
        }
    }

    fn update_player(&mut self, dt: f32) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dy -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dy += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1.0;
        }

        // Normalize diagonal movement
        if dx != 0.0 && dy != 0.0 {
            dx *= 0.707;
            dy *= 0.707;
        }

        self.player.x += dx * PLAYER_SPEED * dt;
        self.player.y += dy * PLAYER_SPEED * dt;

        // Keep player on screen
        self.player.x = self.player.x.clamp(PLAYER_SIZE / 2.0, screen_width() - PLAYER_SIZE / 2.0);
        self.player.y = self.player.y.clamp(PLAYER_SIZE / 2.0, screen_height() - PLAYER_SIZE / 2.0);

        // Shoot laser towards mouse
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let angle = (mouse_y - self.player.y).atan2(mouse_x - self.player.x);
            
            self.lasers.push(Laser {
                x: self.player.x,
                y: self.player.y,
                dx: angle.cos(),
                dy: angle.sin(),
            });
        }
    }

    fn update_lasers(&mut self, dt: f32) {
        for laser in &mut self.lasers {
            laser.x += laser.dx * LASER_SPEED * dt;
            laser.y += laser.dy * LASER_SPEED * dt;
        }

        // Remove lasers that are off screen
        self.lasers.retain(|laser| {
            laser.x > -50.0 && laser.x < screen_width() + 50.0 &&
            laser.y > -50.0 && laser.y < screen_height() + 50.0
        });
    }

    fn update_enemies(&mut self, dt: f32) {
        for enemy in &mut self.enemies {
            let dx = self.player.x - enemy.x;
            let dy = self.player.y - enemy.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance > 0.0 {
                enemy.x += (dx / distance) * ENEMY_SPEED * dt;
                enemy.y += (dy / distance) * ENEMY_SPEED * dt;
            }
        }
    }

    fn spawn_enemies(&mut self, dt: f32) {
        self.enemy_spawn_timer += dt;
        
        if self.enemy_spawn_timer > 2.0 {
            self.enemy_spawn_timer = 0.0;
            
            // Spawn enemy at random edge of screen
            let side = rand::gen_range(0, 4);
            let (x, y) = match side {
                0 => (rand::gen_range(0.0, screen_width()), -ENEMY_RADIUS), // Top
                1 => (screen_width() + ENEMY_RADIUS, rand::gen_range(0.0, screen_height())), // Right
                2 => (rand::gen_range(0.0, screen_width()), screen_height() + ENEMY_RADIUS), // Bottom
                _ => (-ENEMY_RADIUS, rand::gen_range(0.0, screen_height())), // Left
            };
            
            self.enemies.push(Enemy {
                x,
                y,
                health: 1,
            });
        }
    }

    fn check_collisions(&mut self) {
        // Laser-enemy collisions
        let mut enemies_to_remove = Vec::new();
        let mut lasers_to_remove = Vec::new();

        for (laser_idx, laser) in self.lasers.iter().enumerate() {
            for (enemy_idx, enemy) in self.enemies.iter().enumerate() {
                let dx = laser.x - enemy.x;
                let dy = laser.y - enemy.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance < ENEMY_RADIUS + LASER_WIDTH {
                    enemies_to_remove.push(enemy_idx);
                    lasers_to_remove.push(laser_idx);
                    self.score += 10;
                }
            }
        }

        // Remove collided entities (in reverse order to maintain indices)
        enemies_to_remove.sort_by(|a, b| b.cmp(a));
        lasers_to_remove.sort_by(|a, b| b.cmp(a));
        
        for &idx in &enemies_to_remove {
            if idx < self.enemies.len() {
                self.enemies.remove(idx);
            }
        }
        
        for &idx in &lasers_to_remove {
            if idx < self.lasers.len() {
                self.lasers.remove(idx);
            }
        }

        // Player-enemy collisions
        for enemy in &self.enemies {
            let dx = self.player.x - enemy.x;
            let dy = self.player.y - enemy.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < PLAYER_SIZE / 2.0 + ENEMY_RADIUS {
                self.player.health -= 1;
            }
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        if self.game_over {
            let text = format!("Game Over! Final Score: {}", self.score);
            let text_size = measure_text(&text, None, 40, 1.0);
            draw_text(
                &text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0,
                40.0,
                RED,
            );
            
            let restart_text = "Press R to restart";
            let restart_size = measure_text(restart_text, None, 20, 1.0);
            draw_text(
                restart_text,
                screen_width() / 2.0 - restart_size.width / 2.0,
                screen_height() / 2.0 + 60.0,
                20.0,
                WHITE,
            );
            return;
        }

        // Draw player (block)
        draw_rectangle(
            self.player.x - PLAYER_SIZE / 2.0,
            self.player.y - PLAYER_SIZE / 2.0,
            PLAYER_SIZE,
            PLAYER_SIZE,
            BLUE,
        );

        // Draw enemies (circles)
        for enemy in &self.enemies {
            draw_circle(enemy.x, enemy.y, ENEMY_RADIUS, RED);
        }

        // Draw lasers
        for laser in &self.lasers {
            let end_x = laser.x + laser.dx * LASER_LENGTH;
            let end_y = laser.y + laser.dy * LASER_LENGTH;
            draw_line(laser.x, laser.y, end_x, end_y, LASER_WIDTH, GREEN);
        }

        // Draw UI
        draw_text(&format!("Health: {}", self.player.health), 10.0, 30.0, 20.0, WHITE);
        draw_text(&format!("Score: {}", self.score), 10.0, 55.0, 20.0, WHITE);
        draw_text("Use WASD/Arrow keys to move", 10.0, screen_height() - 40.0, 16.0, GRAY);
        draw_text("Click to shoot laser at mouse cursor", 10.0, screen_height() - 20.0, 16.0, GRAY);
    }
}

#[macroquad::main("Survival Game")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        if game.game_over && is_key_pressed(KeyCode::R) {
            game = Game::new();
        }

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
