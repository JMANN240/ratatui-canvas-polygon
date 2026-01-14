# ratatui-canvas-polygon

A shape library for ratatui

## Installation

Install with cargo:

`cargo add ratatui-canvas-polygon`

## Usage

Use like any other shape:

```rust
let canvas = Canvas::default()
    .x_bounds([0.0, 1.0])
    .y_bounds([0.0, 1.0])
    .paint(|context| {
        context.draw(&Triangle::new(
            [(0.25, 0.25), (0.25, 0.75), (0.5, 0.75)],
            Color::White,
        ));
        context.draw(&Triangle::new(
            [(0.75, 0.25), (0.75, 0.75), (0.5, 0.75)],
            Color::White,
        ));
    });
```

Draws:

```
                                                  
            ⢸⣿⣿⣿⣿⣿⣿⣿⠿⠟⠛⠋⠉⠉⠉⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿             
            ⠸⠿⠟⠛⠋⠉              ⠉⠉⠛⠻⠿             
                                                  
```