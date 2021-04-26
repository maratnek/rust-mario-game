use sfml::{
    graphics::{
        CircleShape, Color, ConvexShape, Font, IntRect, RectangleShape, RenderTarget, RenderWindow,
        Sprite, Text, Texture, Transformable,
    },
    window::{Event, Key, Style},
};

use std::time::{Duration, Instant};

#[derive(Debug)]
struct Mario {
    position: f32,
}

impl Mario {
    fn back(&mut self) {
        println!("back");
        self.position -= 1.0;
    }
    fn forward(&mut self) {
        println!("forward");
        self.position += 1.0;
    }
}

trait Drawn {
    fn draw(&mut self) {
        println!("Draw for drawn");
    }
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

// fn change_txture(state: &mut State) {
//     match state {
//         Go1 => {
//             println!("go first");
//             *state = State::Go2;
//         }
//         Go2 => {
//             println!("go second");
//             *state = State::Go1;
//         }
//     }
// }

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
    let mut mario = Mario { position: 0.0 };
    let mut mario_sprite = Sprite::new();
    mario_sprite.set_texture(&mario_go_right_tx1, true);
    // mario_sprite.set_texture(&mario_texture, true);
    mario_sprite.set_scale((2.5, 2.5));

    let mut position = 0.0;
    let mut go_state = State::Go1;
    mario_sprite.set_position((position, 0.0));

    // Create a new texture. (Hey Frank!)
    let frank = Texture::from_file("resources/frank.jpeg").unwrap();

    // Create a font.
    let font = Font::from_file("resources/sansation.ttf").unwrap();

    // rectang.set_position(position: P)
    // Create a circle with the Texture.
    let mut circle = CircleShape::with_texture(&frank);
    circle.set_radius(70.0);
    circle.set_position((100.0, 100.0));

    // Create a Sprite.
    let mut sprite = Sprite::new();
    // Have it use the same texture as the circle.
    sprite.set_texture(&frank, true);
    sprite.set_position((400.0, 300.0));
    sprite.set_scale((0.5, 0.5));

    // Create a ConvexShape using the same texture.
    let mut convex_shape = ConvexShape::with_texture(6, &frank);
    convex_shape.set_point(0, (400., 100.));
    convex_shape.set_point(1, (500., 70.));
    convex_shape.set_point(2, (450., 100.));
    convex_shape.set_point(3, (580., 150.));
    convex_shape.set_point(4, (420., 230.));
    convex_shape.set_point(5, (420., 120.));

    // Create an initialized text using the font.
    let title = Text::new("Borrowed resources example!", &font, 50);

    // Create a second text using the same font.
    // This time, we create and initialize it separately.
    let mut second_text = Text::default();
    second_text.set_string("This text shares the same font with the title!");
    second_text.set_font(&font);
    second_text.set_fill_color(Color::GREEN);
    second_text.set_position((10.0, 350.0));
    second_text.set_character_size(20);

    // Create a third text using the same font.
    let mut third_text = Text::new("This one too!", &font, 20);
    third_text.set_position((300.0, 100.0));
    third_text.set_fill_color(Color::RED);

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
                    position += 5.0;
                    mario_sprite.set_position((position, mario_sprite.position().y));
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
                    position -= 5.0;
                    mario_sprite.set_position((position, mario_sprite.position().y));
                }
                Event::KeyPressed {
                    code: Key::Space, ..
                } => {
                    println!("Click to Space`ship` key");
                    // here need to add acceleration
                    if mario.position == flaw_position {
                        velocity = -100.0;    
                    }
                }
                _ => {}
            }
        }


        if mario.position >= flaw_position {
            mario.position = flaw_position;
            // velocity = 0.0;
            // println!("Velocity 0.0");
            if velocity < 0.0 {
                start = Instant::now();
                duration = start.elapsed().as_millis();
                // let t_sec = duration as f32 / 1000.0;
                let t_sec = 1.0 / 1000.0;
                let path = t_sec * ( velocity + 9.81 * t_sec );
                mario.position += path;
                println!(
                    "Duration time {:?} velocity {} path {}",
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
                mario.position += path;
                println!(
                    "Duration time {:?} velocity {} path {}",
                    duration, velocity, path
                );
                update(&mut mario_sprite, &mut mario);
            }
        }

        window.draw(&circle);
        window.clear(Color::BLACK);
        window.draw(&mario_sprite);
        // window.draw(&sprite);
        // window.draw(&convex_shape);
        // window.draw(&title);
        // window.draw(&second_text);
        // window.draw(&third_text);
        window.display();
    }
}

fn update(sprite: &mut Sprite, mario: &mut Mario) {
    // println!("Update {:?} {:?}!", sprite, mario);
    // mario.position += 0.1;
    sprite.set_position((sprite.position().x, mario.position));
}
