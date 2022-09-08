# turtle-svg

## How to build and run ?

1. Install the dependencies
    - `cargo`

## Usage example

```rust
use turtle_svg::{
    turtle::TurtleSvg,
    color::ColorPre
};

fn main() {
    let mut turtle = TurtleSvg::new();
    
    // Pen settings
    turtle.set_pen_size(5.);
    turtle.set_pen_color(ColorPre::Red);
    turtle.set_background_color(ColorPre::None);

    // Square
    turtle.forward(40.);
    turtle.right(90.);
    turtle.forward(40.);
    turtle.right(90.);
    turtle.forward(40.);
    turtle.right(90.);
    turtle.forward(40.);

    // Drawing size, name
    turtle.drawing_mut().set_size((300., 100.));
    turtle.drawing_mut().set_center((20., 20.));
    turtle.drawing_mut().save_svg("red_square.svg");
}
```

## Documentation

Run `cargo doc --open` to read the documentation in the browser.
