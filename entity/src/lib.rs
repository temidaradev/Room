use bevy_ecs::prelude::*;
use std::time::Duration;

pub struct Entity;

// Components
#[derive(Component, Debug, Clone)]
pub enum EntityType {
    Monster {
        health: i32,
        monster_type: MonsterType,
    },
    Item {
        item_type: ItemType,
        respawn_time: Option<Duration>,
    },
    Projectile {
        damage: i32,
        velocity: (f64, f64),
    },
    Decoration,
}

#[derive(Component, Debug, Clone)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub angle: f64,
}

#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub radius: f64,
    pub height: f64,
}

#[derive(Component, Debug, Clone)]
pub struct Sprite {
    pub name: String,
}

#[derive(Component)]
pub struct Active(bool);

#[derive(Debug, Clone)]
pub enum MonsterType {
    Imp,
    Demon,
    Cacodemon,
    BaronOfHell,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Health,
    Armor,
    Weapon(WeaponType),
    Ammo(AmmoType),
    Key(KeyType),
}

// Systems
pub fn update_monsters(
    mut monsters: Query<(&mut Transform, &EntityType), With<Active>>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = if let Ok(transform) = player.get_single() {
        transform
    } else {
        return;
    };

    for (mut transform, entity_type) in monsters.iter_mut() {
        if let EntityType::Monster { .. } = entity_type {
            let dx = player_transform.x - transform.x;
            let dy = player_transform.y - transform.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 50.0 {
                let move_speed = 50.0;
                let dt = time.delta_seconds_f64();

                transform.x += (dx / distance) * move_speed * dt;
                transform.y += (dy / distance) * move_speed * dt;
                transform.angle = dy.atan2(dx);
            }
        }
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &EntityType), With<Active>>,
    time: Res<Time>,
) {
    for (entity, mut transform, entity_type) in projectiles.iter_mut() {
        if let EntityType::Projectile { velocity, .. } = entity_type {
            transform.x += velocity.0 * time.delta_seconds_f64();
            transform.y += velocity.1 * time.delta_seconds_f64();

            // TODO: Add collision detection
            // if check_projectile_collision(...) {
            //     commands.entity(entity).despawn();
            // }
        }
    }
}

// Spawn helper functions
pub fn spawn_entity(
    commands: &mut Commands,
    x: f64,
    y: f64,
    entity_type: EntityType,
    sprite_name: String,
) -> Entity {
    commands
        .spawn((
            entity_type,
            Transform {
                x,
                y,
                z: 0.0,
                angle: 0.0,
            },
            Collider {
                radius: 20.0,
                height: 56.0,
            },
            Sprite { name: sprite_name },
            Active(true),
        ))
        .id()
}

// Plugin to organize the systems
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_monsters, update_projectiles));
    }
}
