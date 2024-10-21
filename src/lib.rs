/*!

**draw-i16** is a 2D shape and font rasterizing middleware
tailored to small LCD output and embedded systems. It uses i16
values for coordinates and no floating point arithmetics.
The library is no_std compatible.

- Shape rasterizing feature: provide a pixel setting function and use the library
  to draw lines, curves, ellipses and texts. One may use is a standard pixel-setting type.
- Text rasterizing feature: use a tiny vector font definition designed to be lightweight
  and scalable on low-resolution displays. Provide the char-to-set-of-commands converter
  and call the text drawing utilities. One may use the standard converter containing
  latin, extended latin, greek and cyrillic characters of ~8 kbyte size, which is available
  under `font_data` feature.

**Note**: draw-i16 depends on the unstable nightly-only [`ranged_integers`] library.

## How to use shapes

**Briefly**: implement [`Drawable`] on some type and use [`DrawableMethods`].

- Define a type representing the pixel colour. The library is all about manipulating
  pixel colours, so, the type will be used in every drawing procedure. The colour must
  be a small copyable type:
  - Use `()` if only rasterization coordinates are needed, and the colour doesn't matter.
  - Use [`bool`] or 2-variant enum for the displays having black-or-white pixels only.
  - Use [`Colour565`] for 16-bit RGB-colour without alpha channel.
  - Use [char] or [u8] for ASCII graphics.
  - Use a custom colour (like `[u8; 3]` for rgb) or any data type.

- Create a drawable type by implementing the [`Drawable<ColourType>`] trait for it.
  One needs to define the way to get the canvas size and set a pixel at a point, and optionally
  some speeding-up helper methods. This library contains the [`Canvas`] which
  implements it as a memory buffer of pixels.

```ignore
struct Lcd;
impl Drawable<Colour565> for Lcd {
    unsafe fn _size(&self) -> V2 {
        v2(128, 64)  // assume there is a landscape 128x64 display
    }
    unsafe fn _pixel(&mut self, pos: V2, colour: Colour) {
        // Do some job to set the pixel at a given coordinate to the given value
    }
}
```

- The trait [`DrawableMethods`] is implemented for any type
  that implements [`Drawable`]. Use the trait's methods to interact
  with the drawable.

```
# use draw_i16::*;
# struct Lcd;
# impl Drawable<Colour565> for Lcd {
# fn _size(&self) -> V2{v2(128, 64)}
# fn _clear(&mut self, colour: Colour565) {}
# unsafe fn _pixel(&mut self, pos: V2, colour: Colour565) {} }
let mut display = Lcd;

let size : V2 = display.size();

display.clear(Colour565::BLACK);

// Draw a cross with thick yellow lines
display.line(v2(10, 10), size-v2(10, 10), Colour565::YELLOW, 5);
display.line(v2(size.x-10, 10), v2(10, size.y-10), Colour565::YELLOW, 5);

// Repeat a cross with thin blue lines
display.line(v2(10, 10), size-v2(10, 10), Colour565::BLUE, 1);
display.line(v2(size.x-10, 10), v2(10, size.y-10), Colour565::BLUE, 1);

// Draw a rectangle contour
display.rect(v2(10, 10), size-v2(10, 10), Colour565::WHITE);

// Draw a quadratic bezier
display.quad_bezier(v2(10, size.y/2), v2(size.x/2, 0), v2(size.x-10, size.y/2), Colour565::GREEN, 1);
display.quad_bezier(v2(10, size.y/2), v2(size.x/2, size.y), v2(size.x-10, size.y/2), Colour565::GREEN, 1);

// Generate and display text with monospace 7x12 font
let text = "Hello";
let font_size = v2(7, 12);
let rect_start = size/2 - v2(font_size.x * text.len() as i16 / 2, font_size.y/2);
let rect_end = size/2 + v2(font_size.x * text.len() as i16 / 2, font_size.y/2);
display.rect_fill(rect_start, rect_end, Colour565::BLACK);
# #[cfg(feature="font_data")]
// available under "font_data" feature:
display.text(text, font_size, v2(size.x/2 - 7*5/2, size.y/2 - 12/2), Colour565::WHITE);
```

## How to use fonts

**Way 1**:

- Just use the built-in font data with the `DrawableMethods.text` or `DrawableMethods.symbol` functions.
  The <span class="stab portability" title="Available on crate feature `font_data` only">`font_data`</span>
  crate feature must be enabled for it.

**Way 2**:

- Create a set of [`GlyphTable`](font::GlyphTable)s or a different type implementing
  [`GlyphProvider`](font::GlyphProvider) trait, which converts
  a characer code to a set of commands for glyph drawing. See the [font] module documentation
  for details. One can use [a gui tool](https://github.com/disiamylborane/draw-i16-fontviewer)
  to create or edit the font.

- Use the `DrawableMethods.text_with_provider` or `DrawableMethods.symbol_with_provider` functions passing
  the text and a custom [`GlyphProvider`](font::GlyphProvider).
*/

#![no_std]
#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_exprs)]
#![feature(doc_cfg)]
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod v2;
pub use v2::{V2, v2};

pub mod drawable;
pub use drawable::Drawable;
pub use drawable::DrawableMethods;

pub mod font;
pub use font::fontsize_to_glyphsize;

#[cfg(any(feature="font_data", doc))]
#[doc(cfg(feature="font_data"))]
pub mod font_data;

mod helpers;
mod canvas;
mod colour;

pub use canvas::{Canvas, CanvasSizeMismatchError};
pub use colour::Colour565;
pub use helpers::Stencil;
pub use helpers::{Rotator, RotationType};
