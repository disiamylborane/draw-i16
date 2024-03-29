/*!

**Draw_i16** is a 2D graphics and shape rasterizing middleware
tailored to small LCD output and embedded systems. It uses i16
values for coordinates and no floating point arithmetics. 
The library is no_std compatible.

Currently supports the following primitives:

* Lines
* Bézier curves
* A simple vector font (latin, cyrillic, greek)

# Library overview

All the painting routines are performed on a [`Drawable`](trait.Drawable.html#methods)
trait objects. To use the custom paintable one must implement 
the Drawable trait on a custom type.

The library extensively use [V2](struct.V2.html) (2D vector with i16 coordinates) 
and [Colour](struct.Colour.html) (16-bit colour conversible to different formats).

The [helpers](helpers/index.html) module contains secondary Drawables such as
Stencil (a window to another Drawable) and Rotor (a side-swapper for a Drawable).

The [canvas](canvas/index.html) module contains a primary memory buffer structure
`Canvas`. It can be used as a framebuffer or an intermediate buffer that
can be applied to a `Drawable` after.

# Usage

```

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

*/

#![no_std]
#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_exprs)]
#![deny(missing_docs)]
#![warn(missing_doc_code_examples)]

#[macro_use]
mod v2;
pub use v2::V2;

mod drawable;
pub use drawable::Drawable;
//pub use drawable::AsDrawable;
pub use drawable::DrawableMethods;

pub use font::_glyphsize_by_covnent as glyphsize;

extern crate ranged_integers;

pub use ranged_integers::*;

mod font;
mod font_data;
pub mod helpers;
pub mod canvas;

/// An error returned by `Canvas::new()` on size mismatch
#[derive(Debug)]
pub struct SizeMismatchError;


mod colour;
pub use colour::Colour;
