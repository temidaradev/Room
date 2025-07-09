---
id: task-9
title: Create a simple 2D coordinate system
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn about implementing traits and basic math operations by creating a 2D coordinate system for the game world.

## Acceptance Criteria

- [ ] Creates a Point2D struct with x y fields
- [ ] Implements basic math operations (add subtract)
- [ ] Understands operator overloading in Rust
- [ ] Uses the struct in game logic

## Implementation Plan

## Background

Operator overloading and traits are Rust's way of adding behavior to types. The trait system is similar to interfaces in other languages but more powerful. Understanding traits is crucial for writing idiomatic Rust code.

### Key Concepts for This Task:
- Traits: Shared behavior across different types
- Operator overloading: Implementing + - * / for custom types
- Derive macros: Automatically implementing common traits
- Generic programming: Writing code that works with any type
- Method chaining: Fluent interfaces for better ergonomics

### Common Traits to Implement:
- Display: For custom string formatting
- Debug: For debugging output
- Clone: For creating copies
- PartialEq: For equality comparison
- Add, Sub, Mul: For arithmetic operations

### 2D Math Fundamentals:
- Vector addition: (x1, y1) + (x2, y2) = (x1+x2, y1+y2)
- Vector subtraction: (x1, y1) - (x2, y2) = (x1-x2, y1-y2)
- Scalar multiplication: k * (x, y) = (k*x, k*y)
- Dot product: (x1, y1) · (x2, y2) = x1*x2 + y1*y2
- Distance: sqrt((x2-x1)² + (y2-y1)²)

### Example Code:
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }
    
    fn origin() -> Self {
        Point2D::new(0.0, 0.0)
    }
    
    fn distance_to(&self, other: &Point2D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    fn dot_product(&self, other: &Point2D) -> f32 {
        self.x * other.x + self.y * other.y
    }
    
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    fn normalize(&self) -> Point2D {
        let mag = self.magnitude();
        if mag > 0.0 {
            Point2D::new(self.x / mag, self.y / mag)
        } else {
            *self
        }
    }
    
    fn rotate(&self, angle: f32) -> Point2D {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Point2D::new(
            self.x * cos_a - self.y * sin_a,
            self.x * sin_a + self.y * cos_a,
        )
    }
}

// Implement Display trait for nice output
impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write\!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// Implement Add trait for + operator
impl Add for Point2D {
    type Output = Point2D;
    
    fn add(self, other: Point2D) -> Point2D {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

// Implement Sub trait for - operator
impl Sub for Point2D {
    type Output = Point2D;
    
    fn sub(self, other: Point2D) -> Point2D {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}

// Implement Mul trait for scalar multiplication
impl Mul<f32> for Point2D {
    type Output = Point2D;
    
    fn mul(self, scalar: f32) -> Point2D {
        Point2D::new(self.x * scalar, self.y * scalar)
    }
}

// Allow f32 * Point2D (commutative)
impl Mul<Point2D> for f32 {
    type Output = Point2D;
    
    fn mul(self, point: Point2D) -> Point2D {
        point * self
    }
}

// Implement Div trait for scalar division
impl Div<f32> for Point2D {
    type Output = Point2D;
    
    fn div(self, scalar: f32) -> Point2D {
        Point2D::new(self.x / scalar, self.y / scalar)
    }
}

// Example usage in game logic
fn demonstrate_point_operations() {
    let player_pos = Point2D::new(100.0, 200.0);
    let enemy_pos = Point2D::new(150.0, 250.0);
    
    // Vector from player to enemy
    let direction = enemy_pos - player_pos;
    println\!("Direction vector: {}", direction);
    
    // Normalize and scale for movement
    let movement = direction.normalize() * 50.0;  // 50 units/second
    println\!("Movement vector: {}", movement);
    
    // New player position after movement
    let new_pos = player_pos + movement;
    println\!("New position: {}", new_pos);
    
    // Distance check
    let distance = player_pos.distance_to(&enemy_pos);
    println\!("Distance: {:.2}", distance);
    
    // Rotation example
    let rotated = direction.rotate(std::f32::consts::PI / 4.0);  // 45 degrees
    println\!("Rotated direction: {}", rotated);
}

// Integration with game entities
#[derive(Debug, Clone)]
struct Player {
    position: Point2D,
    velocity: Point2D,
    health: i32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            position: Point2D::new(x, y),
            velocity: Point2D::origin(),
            health: 100,
        }
    }
    
    fn update(&mut self, dt: f32) {
        // Apply velocity to position
        self.position = self.position + self.velocity * dt;
        
        // Apply friction
        self.velocity = self.velocity * 0.95;
    }
    
    fn move_towards(&mut self, target: Point2D, speed: f32) {
        let direction = (target - self.position).normalize();
        self.velocity = direction * speed;
    }
    
    fn distance_to(&self, point: Point2D) -> f32 {
        self.position.distance_to(&point)
    }
}

fn game_example() {
    let mut player = Player::new(0.0, 0.0);
    let target = Point2D::new(100.0, 100.0);
    
    // Move towards target
    player.move_towards(target, 200.0);
    
    // Simulate movement
    for frame in 0..10 {
        player.update(1.0 / 60.0);  // 60 FPS
        println\!("Frame {}: Player at {}", frame, player.position);
        
        if player.distance_to(target) < 5.0 {
            println\!("Reached target\!");
            break;
        }
    }
}

## Step-by-Step Implementation:

1. Create a basic Point2D struct with x, y fields
2. Implement Debug and Clone traits (use derive macro)
3. Add basic methods (new, origin, distance_to)
4. Implement Display trait for pretty printing
5. Implement Add trait for vector addition
6. Implement Sub trait for vector subtraction
7. Implement Mul trait for scalar multiplication
8. Add advanced math methods (normalize, rotate, dot_product)
9. Create a simple Player struct that uses Point2D
10. Test all operations with a small game simulation
