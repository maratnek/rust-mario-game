#[derive(Debug)]
struct Enemy {
    name: String,
    life: bool,
}

trait Draw {
    fn draw(&self); 
}

impl Enemy {
    fn new(name: String, life: bool) -> Self {
        Self {
            name,
            life,
        }
    }

}

impl Draw for Enemy {
    fn draw(&self) {
        println!("Draw enemy {:?}", self);
    }
}

trait Walk {
    fn walk();
}

trait Fly {
    fn fly();
}


pub fn start_enemy() {
    println!("Start my enemy");
}

#[test]
fn enemy() {
    println!("Enemy test");
    let enemy = Enemy::new("tortilus".to_string(), true);
    assert_eq!("tortilus", enemy.name);
}

#[test]
fn draw_enemy() {
    let enemy = Enemy::new("tortilus".to_string(), true);
    enemy.draw();
}