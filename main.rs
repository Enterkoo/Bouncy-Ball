use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound};
use macroquad::prelude::*;

struct Player {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    health: f32,
}

struct Bullet {
    radius: f32,
    position: Vec2,
    velocity: Vec2,
}

impl Bullet {
    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, GREEN);
    }

    fn calculate_position(&mut self) {
        self.position += self.velocity * get_frame_time();
    }
}

impl Player {
    fn calculate_velocity(
        &mut self,
        bullets: &mut Vec<Bullet>,
        enemies: &mut Vec<Enemy>,
        shot: Sound,
    ) {
        let dt = get_frame_time();

        let gravity = 980.0;
        let bounce = 0.7;

        self.velocity.y += gravity * dt;

        if is_key_pressed(KeyCode::E) {
            for enemy in enemies.iter_mut() {
                let direction = (enemy.position - self.position).normalize_or_zero();

                if enemy.position.distance(self.position) <= 200.0 {
                    enemy.position += direction * 50.0;
                }
            }
        }

        if is_key_down(KeyCode::D) {
            self.velocity.x = 500.0;
        }

        if is_key_down(KeyCode::A) {
            self.velocity.x = -500.0;
        }

        if is_key_down(KeyCode::S) {
            self.velocity.y += 50.0;
        }

        if is_key_pressed(KeyCode::Space) {
            self.velocity.y = -1000.0;
        }

        if is_key_pressed(KeyCode::J) {
            let (mx, my) = mouse_position();
            let angle = (Vec2::new(mx, my) - self.position).normalize_or_zero();
            bullets.push(Bullet {
                radius: 10.0,
                position: Vec2::new(self.position.x, self.position.y),
                velocity: angle * 1000.0,
            });

            play_sound(
                &shot,
                PlaySoundParams {
                    looped: false,
                    volume: 0.8,
                },
            );
        }
        self.position += self.velocity * dt;

        if self.position.x - self.radius <= 0.0 {
            self.position.x = self.radius;
            self.velocity.x = self.velocity.x.abs() * bounce;
        }

        if self.position.x + self.radius >= 800.0 {
            self.position.x = 800.0 - self.radius;
            self.velocity.x = -self.velocity.x.abs() * bounce;
        }

        if self.position.y - self.radius <= 0.0 {
            self.position.y = self.radius;
            self.velocity.y = self.velocity.y.abs() * bounce;
        }

        if self.position.y + self.radius >= 590.0 {
            self.position.y = 590.0 - self.radius;
            self.velocity.y = -self.velocity.y.abs() * bounce;
        }
    }

    fn draw(&self, texture: &Texture2D) {
        draw_texture(
            texture,
            self.position.x - self.radius,
            self.position.y - self.radius,
            WHITE,
        );
        draw_text(
            self.health.to_string().as_str(),
            self.position.x - 25.0,
            self.position.y - 50.0,
            50.0,
            RED,
        );
    }
}

struct Enemy {
    position: Vec2,
    velocity: Vec2,
    speed: f32,
    health: f32,
}

impl Enemy {
    fn draw(&mut self, texture: &Texture2D) {
        draw_texture(
            texture,
            self.position.x - texture.width() / 2.0,
            self.position.y - texture.height() / 2.0,
            WHITE,
        );

        draw_text(
            self.health.to_string().as_str(),
            self.position.x - 25.0,
            self.position.y - 50.0,
            50.0,
            RED,
        );
    }

    fn calculate_position(&mut self, opposing_pos: Vec2) {
        let direction = (opposing_pos - self.position).normalize_or_zero();

        self.velocity = direction * self.speed;

        self.position += self.velocity * get_frame_time();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Ball Game".to_owned(),
        window_width: 800,
        window_height: 600,

        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let shot = load_sound("shoot.ogg").await.unwrap();
    let mut running = true;
    let mut level = 1;
    let enemy_texture = load_texture("enemy.png").await.unwrap();
    let player_texture = load_texture("ball.png").await.unwrap();
    let mut bullets: Vec<Bullet> = Vec::new();
    let background = load_texture("background.png").await.unwrap();
    let foreground = load_texture("clouds.png").await.unwrap();
    let mut enemies: Vec<Enemy> = Vec::new();

    let music = load_sound("music.ogg").await.unwrap();

    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.8,
        },
    );

    enemies.push(Enemy {
        position: Vec2::new(300.0, 300.0),
        velocity: Vec2::new(0.0, 0.0),
        speed: 100.0,
        health: 100.0,
    });

    let mut ball = Player {
        position: Vec2::new(300.0, 300.0),
        velocity: Vec2::new(0.0, 0.0),
        radius: 25.0,
        health: 100.0,
    };

    loop {
        enemies.retain(|enemy| enemy.health >= 0.0);

        clear_background(SKYBLUE);
        if running {
            draw_texture(&background, ball.position.x / 100.0, 0.0, WHITE);

            draw_texture(&foreground, ball.position.x / 10.0, 0.0, WHITE);
        }

        ball.calculate_velocity(&mut bullets, &mut enemies, shot.clone());
        ball.draw(&player_texture);

        if enemies.len() == 0 {
            level *= 2;
            for _ in 1..=level {
                let position = loop {
                    let random_position =
                        Vec2::new(rand::gen_range(50.0, 750.0), rand::gen_range(50.0, 550.0));

                    if random_position.distance(ball.position) >= 300.0 {
                        break random_position;
                    }
                };

                enemies.push(Enemy {
                    health: 100.0,
                    position,
                    velocity: Vec2::new(0.0, 0.0),
                    speed: rand::gen_range(75.0, 200.0),
                });
            }
        }

        for bullet in &mut bullets {
            if running {
                bullet.calculate_position();
                bullet.draw();
            }

            for enemy in &mut enemies {
                let distance_x = (bullet.position.x - enemy.position.x).abs();
                let distance_y = (bullet.position.y - enemy.position.y).abs();
                let is_in_range = distance_x <= 50.0 && distance_y <= 50.0;

                if is_in_range {
                    enemy.health -= 10.0;
                }

                let distance_x_ball = (ball.position.x - enemy.position.x).abs();
                let distance_y_ball = (ball.position.y - enemy.position.y).abs();
                let ball_in_range = distance_x_ball <= 100.0 && distance_y_ball <= 100.0;
                if ball_in_range {
                    ball.health -= 0.001;
                }
            }
        }

        for enemy in &mut enemies {
            if running {
                enemy.calculate_position(ball.position);
                enemy.draw(&enemy_texture);
            }
        }

        if ball.health <= 0.0 {
            running = false;
        }

        if !running {
            draw_text(
                "YOU DIED",
                screen_height() / 2.0,
                screen_width() / 2.0,
                50.0,
                BLACK,
            );
        }

        draw_text("J -> SHOOT", 0.0, 50.0, 40.0, YELLOW);
        draw_text("E -> SHOCKWAVE", 0.0, 75.0, 40.0, YELLOW);

        next_frame().await;
    }
}
