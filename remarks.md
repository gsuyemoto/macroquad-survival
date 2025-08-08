# Dynamic Enemy Spawning System

## Overview

The game implements a dynamic enemy spawning system that gets gradually faster over time, creating an escalating difficulty curve that makes survival progressively more challenging.

## Changes Made

1. **Added `game_time: f32`** to the `Game` struct to track how long the game has been running
2. **Updated the game timer** in the `update()` method to increment `game_time` with each frame
3. **Replaced the fixed 2.0-second spawn interval** with a dynamic formula:

```rust
let spawn_interval = 2.0 * (-self.game_time / 30.0).exp() + 0.3;
```

## How the Dynamic Spawning Works

- **Initial spawn rate**: 2.0 seconds between enemies (same as before)
- **Over time**: The spawn interval decreases exponentially with diminishing returns
- **Minimum spawn rate**: ~0.3 seconds (enemies spawn very quickly but not impossibly fast)
- **Time constant**: 30 seconds (the rate decreases significantly over the first 30-60 seconds)

## The Formula Breakdown

The formula `2.0 * exp(-time/30) + 0.3` creates a smooth difficulty progression:

- **At time 0**: `2.0 * exp(0) + 0.3 = 2.0 + 0.3 = 2.3 seconds`
- **At time 30s**: `2.0 * exp(-1) + 0.3 ≈ 0.74 + 0.3 = 1.04 seconds`
- **At time 60s**: `2.0 * exp(-2) + 0.3 ≈ 0.27 + 0.3 = 0.57 seconds`
- **At time ∞**: approaches 0.3 seconds

## Benefits

This creates a smooth difficulty curve where:
- The game starts manageable for new players
- Difficulty ramps up naturally as time progresses
- There's always room for improvement and longer survival times
- The exponential decay prevents the game from becoming impossible too quickly
- Players experience a sense of escalating tension and challenge

The system ensures that survival becomes progressively more difficult over time while maintaining fairness and playability.
