# draw-i16

**Draw_i16** is a 2D graphics and shape rasterizing middleware
tailored to small LCD output and embedded systems. It uses i16
values for coordinates and no floating point arithmetics.
The library is no_std compatible.

Currently supports the following primitives:

* Lines
* Bézier curves
* A simple vector font (latin, cyrillic, greek)

## Library overview

All the painting routines are performed on a `Drawable`
trait objects. To use the custom paintable one must implement
the Drawable trait on a custom type.

## Usage

```rust

// External implementation
struct Lcd;
impl Drawable for Lcd {
    ...
}

// Library usage
let mut _lcd_instance = Lcd;
let display : &mut dyn Drawable = &mut _lcd_instance;

let size : V2 = display.size();

display.clear(Colour::BLACK);

// Draw a cross with thick yellow lines
display.line(v2!(10, 10), size-v2!(10, 10), Colour::YELLOW, 5);
display.line(v2!(size.x-10, 10), v2!(10, size.y-10), Colour::YELLOW, 5);

// Repeat a cross with thin blue lines
display.line(v2!(10, 10), size-v2!(10, 10), Colour::BLUE, 1);
display.line(v2!(size.x-10, 10), v2!(10, size.y-10), Colour::BLUE, 1);

// Draw a rectangle contour
display.rect(v2!(10, 10), size-v2!(10, 10), Colour::WHITE);

// Draw a quadratic bezier
display.quad_bezier(v2!(10, size.y/2), v2!(size.x/2, 0), v2!(size.x-10, size.y/2), Colour::GREEN);
display.quad_bezier(v2!(10, size.y/2), v2!(size.x/2, size.y), v2!(size.x-10, size.y/2), Colour::GREEN);

// Generate and display text with monospace 7x12 font
let text = "Hello";
let font_size = v2![7, 12];
let rect_start = size/2 - v2![font_size.x * text.len() / 2, font_size.y/2];
let rect_end = size/2 + v2![font_size.x * text.len() / 2, font_size.y/2];
display.rect_fill(rect_start, rect_end, Colour::BLACK);
display.text(text, font_size, v2!(size.x/2 - 7*5/2, size.y/2 - 12/2), Colour::WHITE);

```
