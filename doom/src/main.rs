#[derive(Clone)]
struct Player {
    x: f32,
    y: f32,
    health: i32,
}

fn main() {
    let mut player = Player {
        x: 0.0,
        y: 0.0,
        health: 100,
    };

    print_player(&player);
    move_player(&mut player, 10.0, 5.0);
    let other_player = player;
    let cloned_player = other_player.clone();

    print_player(&cloned_player);
}

fn print_player(p: &Player) {
    println!("Player at ({}, {}) with {} health", p.x, p.y, p.health);
}

fn move_player(p: &mut Player, dx: f32, dy: f32) {
    p.x += dx;
    p.y += dy;
}
