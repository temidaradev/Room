---
id: task-8
title: Learn pattern matching with enums
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn Rust's powerful pattern matching by creating game entity types. Enums and match are core Rust features.

## Acceptance Criteria

- [ ] Creates an enum for different entity types
- [ ] Uses match statements for pattern matching
- [ ] Understands exhaustive matching
- [ ] Can handle Option and Result with match

## Implementation Plan

## Background

Enums and pattern matching are among Rust's most powerful features. Unlike C enums (which are just numbers), Rust enums can carry data and represent complex state machines. Pattern matching forces you to handle all possible cases, preventing bugs.

### Key Concepts for This Task:
- Enums: Types that can be one of several variants
- Pattern matching: Destructuring data and handling all cases
- Exhaustive matching: Compiler ensures all cases are handled
- Data-carrying enums: Variants that contain associated data
- Guards and advanced patterns: Complex matching logic

### Pattern Matching Benefits:
- Compile-time exhaustiveness checking
- No null pointer exceptions
- Clear state machine modeling
- Elegant error handling
- Functional programming patterns

### Game Entity Types:
Different game objects need different data:
- Player: Position, health, weapons
- Enemy: Position, health, AI state
- Pickup: Position, item type
- Wall: Start/end points, texture
- Door: Position, open/closed state

### Example Code:
#[derive(Debug, Clone)]
enum GameEntity {
    Player {
        x: f32,
        y: f32,
        health: i32,
        weapon: WeaponType,
    },
    Enemy {
        x: f32,
        y: f32,
        health: i32,
        enemy_type: EnemyType,
        ai_state: AIState,
    },
    Pickup {
        x: f32,
        y: f32,
        item: ItemType,
    },
    Wall {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        texture: String,
    },
    Door {
        x: f32,
        y: f32,
        is_open: bool,
        open_timer: f32,
    },
}

#[derive(Debug, Clone)]
enum WeaponType {
    Pistol,
    Shotgun,
    Chaingun,
    RocketLauncher,
}

#[derive(Debug, Clone)]
enum EnemyType {
    Zombie,
    Demon,
    Cacodemon,
}

#[derive(Debug, Clone)]
enum AIState {
    Idle,
    Chasing { target_x: f32, target_y: f32 },
    Attacking { cooldown: f32 },
    Fleeing { direction: f32 },
}

#[derive(Debug, Clone)]
enum ItemType {
    HealthPotion,
    Ammo { weapon: WeaponType, count: i32 },
    KeyCard { color: String },
}

impl GameEntity {
    fn position(&self) -> (f32, f32) {
        match self {
            GameEntity::Player { x, y, .. } => (*x, *y),
            GameEntity::Enemy { x, y, .. } => (*x, *y),
            GameEntity::Pickup { x, y, .. } => (*x, *y),
            GameEntity::Wall { x1, y1, .. } => (*x1, *y1),  // Start point
            GameEntity::Door { x, y, .. } => (*x, *y),
        }
    }
    
    fn is_alive(&self) -> bool {
        match self {
            GameEntity::Player { health, .. } | GameEntity::Enemy { health, .. } => *health > 0,
            _ => false,  // Non-living entities
        }
    }
    
    fn take_damage(&mut self, damage: i32) {
        match self {
            GameEntity::Player { health, .. } | GameEntity::Enemy { health, .. } => {
                *health = (*health - damage).max(0);
            }
            _ => {} // Other entities can't take damage
        }
    }
}

fn update_entity(entity: &mut GameEntity, dt: f32) {
    match entity {
        GameEntity::Player { .. } => {
            // Handle player input
        }
        GameEntity::Enemy { ai_state, x, y, .. } => {
            match ai_state {
                AIState::Idle => {
                    // Look for player
                }
                AIState::Chasing { target_x, target_y } => {
                    // Move towards target
                    let dx = target_x - *x;
                    let dy = target_y - *y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    if distance > 0.1 {
                        *x += dx / distance * 50.0 * dt;
                        *y += dy / distance * 50.0 * dt;
                    }
                }
                AIState::Attacking { cooldown } => {
                    *cooldown -= dt;
                    if *cooldown <= 0.0 {
                        // Attack and reset cooldown
                        *cooldown = 1.0;
                    }
                }
                AIState::Fleeing { direction } => {
                    *x += direction.cos() * 100.0 * dt;
                    *y += direction.sin() * 100.0 * dt;
                }
            }
        }
        GameEntity::Door { is_open, open_timer, .. } => {
            if *is_open {
                *open_timer -= dt;
                if *open_timer <= 0.0 {
                    *is_open = false;
                }
            }
        }
        _ => {} // Other entities don't need updates
    }
}

fn handle_collision(entity1: &mut GameEntity, entity2: &mut GameEntity) {
    match (entity1, entity2) {
        (GameEntity::Player { .. }, GameEntity::Pickup { item, .. }) => {
            match item {
                ItemType::HealthPotion => {
                    // Heal player
                    println\!("Player picked up health potion");
                }
                ItemType::Ammo { weapon, count } => {
                    println\!("Player picked up {} ammo for {:?}", count, weapon);
                }
                ItemType::KeyCard { color } => {
                    println\!("Player picked up {} key card", color);
                }
            }
        }
        (GameEntity::Player { health, .. }, GameEntity::Enemy { .. }) => {
            // Player hit by enemy
            if let GameEntity::Player { health, .. } = entity1 {
                *health -= 10;
            }
        }
        _ => {} // Other collision types
    }
}

## Step-by-Step Implementation:

1. Create basic enums for entity types (start simple)
2. Add data to enum variants (position, health, etc.)
3. Implement pattern matching with match statements
4. Practice exhaustive matching (handle all cases)
5. Add nested enums (AI states, item types)
6. Implement methods that use pattern matching
7. Handle Option<T> and Result<T, E> with match
8. Use if let for single-case matching
9. Explore guards and complex patterns
10. Create a small entity system using enums
