# 🐢 turtle-svg

## 📖 How to build and run ?

1. Install the dependencies
    - `cargo`

## 📜 Usage example

```rust
use turtle_svg::{
    turtle::TurtleSvg,
    color::ColorPre
};

fn main() {
    let mut turtle = TurtleSvg::new();
    
    // Pen settings
    turtle.set_pen_size(5.);
    turtle.set_background_color(ColorPre::None);
    
    // Square
    turtle.set_pen_color(ColorPre::Red);
    turtle.forward(40.);
    turtle.right(90.);
    turtle.set_pen_color(ColorPre::Blue);
    turtle.forward(40.);
    turtle.right(90.);
    turtle.set_pen_color(ColorPre::Green);
    turtle.forward(40.);
    turtle.right(90.);
    turtle.set_pen_color((127, 127, 127));
    turtle.forward(40.);

    // Drawing size, name
    turtle.drawing_mut().set_size((300., 100.));
    turtle.drawing_mut().set_center((20., 20.));
    turtle.drawing_mut().save_svg("square.svg");
}
```

## 🖼️ Preview

![Square](img/square.svg)

## ℹ️ Documentation

Run `cargo doc --open` to read the documentation in the browser.
