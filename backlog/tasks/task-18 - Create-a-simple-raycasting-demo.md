---
id: task-18
title: Create a simple raycasting demo
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn about the core Doom rendering technique by implementing simple raycasting. This introduces 3D concepts in 2D.

## Acceptance Criteria

- [ ] Casts rays from player position
- [ ] Finds where rays hit walls
- [ ] Calculates distance to walls
- [ ] Draws a simple 3D-like view

## Implementation Plan

1. Learn about raycasting theory (sending rays from player)
2. Cast rays in multiple directions from player position
3. Calculate intersections with walls using line math
4. Measure distance to each wall intersection
5. Draw vertical lines on screen based on distance (closer = taller)
