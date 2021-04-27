use sfml::{
    graphics::{
        CircleShape, Color, ConvexShape, Font, IntRect, RectangleShape, RenderTarget, RenderWindow,
        Sprite, Text, Texture, Transformable,
    },
    window::{Event, Key, Style},
    system::Vector2f,
};

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
struct Mario {
    position: Position,
}

impl Mario {
    const speed: f32 = 20.0;

    fn get_position(&self) -> Vector2f {
        Vector2f::new(self.position.x, self.position.y)
    }
    fn back(&mut self) {
        println!("back");
        self.position.x -= 1.0;
    }
    fn forward(&mut self) {
        println!("forward");
        self.position.x += 1.0;
    }
}

trait Drawn {
    fn draw(&mut self) {
        println!("Draw for drawn");
    }
}

#[derive(Debug)]
enum FlyState {
    Fly,
    Ground,
}

#[derive(Debug)]
enum State {
    Go1,
    Go2,
}

impl State {
    fn change(&mut self) {
        match *self {
            State::Go1 => {
                *self = State::Go2;
            }
            State::Go2 => {
                *self = State::Go1;
            }
        }
    }
}

fn main() {
    let mut window = RenderWindow::new((1200, 800), "Mario", Style::DEFAULT, &Default::default());
    window.set_vertical_sync_enabled(true);

    // work with textures
    // let mario_texture = Texture::from_file("resources-mario/Player.png").unwrap();
    let rect1 = IntRect::new(30 * 8, 0, 20, 30);
    let mario_go_right_tx1 =
        Texture::from_file_with_rect("resources-mario/Player.png", &rect1).unwrap();
    let rect2 = IntRect::new(30 * 9, 0, 20, 30);
    let mario_go_right_tx2 =
        Texture::from_file_with_rect("resources-mario/Player.png", &rect2).unwrap();
    // mario sprite
    let mut mario = Mario { position: Position {x:0.0, y:0.0} };
    let mut mario_sprite = Sprite::new();
    mario_sprite.set_texture(&mario_go_right_tx1, true);
    // mario_sprite.set_texture(&mario_texture, true);
    mario_sprite.set_scale((2.5, 2.5));

    let mut position = 0.0;
    let mut go_state = State::Go1;
    mario_sprite.set_position(mario.get_position());

    // time
    let mut start = Instant::now();
    let mut duration : u128= 0;
    // velocity
    let mut velocity = 0.0;
    // ground
    let flaw_position = 700.0;
    // acceleration
    let mut acceleration = 0.0;
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::KeyPressed { code: Key::Up, .. } => println!("Click to up key"),
                Event::KeyPressed {
                    code: Key::Down, ..
                } => println!("Click to down key"),
                Event::KeyPressed {
                    code: Key::Right, ..
                } => {
                    println!("Click to right key");
                    // position += 5.0;
                    mario.position.x += Mario::speed;
                    mario_sprite.set_position((mario.position.x, mario_sprite.position().y));
                    go_state.change();
                    println!("Go state {:?}", go_state);
                    match go_state {
                        State::Go1 => {
                            mario_sprite.set_texture(&mario_go_right_tx2, true);
                        }
                        State::Go2 => {
                            mario_sprite.set_texture(&mario_go_right_tx1, true);
                        }
                    }
                }
                Event::KeyPressed {
                    code: Key::Left, ..
                } => {
                    println!("Click to left key");
                    // position -= 5.0;
                    mario.position.x -= Mario::speed;
                    mario_sprite.set_position((mario.position.x, mario_sprite.position().y));
                }
                Event::KeyPressed {
                    code: Key::Space, ..
                } => {
                    println!("Click to Space`ship` key");
                    // here need to add acceleration
                    if mario.position.y == flaw_position {
                        velocity = -100.0;    
                    }
                }
                _ => {}
            }
        }


        if mario.position.y >= flaw_position {
            mario.position.y = flaw_position;
            if velocity < 0.0 {
                start = Instant::now();
                duration = start.elapsed().as_millis();
                // let t_sec = duration as f32 / 1000.0;
                let t_sec = 1.0 / 1000.0;
                let path = t_sec * ( velocity + 9.81 * t_sec );
                mario.position.y += path;
                println!(
                    "Duration time y >= flaw {:?} velocity {} path {}",
                    duration, velocity, path
                );
            }
            update(&mut mario_sprite, &mut mario);
        } else {
            let elapsed_time = start.elapsed().as_millis();
            if elapsed_time > duration {
                duration = elapsed_time;
                let t_sec = duration as f32 / 1000.0;
                velocity = velocity + 9.81 * t_sec;
                let path = velocity * t_sec;
                mario.position.y += path;
                println!(
                    "Duration time {:?} velocity {} path {} mario pos {:?}",
                    duration, velocity, path, mario
                );
                update(&mut mario_sprite, &mut mario);
            }
        }

        window.clear(Color::BLACK);
        window.draw(&mario_sprite);
        window.display();
    }
}

fn update(sprite: &mut Sprite, mario: &mut Mario) {
    // println!("Update {:?} {:?}!", sprite, mario);
    // sprite.set_position((sprite.position().x, mario.position.y));
    sprite.set_position((mario.position.x, mario.position.y));
}
