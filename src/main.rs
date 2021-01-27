use enemy::enemys;
use sfml::{
    graphics::{
        RectangleShape, CircleShape, Color, ConvexShape, Font, RenderTarget, RenderWindow, Sprite, Text, Texture,
        IntRect,
        Transformable,
    },
    window::{Event, Key, Style},
};

mod enemy;

fn main() {
    let mut window = RenderWindow::new(
        (1200, 800),
        "Mario",
        Style::DEFAULT,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    enemy::enemys::start_enemy();

    // work with textures
    let mario_texture = Texture::from_file("resources-mario/Player.png").unwrap();
    let rect = IntRect::new(20 * 6, 0, 20, 30);
    let mario_rect_txture = Texture::from_file_with_rect("resources-mario/Player.png", &rect).unwrap();
    // mario sprite
    let mut mario_sprite = Sprite::new();
    mario_sprite.set_texture(&mario_rect_txture, true);
    // mario_sprite.set_texture(&mario_texture, true);
    mario_sprite.set_position((100.0, 0.0));
    mario_sprite.set_scale((2.5, 2.5));

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

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::KeyPressed {
                    code: Key::Up, ..
                } => {println!("Click to up key")}
                Event::KeyPressed {
                    code: Key::Down, ..
                } => {println!("Click to down key")}
                Event::KeyPressed {
                    code: Key::Right, ..
                } => {println!("Click to right key")}
                Event::KeyPressed {
                    code: Key::Left, ..
                } => {println!("Click to left key")}
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&mario_sprite);
        // window.draw(&circle);
        // window.draw(&sprite);
        // window.draw(&convex_shape);
        // window.draw(&title);
        // window.draw(&second_text);
        // window.draw(&third_text);
        window.display();
    }
}