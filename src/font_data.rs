
use super::font::*;

pub static TABLES: [GlyphTable; 4] = [
    GLYPHS_LATIN,
    GLYPHS_LATIN_EXTA_UNIMPL,
    GLYPHS_CYRILLIC,
    GLYPHS_GREEK,
];

use crate::ranged_integers::*;
use GlyphConnectionType::*;
macro_rules! g2set {
    ($x1:literal, $y1:literal, $z1:expr; $x2:literal, $y2:literal, $z2:expr) => {
        GlyphStep::to_gdata(
            GlyphStep{coord: GlyphCoord{x: r!([] $x1), y: r!([] $y1)}, tp: $z1},
            GlyphStep{coord: GlyphCoord{x: r!([] $x2), y: r!([] $y2)}, tp: $z2},
        )
    };
}

pub const _GDATA_LATIN_ : [[u8; 3]; 255] = [
    g2set!(0, 0, Break;
            6, 1, Break),
    g2set!(6, 15, Outline { thick: true, update: true };
            6, 23, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
            4, 1, Break),
    g2set!(4, 7, Outline { thick: true, update: true };
            8, 1, Break),
    g2set!(8, 7, Outline { thick: true, update: true };
            3, 1, Break),
    g2set!(3, 24, Outline { thick: true, update: true };
            9, 1, Break),
    g2set!(9, 24, Outline { thick: true, update: true };
            0, 6, Break),
    g2set!(12, 6, Outline { thick: false, update: true };
            0, 18, Break),
    g2set!(12, 18, Outline { thick: false, update: true };
            12, 2, Break),
    g2set!(0, 0, Control;
            0, 7, Outline { thick: false, update: true }),
    g2set!(0, 11, Control;
            6, 12, Outline { thick: true, update: true }),
    g2set!(12, 13, Control;
            12, 18, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
            0, 23, Outline { thick: true, update: true }),
    g2set!(6, 0, Break;
            6, 24, Outline { thick: false, update: true }),
    g2set!(6, 4, Break;
            6, 1, Control),
    g2set!(3, 1, Outline { thick: false, update: true };
            0, 1, Control),
    g2set!(0, 4, Outline { thick: false, update: true };
            0, 7, Control),
    g2set!(4, 7, Outline { thick: false, update: true };
            12, 1, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
            7, 17, Outline { thick: false, update: true }),
    g2set!(12, 17, Control;
            12, 21, Outline { thick: false, update: true }),
    g2set!(12, 24, Control;
            9, 24, Outline { thick: false, update: true }),
    g2set!(6, 24, Control;
            6, 21, Outline { thick: false, update: true }),
    g2set!(12, 24, Break;
            6, 11, Outline { thick: false, update: true }),
    g2set!(0, 1, Control;
            12, 2, Outline { thick: false, update: false }),
    g2set!(0, 18, Control;
            3, 24, Outline { thick: true, update: true }),
    g2set!(9, 26, Control;
            12, 15, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
            6, 6, Outline { thick: true, update: true }),
    g2set!(7, 1, Break;
            0, 12, Control),
    g2set!(7, 24, Outline { thick: true, update: true };
            5, 1, Break),
    g2set!(12, 12, Control;
            5, 24, Outline { thick: true, update: true }),
    g2set!(6, 12, Break;
            6, 5, Outline { thick: false, update: false }),
    g2set!(1, 9, Outline { thick: false, update: false };
            11, 9, Outline { thick: false, update: false }),
    g2set!(2, 17, Outline { thick: true, update: false };
            10, 17, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
            12, 12, Outline { thick: true, update: true }),
    g2set!(6, 5, Break;
            6, 19, Outline { thick: false, update: true }),
    g2set!(8, 21, Break;
            8, 24, Outline { thick: true, update: true }),
    g2set!(6, 28, Outline { thick: false, update: true };
            3, 12, Break),
    g2set!(9, 12, Outline { thick: false, update: true };
            6, 23, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
            11, 2, Break),
    g2set!(1, 23, Outline { thick: true, update: true };
            6, 1, Break),
    g2set!(0, 2, Control;
            1, 12, Outline { thick: true, update: true }),
    g2set!(0, 23, Control;
            6, 24, Outline { thick: true, update: true }),
    g2set!(12, 23, Control;
            11, 12, Outline { thick: true, update: true }),
    g2set!(12, 2, Control;
            6, 1, Outline { thick: true, update: true }),
    g2set!(11, 6, Break;
            1, 18, Outline { thick: false, update: true }),
    g2set!(2, 12, Break;
            9, 1, Outline { thick: false, update: true }),
    g2set!(9, 24, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(0, 1, Control;
            7, 2, Outline { thick: false, update: true }),
    g2set!(14, 3, Control;
            7, 14, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: false, update: true };
            12, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Control;
            12, 5, Outline { thick: false, update: true }),
    g2set!(12, 12, Control;
            4, 12, Outline { thick: false, update: true }),
    g2set!(12, 12, Control;
            12, 19, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
            0, 23, Outline { thick: true, update: true }),
    g2set!(9, 24, Break;
            9, 1, Outline { thick: true, update: true }),
    g2set!(0, 17, Outline { thick: false, update: true };
            12, 17, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: false, update: false };
            0, 12, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
            12, 19, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 2, Break;
            0, 1, Control),
    g2set!(0, 18, Outline { thick: true, update: true };
            2, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
            10, 24, Control),
    g2set!(12, 19, Outline { thick: true, update: true };
            12, 11, Control),
    g2set!(1, 14, Outline { thick: true, update: true };
            12, 1, Outline { thick: false, update: true }),
    g2set!(6, 24, Outline { thick: true, update: true };
            6, 12, Break),
    g2set!(12, 10, Control;
            12, 6, Outline { thick: false, update: true }),
    g2set!(12, 1, Control;
            6, 1, Outline { thick: false, update: true }),
    g2set!(0, 1, Control;
            0, 6, Outline { thick: false, update: true }),
    g2set!(0, 10, Control;
            6, 12, Outline { thick: false, update: true }),
    g2set!(12, 14, Control;
            12, 20, Outline { thick: true, update: true }),
    g2set!(6, 30, Control;
            0, 20, Outline { thick: true, update: true }),
    g2set!(0, 14, Control;
            6, 12, Outline { thick: true, update: true }),
    g2set!(1, 23, Break;
            13, 25, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
            12, 2, Control),
    g2set!(6, 2, Outline { thick: true, update: true };
            0, 3, Control),
    g2set!(0, 8, Outline { thick: true, update: true };
            3, 15, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
            6, 6, Break),
    g2set!(6, 8, Outline { thick: true, update: true };
            6, 22, Break),
    g2set!(6, 23, Outline { thick: true, update: true };
            7, 6, Break),
    g2set!(7, 7, Outline { thick: true, update: true };
            8, 23, Break),
    g2set!(5, 27, Outline { thick: false, update: true };
            12, 5, Break),
    g2set!(0, 14, Outline { thick: true, update: true };
            12, 24, Outline { thick: true, update: true }),
    g2set!(0, 10, Break;
            12, 10, Outline { thick: false, update: true }),
    g2set!(0, 14, Break;
            12, 14, Outline { thick: false, update: true }),
    g2set!(0, 5, Break;
            12, 14, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 5, Break),
    g2set!(0, 1, Control;
            6, 1, Outline { thick: true, update: true }),
    g2set!(14, 3, Control;
            10, 8, Outline { thick: true, update: true }),
    g2set!(6, 11, Control;
            6, 15, Outline { thick: true, update: true }),
    g2set!(6, 24, Break;
            6, 26, Outline { thick: true, update: true }),
    g2set!(8, 8, Break;
            8, 18, Outline { thick: false, update: false }),
    g2set!(3, 8, Control;
            3, 12, Outline { thick: false, update: true }),
    g2set!(3, 17, Control;
            9, 18, Outline { thick: false, update: true }),
    g2set!(13, 16, Control;
            13, 10, Outline { thick: false, update: true }),
    g2set!(13, 4, Control;
            6, 4, Outline { thick: false, update: true }),
    g2set!(0, 3, Control;
            0, 13, Outline { thick: false, update: true }),
    g2set!(1, 24, Control;
            13, 24, Outline { thick: false, update: true }),
    g2set!(0, 24, Break;
            6, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
            2, 17, Break),
    g2set!(10, 17, Outline { thick: false, update: true };
            0, 24, Outline { thick: true, update: true }),
    g2set!(13, 25, Control;
            12, 18, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
            0, 12, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
            12, 6, Outline { thick: false, update: true }),
    g2set!(12, 1, Control;
            0, 1, Outline { thick: false, update: true }),
    g2set!(12, 2, Break;
            0, 0, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
            0, 25, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
            12, 12, Outline { thick: true, update: true }),
    g2set!(12, 1, Control;
            0, 1, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: false };
            0, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
            12, 1, Outline { thick: true, update: false }),
    g2set!(0, 12, Outline { thick: true, update: true };
            9, 12, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
            12, 2, Break),
    g2set!(0, 1, Control;
            0, 12, Outline { thick: true, update: true }),
    g2set!(0, 25, Control;
            12, 23, Outline { thick: true, update: true }),
    g2set!(12, 12, Outline { thick: false, update: true };
            7, 12, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 12, Break),
    g2set!(12, 12, Outline { thick: false, update: true };
            12, 1, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
            3, 1, Break),
    g2set!(9, 1, Outline { thick: false, update: true };
            6, 1, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
            3, 24, Break),
    g2set!(9, 24, Outline { thick: false, update: true };
            10, 1, Break),
    g2set!(10, 20, Outline { thick: true, update: true };
            8, 27, Control),
    g2set!(3, 23, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
            12, 1, Outline { thick: false, update: false }),
    g2set!(12, 24, Outline { thick: false, update: false };
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
            0, 24, Outline { thick: true, update: false }),
    g2set!(6, 14, Outline { thick: false, update: true };
            12, 1, Outline { thick: false, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: false }),
    g2set!(12, 24, Outline { thick: false, update: true };
            12, 1, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
            0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
            0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
            12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
            12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: false }),
    g2set!(12, 1, Control;
            12, 6, Outline { thick: true, update: true }),
    g2set!(12, 14, Control;
            0, 14, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
            0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
            0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
            12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
            12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
            6, 14, Break),
    g2set!(13, 24, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: false }),
    g2set!(12, 1, Control;
            12, 6, Outline { thick: true, update: true }),
    g2set!(12, 14, Control;
            0, 14, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
            12, 2, Break),
    g2set!(0, 0, Control;
            0, 7, Outline { thick: false, update: true }),
    g2set!(0, 11, Control;
            6, 12, Outline { thick: true, update: true }),
    g2set!(12, 13, Control;
            12, 18, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
            0, 23, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
            6, 1, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
            0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
            8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
            12, 1, Outline { thick: true, update: true }),
    g2set!(6, 24, Outline { thick: true, update: true };
            12, 1, Outline { thick: true, update: true }),
    g2set!(1, 24, Outline { thick: true, update: true };
            6, 1, Outline { thick: true, update: true }),
    g2set!(11, 24, Outline { thick: true, update: true };
            12, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
            12, 1, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
            6, 11, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: false };
            6, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
            9, 1, Break),
    g2set!(3, 1, Outline { thick: true, update: true };
            3, 24, Outline { thick: true, update: true }),
    g2set!(9, 24, Outline { thick: true, update: true };
            1, 2, Break),
    g2set!(11, 23, Outline { thick: true, update: true };
            3, 1, Break),
    g2set!(9, 1, Outline { thick: true, update: true };
            9, 24, Outline { thick: true, update: true }),
    g2set!(3, 24, Outline { thick: true, update: true };
            3, 7, Break),
    g2set!(6, 2, Outline { thick: true, update: true };
            9, 7, Outline { thick: true, update: true }),
    g2set!(0, 23, Break;
            12, 23, Outline { thick: true, update: true }),
    g2set!(4, 1, Break;
            8, 6, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
            14, 6, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
            1, 24, Control),
    g2set!(1, 21, Outline { thick: true, update: true };
            0, 14, Control),
    g2set!(11, 15, Outline { thick: true, update: true };
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
            12, 18, Outline { thick: false, update: true }),
    g2set!(12, 12, Control;
            0, 12, Outline { thick: false, update: true }),
    g2set!(12, 7, Break;
            0, 7, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
            0, 24, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
            12, 1, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
            0, 25, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
            0, 10, Control),
    g2set!(12, 11, Outline { thick: false, update: true };
            0, 17, Break),
    g2set!(12, 17, Outline { thick: false, update: true };
            12, 10, Control),
    g2set!(6, 10, Outline { thick: false, update: true };
            0, 10, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
            0, 24, Control),
    g2set!(6, 24, Outline { thick: false, update: true };
            11, 24, Outline { thick: false, update: true }),
    g2set!(3, 24, Break;
            3, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
            3, 3, Control),
    g2set!(10, 4, Outline { thick: true, update: true };
            12, 24, Break),
    g2set!(0, 24, Control;
            0, 17, Outline { thick: true, update: true }),
    g2set!(0, 10, Control;
            12, 10, Outline { thick: true, update: true }),
    g2set!(12, 25, Outline { thick: true, update: true };
            12, 30, Control),
    g2set!(1, 28, Outline { thick: true, update: true };
            0, 0, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 12, Break),
    g2set!(14, 6, Control;
            12, 24, Outline { thick: true, update: true }),
    g2set!(6, 4, Break;
            6, 5, Outline { thick: true, update: true }),
    g2set!(6, 11, Break;
            6, 24, Outline { thick: true, update: true }),
    g2set!(6, 4, Break;
            6, 5, Outline { thick: true, update: true }),
    g2set!(6, 11, Break;
            6, 25, Outline { thick: true, update: true }),
    g2set!(6, 29, Control;
            3, 29, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 15, Break),
    g2set!(12, 24, Outline { thick: true, update: false };
            12, 8, Outline { thick: false, update: true }),
    g2set!(4, 1, Break;
            7, 1, Outline { thick: false, update: true }),
    g2set!(7, 24, Outline { thick: false, update: true };
            0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: false };
            6, 7, Control),
    g2set!(6, 23, Outline { thick: false, update: true };
            6, 6, Control),
    g2set!(9, 7, Outline { thick: false, update: true };
            12, 7, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
            0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 10, Break),
    g2set!(14, 2, Control;
            12, 24, Outline { thick: true, update: true }),
    g2set!(6, 7, Break;
            0, 7, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
            0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
            12, 24, Control),
    g2set!(12, 16, Outline { thick: true, update: true };
            12, 7, Control),
    g2set!(6, 7, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(0, 31, Outline { thick: true, update: false };
            11, 6, Control),
    g2set!(12, 15, Outline { thick: true, update: true };
            12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
            12, 7, Break),
    g2set!(12, 31, Outline { thick: true, update: false };
            0, 6, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
            0, 24, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 6, Control),
    g2set!(12, 8, Outline { thick: true, update: true };
            12, 7, Break),
    g2set!(0, 6, Control;
            1, 12, Outline { thick: true, update: true }),
    g2set!(3, 14, Control;
            6, 15, Outline { thick: true, update: true }),
    g2set!(10, 16, Control;
            12, 19, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
            0, 24, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
            3, 7, Outline { thick: true, update: true }),
    g2set!(9, 7, Outline { thick: false, update: false };
            2, 25, Control),
    g2set!(9, 24, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(0, 28, Control;
            12, 24, Outline { thick: false, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
            12, 7, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
            2, 24, Outline { thick: true, update: true }),
    g2set!(6, 7, Outline { thick: true, update: true };
            10, 24, Outline { thick: true, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
            12, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(0, 26, Control;
            12, 24, Outline { thick: false, update: true }),
    g2set!(0, 0, Break;
            12, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
            12, 31, Control),
    g2set!(1, 29, Outline { thick: true, update: true };
            0, 7, Break),
    g2set!(12, 7, Outline { thick: false, update: true };
            0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
            8, 1, Break),
    g2set!(4, 1, Control;
            5, 7, Outline { thick: true, update: true }),
    g2set!(6, 13, Control;
            3, 13, Outline { thick: true, update: true }),
    g2set!(6, 13, Control;
            5, 19, Outline { thick: true, update: true }),
    g2set!(4, 25, Control;
            8, 25, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
            6, 29, Outline { thick: false, update: true }),
    g2set!(4, 1, Break;
            8, 1, Control),
    g2set!(7, 7, Outline { thick: true, update: true };
            6, 13, Control),
    g2set!(10, 13, Outline { thick: true, update: true };
            6, 13, Control),
    g2set!(7, 19, Outline { thick: true, update: true };
            8, 25, Control),
    g2set!(4, 25, Outline { thick: true, update: true };
            0, 0, Break),
];

pub const GLYPHS_LATIN : GlyphTable = GlyphTable {
    basechar: ' ',
    addr: & [0, 1, 5, 9, 17, 28, 44, 52, 54, 57, 60, 66, 70, 73, 75, 77, 79, 90, 93, 100, 108, 112, 118, 127, 129, 144, 153, 157, 161, 164, 168, 171, 180, 194, 199, 208, 213, 218, 223, 227, 234, 239, 245, 249, 253, 255, 259, 262, 271, 276, 287, 293, 302, 305, 310, 312, 316, 319, 322, 325, 329, 331, 335, 338, 340, 342, 349, 354, 359, 365, 374, 379, 387, 392, 396, 402, 406, 409, 417, 422, 431, 437, 443, 447, 456, 461, 465, 468, 473, 477, 485, 489, 498, 500, 509],
    data: GlyphData(&_GDATA_LATIN_),
};

pub const _GDATA_CYRILLIC_ : [[u8; 3]; 322] = [
    g2set!(0, 4, Break;
           12, 4, Outline { thick: true, update: false }),
    g2set!(0, 14, Outline { thick: true, update: true };
           9, 14, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(7, 0, Break;
           5, 3, Outline { thick: true, update: true }),
    g2set!(0, 4, Break;
           12, 4, Outline { thick: true, update: false }),
    g2set!(0, 14, Outline { thick: true, update: true };
           9, 14, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(4, 1, Break;
           3, 1, Outline { thick: true, update: true }),
    g2set!(8, 1, Break;
           9, 1, Outline { thick: true, update: true }),
    g2set!(6, 1, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(3, 24, Outline { thick: true, update: true };
           3, 12, Break),
    g2set!(13, 9, Control;
           12, 18, Outline { thick: true, update: true }),
    g2set!(13, 25, Control;
           7, 24, Outline { thick: true, update: true }),
    g2set!(12, 4, Break;
           0, 4, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           7, 0, Break),
    g2set!(5, 3, Outline { thick: true, update: true };
           12, 1, Break),
    g2set!(0, 1, Control;
           0, 12, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
           10, 12, Outline { thick: true, update: true }),
    g2set!(12, 2, Break;
           0, 0, Control),
    g2set!(0, 7, Outline { thick: false, update: true };
           0, 11, Control),
    g2set!(6, 12, Outline { thick: true, update: true };
           12, 13, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(9, 1, Outline { thick: false, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 24, Break),
    g2set!(9, 24, Outline { thick: false, update: true };
           3, 3, Break),
    g2set!(9, 3, Outline { thick: false, update: true };
           6, 3, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 24, Break),
    g2set!(9, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(3, 0, Outline { thick: true, update: true };
           9, 1, Break),
    g2set!(9, 0, Outline { thick: true, update: true };
           10, 1, Break),
    g2set!(10, 20, Outline { thick: true, update: true };
           8, 27, Control),
    g2set!(3, 23, Outline { thick: true, update: true };
           0, 21, Break),
    g2set!(0, 24, Control;
           2, 24, Outline { thick: true, update: true }),
    g2set!(4, 24, Control;
           3, 1, Outline { thick: true, update: true }),
    g2set!(8, 1, Outline { thick: true, update: true };
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 18, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
           8, 11, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(7, 12, Outline { thick: true, update: true };
           7, 1, Break),
    g2set!(7, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 12, Control),
    g2set!(7, 12, Outline { thick: true, update: true };
           6, 1, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           3, 24, Outline { thick: true, update: true }),
    g2set!(3, 12, Break;
           13, 9, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
           12, 1, Outline { thick: false, update: false }),
    g2set!(12, 24, Outline { thick: false, update: false };
           7, 0, Break),
    g2set!(5, 3, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(5, 0, Break;
           7, 4, Outline { thick: true, update: true }),
    g2set!(0, 18, Control;
           9, 17, Outline { thick: true, update: true }),
    g2set!(12, 1, Break;
           12, 24, Control),
    g2set!(3, 24, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(5, 1, Break;
           6, 3, Control),
    g2set!(8, 1, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(6, 24, Break;
           6, 28, Outline { thick: true, update: true }),
    g2set!(0, 24, Break;
           6, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 17, Break),
    g2set!(10, 17, Outline { thick: false, update: true };
           12, 1, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 26, Control),
    g2set!(12, 17, Outline { thick: true, update: true };
           12, 10, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(13, 25, Control;
           12, 18, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
           0, 12, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
           12, 6, Outline { thick: false, update: true }),
    g2set!(12, 1, Control;
           0, 1, Outline { thick: false, update: true }),
    g2set!(12, 4, Break;
           0, 4, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           3, 24, Break),
    g2set!(3, 1, Outline { thick: true, update: true };
           9, 1, Outline { thick: true, update: true }),
    g2set!(9, 24, Outline { thick: true, update: true };
           0, 29, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(12, 29, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: false }),
    g2set!(0, 12, Outline { thick: true, update: true };
           9, 12, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 1, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(12, 5, Outline { thick: false, update: true };
           12, 12, Control),
    g2set!(4, 12, Outline { thick: false, update: true };
           12, 12, Control),
    g2set!(12, 19, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           4, 1, Break),
    g2set!(6, 3, Control;
           8, 1, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(12, 1, Outline { thick: false, update: false };
           12, 24, Outline { thick: false, update: false }),
    g2set!(0, 21, Break;
           0, 24, Control),
    g2set!(2, 24, Outline { thick: true, update: true };
           4, 24, Control),
    g2set!(3, 1, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: false }),
    g2set!(6, 14, Outline { thick: false, update: true };
           12, 1, Outline { thick: false, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
           12, 12, Outline { thick: false, update: true }),
    g2set!(12, 1, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: false }),
    g2set!(12, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 1, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           12, 2, Break),
    g2set!(0, 0, Control;
           0, 12, Outline { thick: true, update: true }),
    g2set!(0, 25, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
           0, 18, Control),
    g2set!(9, 17, Outline { thick: true, update: true };
           12, 1, Break),
    g2set!(12, 24, Control;
           3, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           6, 24, Break),
    g2set!(6, 1, Outline { thick: true, update: true };
           0, 1, Control),
    g2set!(0, 6, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(6, 14, Outline { thick: true, update: true };
           12, 15, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           9, 24, Outline { thick: true, update: true }),
    g2set!(9, 1, Outline { thick: true, update: false };
           13, 24, Outline { thick: true, update: true }),
    g2set!(13, 28, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(6, 24, Outline { thick: true, update: true };
           6, 7, Outline { thick: true, update: false }),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           5, 24, Outline { thick: true, update: true }),
    g2set!(5, 7, Outline { thick: true, update: false };
           10, 24, Outline { thick: true, update: true }),
    g2set!(10, 1, Outline { thick: true, update: false };
           13, 24, Outline { thick: true, update: true }),
    g2set!(13, 28, Outline { thick: true, update: true };
           3, 1, Outline { thick: true, update: true }),
    g2set!(3, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 17, Outline { thick: true, update: true };
           12, 11, Control),
    g2set!(3, 11, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(9, 24, Control;
           8, 18, Outline { thick: true, update: true }),
    g2set!(9, 11, Control;
           0, 11, Outline { thick: true, update: true }),
    g2set!(12, 1, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           11, 24, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           11, 11, Control),
    g2set!(0, 11, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 24, Outline { thick: true, update: false };
           1, 12, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(4, 12, Outline { thick: true, update: true };
           4, 24, Control),
    g2set!(8, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(8, 1, Outline { thick: true, update: true };
           4, 1, Control),
    g2set!(4, 12, Outline { thick: true, update: true };
           12, 24, Break),
    g2set!(12, 1, Outline { thick: true, update: true };
           0, 1, Control),
    g2set!(0, 6, Outline { thick: true, update: true };
           0, 12, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           0, 24, Outline { thick: false, update: true }),
    g2set!(0, 8, Break;
           1, 7, Control),
    g2set!(6, 7, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(12, 14, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: false }),
    g2set!(0, 14, Control;
           0, 19, Outline { thick: true, update: true }),
    g2set!(1, 25, Control;
           14, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Break;
           1, 3, Control),
    g2set!(1, 7, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(3, 24, Outline { thick: true, update: true };
           12, 26, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 12, Control),
    g2set!(1, 14, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 19, Outline { thick: true, update: true };
           12, 15, Control),
    g2set!(0, 15, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(12, 11, Outline { thick: false, update: true };
           12, 7, Control),
    g2set!(0, 7, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(12, 7, Outline { thick: false, update: false };
           0, 24, Outline { thick: true, update: false }),
    g2set!(3, 24, Break;
           3, 7, Outline { thick: true, update: true }),
    g2set!(9, 7, Outline { thick: true, update: true };
           9, 24, Outline { thick: true, update: true }),
    g2set!(0, 29, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 29, Outline { thick: true, update: true }),
    g2set!(0, 17, Break;
           12, 17, Outline { thick: false, update: true }),
    g2set!(12, 10, Control;
           6, 10, Outline { thick: false, update: true }),
    g2set!(0, 10, Control;
           0, 17, Outline { thick: false, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: false, update: true }),
    g2set!(11, 24, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 7, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 6, Control;
           12, 10, Outline { thick: false, update: true }),
    g2set!(12, 15, Control;
           4, 15, Outline { thick: false, update: true }),
    g2set!(12, 15, Control;
           12, 20, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(4, 6, Break;
           6, 9, Control),
    g2set!(8, 6, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 15, Break),
    g2set!(12, 7, Outline { thick: false, update: false };
           12, 24, Outline { thick: false, update: false }),
    g2set!(0, 22, Break;
           0, 24, Control),
    g2set!(2, 24, Outline { thick: true, update: true };
           4, 24, Control),
    g2set!(3, 7, Outline { thick: true, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: false };
           6, 16, Outline { thick: false, update: true }),
    g2set!(12, 7, Outline { thick: false, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 15, Break;
           12, 15, Outline { thick: false, update: true }),
    g2set!(12, 7, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 7, Break;
           0, 7, Control),
    g2set!(0, 15, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 15, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(6, 7, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 7, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 31, Outline { thick: true, update: false };
           11, 6, Control),
    g2set!(12, 15, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 25, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           12, 7, Outline { thick: true, update: true }),
    g2set!(6, 7, Break;
           6, 24, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 26, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 31, Control),
    g2set!(1, 29, Outline { thick: true, update: true };
           6, 30, Break),
    g2set!(6, 7, Outline { thick: true, update: true };
           0, 7, Control),
    g2set!(0, 15, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 15, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(6, 7, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           9, 24, Outline { thick: true, update: true }),
    g2set!(9, 7, Outline { thick: true, update: false };
           13, 24, Outline { thick: true, update: true }),
    g2set!(13, 28, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 15, Control;
           12, 15, Outline { thick: true, update: true }),
    g2set!(12, 7, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(6, 24, Outline { thick: true, update: true };
           6, 12, Outline { thick: true, update: false }),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(5, 24, Outline { thick: true, update: true };
           5, 13, Outline { thick: true, update: false }),
    g2set!(10, 24, Outline { thick: true, update: true };
           10, 7, Outline { thick: true, update: false }),
    g2set!(13, 24, Outline { thick: true, update: true };
           13, 28, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           3, 7, Outline { thick: true, update: true }),
    g2set!(3, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 19, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(3, 14, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           9, 24, Control),
    g2set!(8, 19, Outline { thick: true, update: true };
           9, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           11, 24, Control),
    g2set!(11, 19, Outline { thick: true, update: true };
           11, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 7, Control;
           12, 15, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           0, 24, Outline { thick: true, update: false }),
    g2set!(0, 15, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 15, Break),
    g2set!(4, 15, Outline { thick: true, update: true };
           4, 24, Control),
    g2set!(8, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 15, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(8, 7, Outline { thick: true, update: true };
           4, 7, Control),
    g2set!(4, 15, Outline { thick: true, update: true };
           12, 24, Break),
    g2set!(12, 7, Outline { thick: true, update: true };
           0, 7, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 16, Control),
    g2set!(12, 16, Outline { thick: true, update: true };
           0, 24, Outline { thick: false, update: true }),
    g2set!(0, 17, Break;
           12, 17, Outline { thick: false, update: true }),
    g2set!(12, 10, Control;
           6, 10, Outline { thick: false, update: true }),
    g2set!(0, 10, Control;
           0, 17, Outline { thick: false, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: false, update: true }),
    g2set!(11, 24, Outline { thick: false, update: true };
           5, 3, Break),
    g2set!(7, 7, Outline { thick: false, update: true };
           0, 17, Break),
    g2set!(12, 17, Outline { thick: false, update: true };
           12, 10, Control),
    g2set!(6, 10, Outline { thick: false, update: true };
           0, 10, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: false, update: true };
           11, 24, Outline { thick: false, update: true }),
    g2set!(3, 5, Break;
           4, 5, Outline { thick: true, update: true }),
    g2set!(8, 5, Break;
           9, 5, Outline { thick: true, update: true }),
    g2set!(0, 4, Break;
           6, 4, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           3, 24, Outline { thick: true, update: true }),
    g2set!(3, 12, Break;
           13, 9, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           13, 31, Control),
    g2set!(7, 30, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 7, Outline { thick: true, update: false };
           0, 24, Outline { thick: true, update: true }),
    g2set!(8, 1, Break;
           4, 5, Outline { thick: true, update: true }),
    g2set!(12, 7, Break;
           0, 7, Control),
    g2set!(0, 15, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(12, 24, Outline { thick: true, update: false };
           12, 15, Outline { thick: false, update: true }),
    g2set!(12, 7, Break;
           0, 6, Control),
    g2set!(1, 12, Outline { thick: true, update: true };
           3, 14, Control),
    g2set!(6, 15, Outline { thick: true, update: true };
           10, 16, Control),
    g2set!(12, 19, Outline { thick: true, update: true };
           12, 25, Control),
    g2set!(0, 24, Outline { thick: true, update: true };
           6, 4, Break),
    g2set!(6, 5, Outline { thick: true, update: true };
           6, 11, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           6, 10, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(3, 0, Outline { thick: true, update: true };
           9, 1, Break),
    g2set!(9, 0, Outline { thick: true, update: true };
           10, 9, Break),
    g2set!(10, 20, Outline { thick: true, update: true };
           10, 30, Control),
    g2set!(5, 30, Outline { thick: true, update: true };
           0, 21, Break),
    g2set!(0, 24, Control;
           2, 24, Outline { thick: true, update: true }),
    g2set!(4, 24, Control;
           3, 7, Outline { thick: true, update: true }),
    g2set!(8, 7, Outline { thick: true, update: true };
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 19, Outline { thick: true, update: true }),
    g2set!(12, 14, Control;
           8, 14, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 16, Break;
           7, 16, Outline { thick: true, update: true }),
    g2set!(7, 7, Break;
           7, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 20, Outline { thick: true, update: true }),
    g2set!(12, 16, Control;
           7, 16, Outline { thick: true, update: true }),
    g2set!(0, 5, Break;
           6, 5, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           3, 24, Outline { thick: true, update: true }),
    g2set!(3, 15, Break;
           12, 12, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 15, Break),
    g2set!(12, 7, Outline { thick: false, update: false };
           12, 24, Outline { thick: false, update: false }),
    g2set!(7, 3, Break;
           5, 7, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(5, 3, Break;
           7, 6, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 26, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 31, Control),
    g2set!(1, 29, Outline { thick: true, update: true };
           4, 5, Break),
    g2set!(6, 8, Control;
           8, 5, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(6, 24, Break;
           6, 28, Outline { thick: true, update: true }),
];

pub const GLYPHS_CYRILLIC : GlyphTable = GlyphTable {
    basechar: 'Ѐ',
    addr: & [0, 8, 18, 26, 31, 38, 47, 53, 63, 67, 78, 87, 93, 99, 104, 113, 118, 123, 129, 138, 141, 149, 154, 159, 167, 170, 176, 180, 187, 191, 196, 205, 208, 213, 218, 221, 227, 237, 240, 245, 249, 254, 261, 267, 274, 279, 284, 295, 302, 312, 321, 331, 334, 342, 351, 357, 366, 370, 377, 382, 389, 394, 400, 409, 413, 419, 424, 428, 435, 445, 449, 455, 460, 466, 474, 481, 489, 495, 501, 513, 520, 531, 544, 553, 558, 564, 573, 577, 583, 587, 598, 608, 615, 622, 628, 638, 644],
    data: GlyphData(&_GDATA_CYRILLIC_),
};

pub const _GDATA_GREEK_ : [[u8; 3]; 283] = [
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(12, 12, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 15, Outline { thick: true, update: true }),
    g2set!(11, 15, Outline { thick: true, update: true };
           0, 7, Outline { thick: false, update: false }),
    g2set!(12, 1, Outline { thick: true, update: true };
           12, 7, Outline { thick: false, update: true }),
    g2set!(6, 1, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 7, Break;
           3, 3, Outline { thick: false, update: true }),
    g2set!(9, 3, Outline { thick: true, update: true };
           9, 7, Outline { thick: false, update: true }),
    g2set!(6, 3, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(7, 4, Break;
           5, 6, Outline { thick: true, update: true }),
    g2set!(7, 23, Break;
           5, 26, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 23, Break),
    g2set!(8, 26, Outline { thick: true, update: true };
           0, 6, Break),
    g2set!(11, 6, Control;
           12, 15, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 7, Break;
           0, 7, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 16, Break),
    g2set!(8, 16, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(11, 6, Control;
           12, 15, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           0, 23, Outline { thick: true, update: true }),
    g2set!(5, 15, Break;
           7, 15, Outline { thick: true, update: true }),
    g2set!(7, 6, Break;
           7, 7, Outline { thick: true, update: true }),
    g2set!(8, 23, Break;
           5, 27, Outline { thick: false, update: true }),
    g2set!(10, 1, Break;
           10, 20, Outline { thick: true, update: true }),
    g2set!(8, 27, Control;
           3, 23, Outline { thick: true, update: true }),
    g2set!(8, 1, Break;
           4, 6, Outline { thick: true, update: true }),
    g2set!(8, 1, Break;
           4, 6, Outline { thick: true, update: true }),
    g2set!(2, 9, Break;
           4, 9, Outline { thick: true, update: true }),
    g2set!(8, 9, Break;
           10, 9, Outline { thick: true, update: true }),
    g2set!(0, 24, Break;
           6, 4, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 17, Break),
    g2set!(10, 17, Outline { thick: false, update: true };
           4, 1, Break),
    g2set!(1, 5, Outline { thick: true, update: true };
           6, 11, Break),
    g2set!(6, 13, Outline { thick: true, update: true };
           12, 3, Break),
    g2set!(2, 3, Outline { thick: true, update: true };
           2, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
           2, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           4, 1, Break),
    g2set!(1, 5, Outline { thick: true, update: true };
           2, 3, Break),
    g2set!(2, 24, Outline { thick: true, update: true };
           2, 12, Break),
    g2set!(12, 12, Outline { thick: false, update: true };
           12, 3, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 1, Break),
    g2set!(0, 5, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(9, 1, Outline { thick: false, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 24, Break),
    g2set!(9, 24, Outline { thick: false, update: true };
           2, 1, Break),
    g2set!(0, 5, Outline { thick: true, update: true };
           7, 2, Break),
    g2set!(2, 2, Control;
           2, 13, Outline { thick: true, update: true }),
    g2set!(2, 24, Control;
           7, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 13, Outline { thick: true, update: true }),
    g2set!(12, 2, Control;
           7, 2, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           0, 6, Outline { thick: true, update: true }),
    g2set!(2, 4, Break;
           7, 11, Outline { thick: true, update: true }),
    g2set!(12, 4, Outline { thick: true, update: false };
           7, 24, Outline { thick: true, update: true }),
    g2set!(2, 1, Break;
           0, 7, Outline { thick: true, update: true }),
    g2set!(0, 24, Break;
           4, 24, Outline { thick: false, update: true }),
    g2set!(0, 23, Control;
           0, 14, Outline { thick: true, update: true }),
    g2set!(0, 4, Control;
           6, 4, Outline { thick: true, update: true }),
    g2set!(12, 4, Control;
           12, 14, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(0, 5, Outline { thick: true, update: true };
           6, 9, Break),
    g2set!(6, 27, Control;
           9, 24, Outline { thick: true, update: true }),
    g2set!(4, 6, Break;
           4, 5, Outline { thick: true, update: true }),
    g2set!(5, 2, Break;
           6, 2, Outline { thick: true, update: true }),
    g2set!(8, 6, Break;
           8, 5, Outline { thick: true, update: true }),
    g2set!(0, 24, Break;
           6, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 17, Break),
    g2set!(10, 17, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(13, 25, Control;
           12, 18, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
           0, 12, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
           12, 6, Outline { thick: false, update: true }),
    g2set!(12, 1, Control;
           0, 1, Outline { thick: false, update: true }),
    g2set!(12, 4, Break;
           0, 4, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 1, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: false }),
    g2set!(0, 12, Outline { thick: true, update: true };
           9, 12, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
           12, 12, Outline { thick: false, update: true }),
    g2set!(12, 1, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           2, 12, Break),
    g2set!(10, 12, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(9, 1, Outline { thick: false, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 24, Break),
    g2set!(9, 24, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
           12, 1, Outline { thick: false, update: false }),
    g2set!(12, 24, Outline { thick: false, update: false };
           0, 24, Break),
    g2set!(6, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           6, 14, Outline { thick: false, update: true }),
    g2set!(12, 1, Outline { thick: false, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(3, 10, Break;
           9, 10, Outline { thick: true, update: true }),
    g2set!(0, 24, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: false }),
    g2set!(12, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 1, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: false }),
    g2set!(8, 12, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           6, 24, Outline { thick: false, update: true }),
    g2set!(6, 11, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: false }),
    g2set!(6, 24, Outline { thick: true, update: true };
           6, 24, Break),
    g2set!(6, 1, Outline { thick: true, update: true };
           0, 1, Control),
    g2set!(0, 6, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(6, 14, Outline { thick: true, update: true };
           12, 15, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Control;
           6, 12, Outline { thick: true, update: true }),
    g2set!(12, 12, Control;
           12, 1, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Break;
           4, 24, Outline { thick: false, update: true }),
    g2set!(0, 24, Control;
           0, 12, Outline { thick: true, update: true }),
    g2set!(0, 1, Control;
           6, 1, Outline { thick: true, update: true }),
    g2set!(12, 1, Control;
           12, 12, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
           3, 3, Break),
    g2set!(9, 3, Outline { thick: false, update: true };
           6, 3, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 24, Break),
    g2set!(9, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(3, 0, Outline { thick: true, update: true };
           9, 1, Break),
    g2set!(9, 0, Outline { thick: true, update: true };
           0, 4, Break),
    g2set!(6, 12, Outline { thick: true, update: true };
           12, 4, Outline { thick: true, update: false }),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(3, 0, Outline { thick: true, update: true };
           9, 1, Break),
    g2set!(9, 0, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(9, 26, Control;
           4, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           1, 16, Outline { thick: true, update: true }),
    g2set!(1, 7, Control;
           4, 7, Outline { thick: true, update: true }),
    g2set!(10, 7, Control;
           13, 24, Outline { thick: true, update: true }),
    g2set!(7, 3, Break;
           5, 7, Outline { thick: false, update: true }),
    g2set!(12, 7, Break;
           0, 6, Control),
    g2set!(0, 11, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(9, 15, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(0, 19, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(12, 23, Outline { thick: true, update: true };
           7, 2, Break),
    g2set!(5, 7, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 15, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: false }),
    g2set!(0, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           11, 31, Outline { thick: true, update: true }),
    g2set!(7, 2, Break;
           5, 7, Outline { thick: false, update: true }),
    g2set!(6, 7, Break;
           6, 25, Control),
    g2set!(10, 24, Outline { thick: true, update: true };
           7, 2, Break),
    g2set!(5, 5, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Control;
           4, 24, Outline { thick: true, update: true }),
    g2set!(12, 22, Control;
           12, 7, Outline { thick: true, update: true }),
    g2set!(7, 3, Break;
           5, 7, Outline { thick: false, update: true }),
    g2set!(3, 8, Break;
           3, 7, Outline { thick: true, update: true }),
    g2set!(9, 8, Break;
           9, 7, Outline { thick: true, update: true }),
    g2set!(12, 7, Break;
           9, 26, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(1, 16, Outline { thick: true, update: true };
           1, 7, Control),
    g2set!(4, 7, Outline { thick: true, update: true };
           10, 7, Control),
    g2set!(13, 24, Outline { thick: true, update: true };
           0, 31, Break),
    g2set!(0, 9, Control;
           2, 4, Outline { thick: true, update: true }),
    g2set!(8, 1, Control;
           12, 5, Outline { thick: true, update: true }),
    g2set!(12, 10, Control;
           5, 10, Outline { thick: true, update: true }),
    g2set!(12, 10, Control;
           12, 16, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           1, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           6, 12, Outline { thick: true, update: true }),
    g2set!(10, 6, Outline { thick: true, update: false };
           5, 23, Outline { thick: true, update: true }),
    g2set!(12, 5, Break;
           0, 3, Control),
    g2set!(3, 8, Outline { thick: true, update: true };
           12, 12, Control),
    g2set!(12, 19, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           0, 23, Control),
    g2set!(2, 17, Outline { thick: true, update: true };
           8, 12, Outline { thick: true, update: true }),
    g2set!(12, 7, Break;
           0, 6, Control),
    g2set!(0, 11, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(9, 15, Outline { thick: true, update: true };
           0, 15, Control),
    g2set!(0, 19, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(12, 23, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(6, 9, Outline { thick: true, update: true };
           0, 17, Control),
    g2set!(0, 19, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(8, 24, Outline { thick: true, update: true };
           13, 24, Control),
    g2set!(10, 28, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 15, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: false }),
    g2set!(0, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           11, 31, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           2, 1, Control),
    g2set!(2, 12, Outline { thick: true, update: true };
           2, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           10, 24, Control),
    g2set!(10, 12, Outline { thick: true, update: true };
           10, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           2, 12, Break),
    g2set!(10, 12, Outline { thick: false, update: true };
           6, 7, Break),
    g2set!(6, 25, Control;
           10, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 15, Break;
           12, 7, Outline { thick: false, update: false }),
    g2set!(12, 24, Outline { thick: false, update: false };
           0, 24, Break),
    g2set!(7, 12, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: false }),
    g2set!(6, 4, Outline { thick: true, update: true };
           2, 1, Control),
    g2set!(2, 7, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 16, Outline { thick: true, update: true };
           0, 31, Outline { thick: true, update: false }),
    g2set!(2, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(11, 24, Control;
           12, 8, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(5, 24, Outline { thick: true, update: true };
           14, 14, Control),
    g2set!(11, 7, Outline { thick: true, update: true };
           12, 1, Outline { thick: false, update: true }),
    g2set!(0, 1, Control;
           0, 7, Outline { thick: false, update: true }),
    g2set!(0, 11, Control;
           12, 12, Outline { thick: false, update: true }),
    g2set!(0, 11, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 21, Control;
           8, 21, Outline { thick: true, update: true }),
    g2set!(13, 21, Control;
           11, 25, Outline { thick: false, update: true }),
    g2set!(6, 7, Break;
           0, 7, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 16, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(6, 7, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 7, Outline { thick: true, update: true };
           2, 7, Break),
    g2set!(2, 24, Outline { thick: true, update: true };
           10, 7, Break),
    g2set!(10, 24, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 31, Break;
           0, 17, Control),
    g2set!(3, 11, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(11, 15, Outline { thick: true, update: true };
           10, 23, Control),
    g2set!(1, 21, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           10, 29, Outline { thick: true, update: true }),
    g2set!(12, 7, Break;
           0, 5, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           13, 11, Control),
    g2set!(3, 10, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 7, Outline { thick: true, update: true };
           6, 7, Break),
    g2set!(6, 24, Control;
           10, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           12, 22, Control),
    g2set!(12, 7, Outline { thick: true, update: true };
           3, 7, Break),
    g2set!(0, 7, Control;
           0, 15, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 15, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(6, 31, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 29, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 29, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 7, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           6, 31, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Control),
    g2set!(2, 24, Outline { thick: true, update: true };
           6, 24, Control),
    g2set!(6, 12, Outline { thick: true, update: true };
           6, 24, Control),
    g2set!(9, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 7, Outline { thick: true, update: true };
           6, 7, Break),
    g2set!(6, 25, Control;
           10, 24, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           3, 0, Outline { thick: true, update: true }),
    g2set!(9, 1, Break;
           9, 0, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           12, 22, Control),
    g2set!(12, 7, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(3, 0, Outline { thick: true, update: true };
           9, 1, Break),
    g2set!(9, 0, Outline { thick: true, update: true };
           6, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 16, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(7, 3, Break;
           5, 7, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           12, 22, Control),
    g2set!(12, 7, Outline { thick: true, update: true };
           7, 3, Break),
    g2set!(5, 7, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Control;
           2, 24, Outline { thick: true, update: true }),
    g2set!(6, 24, Control;
           6, 12, Outline { thick: true, update: true }),
    g2set!(6, 24, Control;
           9, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 7, Outline { thick: true, update: true }),
    g2set!(7, 3, Break;
           5, 7, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(12, 1, Outline { thick: false, update: false };
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 28, Outline { thick: true, update: true };
           0, 0, Break),
];

pub const GLYPHS_GREEK : GlyphTable = GlyphTable {
    basechar: 'Ͱ',
    addr: & [0, 3, 7, 12, 18, 20, 22, 25, 29, 29, 29, 31, 36, 43, 50, 54, 58, 58, 58, 58, 58, 60, 66, 73, 75, 83, 91, 99, 99, 110, 110, 116, 129, 138, 143, 152, 155, 159, 164, 167, 172, 183, 189, 193, 196, 200, 203, 208, 217, 220, 225, 225, 229, 232, 235, 245, 248, 254, 265, 275, 283, 294, 305, 314, 319, 330, 339, 350, 354, 364, 373, 381, 388, 399, 402, 407, 413, 421, 425, 436, 445, 452, 459, 466, 475, 480, 485, 495, 499, 506, 515, 522, 531, 542, 549, 560, 565],
    data: GlyphData(&_GDATA_GREEK_),
};

pub const _GDATA_GLYPHS_LATIN_EXTA_UNIMPL : [[u8; 3]; 443] = [
    g2set!(0, 24, Break;
           6, 3, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 17, Break),
    g2set!(10, 17, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(9, 1, Outline { thick: false, update: true };
           0, 8, Break),
    g2set!(1, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           12, 14, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: false };
           0, 14, Control),
    g2set!(0, 19, Outline { thick: true, update: true };
           1, 25, Control),
    g2set!(14, 24, Outline { thick: true, update: true };
           3, 4, Break),
    g2set!(9, 4, Outline { thick: false, update: true };
           0, 24, Break),
    g2set!(6, 3, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(2, 17, Break;
           10, 17, Outline { thick: false, update: true }),
    g2set!(3, 0, Break;
           6, 3, Control),
    g2set!(9, 0, Outline { thick: false, update: true };
           0, 8, Break),
    g2set!(1, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           12, 14, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: false };
           0, 14, Control),
    g2set!(0, 19, Outline { thick: true, update: true };
           1, 25, Control),
    g2set!(14, 24, Outline { thick: true, update: true };
           3, 2, Break),
    g2set!(6, 6, Control;
           9, 2, Outline { thick: false, update: true }),
    g2set!(0, 24, Break;
           6, 3, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 17, Break),
    g2set!(10, 17, Outline { thick: false, update: true };
           12, 24, Break),
    g2set!(6, 30, Control;
           12, 29, Outline { thick: false, update: true }),
    g2set!(0, 8, Break;
           1, 7, Control),
    g2set!(6, 7, Outline { thick: true, update: true };
           12, 7, Control),
    g2set!(12, 14, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: false }),
    g2set!(0, 14, Control;
           0, 19, Outline { thick: true, update: true }),
    g2set!(1, 25, Control;
           14, 24, Outline { thick: true, update: true }),
    g2set!(8, 29, Control;
           14, 29, Outline { thick: false, update: true }),
    g2set!(12, 4, Break;
           0, 4, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           8, 0, Break),
    g2set!(4, 4, Outline { thick: false, update: true };
           12, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(8, 1, Break;
           5, 5, Outline { thick: false, update: true }),
    g2set!(12, 4, Break;
           0, 3, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 3, Break),
    g2set!(6, 0, Outline { thick: false, update: true };
           10, 3, Outline { thick: false, update: true }),
    g2set!(12, 7, Break;
           0, 7, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           2, 5, Break),
    g2set!(6, 2, Outline { thick: true, update: true };
           10, 5, Outline { thick: true, update: true }),
    g2set!(12, 3, Break;
           0, 3, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(7, 1, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 5, Break;
           7, 5, Outline { thick: true, update: true }),
    g2set!(12, 3, Break;
           0, 3, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           3, 0, Break),
    g2set!(6, 4, Control;
           9, 0, Outline { thick: false, update: true }),
    g2set!(12, 7, Break;
           0, 7, Control),
    g2set!(0, 16, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(12, 24, Outline { thick: true, update: true };
           3, 4, Break),
    g2set!(6, 8, Control;
           10, 4, Outline { thick: false, update: true }),
    g2set!(0, 5, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 15, Outline { thick: true, update: true }),
    g2set!(12, 5, Control;
           0, 5, Outline { thick: true, update: true }),
    g2set!(2, 1, Break;
           6, 6, Control),
    g2set!(10, 1, Outline { thick: false, update: true };
           11, 1, Break),
    g2set!(11, 24, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
           0, 10, Control),
    g2set!(12, 11, Outline { thick: false, update: true };
           14, 0, Break),
    g2set!(11, 7, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(0, 1, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(6, 12, Outline { thick: true, update: true };
           12, 1, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
           0, 10, Control),
    g2set!(12, 11, Outline { thick: false, update: true };
           9, 4, Break),
    g2set!(15, 4, Outline { thick: false, update: true };
           0, 3, Break),
    g2set!(12, 3, Outline { thick: true, update: false };
           0, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           3, 1, Break),
    g2set!(9, 1, Outline { thick: true, update: true };
           0, 17, Break),
    g2set!(12, 17, Outline { thick: false, update: true };
           12, 10, Control),
    g2set!(6, 10, Outline { thick: false, update: true };
           0, 10, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: false, update: true };
           11, 24, Outline { thick: false, update: true }),
    g2set!(3, 6, Break;
           9, 6, Outline { thick: true, update: true }),
    g2set!(0, 3, Break;
           12, 3, Outline { thick: true, update: false }),
    g2set!(0, 3, Outline { thick: true, update: true };
           0, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           3, 0, Break),
    g2set!(6, 4, Control;
           9, 0, Outline { thick: false, update: true }),
    g2set!(0, 17, Break;
           12, 17, Outline { thick: false, update: true }),
    g2set!(12, 10, Control;
           6, 10, Outline { thick: false, update: true }),
    g2set!(0, 10, Control;
           0, 17, Outline { thick: false, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: false, update: true }),
    g2set!(11, 24, Outline { thick: false, update: true };
           3, 4, Break),
    g2set!(6, 8, Control;
           10, 4, Outline { thick: false, update: true }),
    g2set!(0, 3, Break;
           12, 3, Outline { thick: true, update: false }),
    g2set!(0, 12, Outline { thick: false, update: true };
           10, 12, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: false, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           7, 1, Outline { thick: true, update: true }),
    g2set!(0, 17, Break;
           12, 17, Outline { thick: false, update: true }),
    g2set!(12, 10, Control;
           6, 10, Outline { thick: false, update: true }),
    g2set!(0, 10, Control;
           0, 17, Outline { thick: false, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: false, update: true }),
    g2set!(11, 24, Outline { thick: false, update: true };
           6, 5, Break),
    g2set!(7, 5, Outline { thick: true, update: true };
           0, 3, Break),
    g2set!(12, 3, Outline { thick: true, update: false };
           0, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 28, Control),
    g2set!(12, 28, Outline { thick: true, update: true };
           0, 17, Break),
    g2set!(12, 17, Outline { thick: false, update: true };
           12, 10, Control),
    g2set!(6, 10, Outline { thick: false, update: true };
           0, 10, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: false, update: true };
           11, 24, Outline { thick: false, update: true }),
    g2set!(6, 29, Control;
           12, 29, Outline { thick: false, update: true }),
    g2set!(0, 3, Break;
           12, 3, Outline { thick: true, update: false }),
    g2set!(0, 12, Outline { thick: true, update: true };
           9, 12, Outline { thick: false, update: false }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(3, 0, Break;
           6, 4, Control),
    g2set!(9, 0, Outline { thick: false, update: true };
           0, 17, Break),
    g2set!(12, 17, Outline { thick: false, update: true };
           12, 10, Control),
    g2set!(6, 10, Outline { thick: false, update: true };
           0, 10, Control),
    g2set!(0, 17, Outline { thick: false, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: false, update: true };
           11, 24, Outline { thick: false, update: true }),
    g2set!(3, 4, Break;
           6, 8, Control),
    g2set!(10, 4, Outline { thick: false, update: true };
           12, 4, Break),
    g2set!(0, 3, Control;
           0, 14, Outline { thick: true, update: true }),
    g2set!(0, 25, Control;
           12, 23, Outline { thick: true, update: true }),
    g2set!(12, 12, Outline { thick: false, update: true };
           7, 12, Outline { thick: false, update: true }),
    g2set!(2, 3, Break;
           6, 0, Outline { thick: false, update: true }),
    g2set!(10, 3, Outline { thick: false, update: true };
           12, 24, Break),
    g2set!(0, 24, Control;
           0, 17, Outline { thick: true, update: true }),
    g2set!(0, 10, Control;
           12, 10, Outline { thick: true, update: true }),
    g2set!(12, 25, Outline { thick: true, update: true };
           12, 30, Control),
    g2set!(1, 28, Outline { thick: true, update: true };
           2, 7, Break),
    g2set!(6, 4, Outline { thick: false, update: true };
           10, 7, Outline { thick: false, update: true }),
    g2set!(12, 4, Break;
           0, 4, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 23, Outline { thick: true, update: true };
           12, 12, Outline { thick: false, update: true }),
    g2set!(7, 12, Outline { thick: false, update: true };
           3, 0, Break),
    g2set!(6, 4, Control;
           9, 0, Outline { thick: false, update: true }),
    g2set!(12, 24, Break;
           0, 24, Control),
    g2set!(0, 17, Outline { thick: true, update: true };
           0, 10, Control),
    g2set!(12, 10, Outline { thick: true, update: true };
           12, 25, Outline { thick: true, update: true }),
    g2set!(12, 30, Control;
           1, 28, Outline { thick: true, update: true }),
    g2set!(3, 5, Break;
           6, 8, Control),
    g2set!(9, 5, Outline { thick: false, update: true };
           12, 2, Break),
    g2set!(0, 1, Control;
           0, 12, Outline { thick: true, update: true }),
    g2set!(0, 25, Control;
           12, 23, Outline { thick: true, update: true }),
    g2set!(12, 12, Outline { thick: false, update: true };
           7, 12, Outline { thick: false, update: true }),
    g2set!(6, 1, Break;
           7, 1, Outline { thick: true, update: true }),
    g2set!(12, 24, Break;
           0, 24, Control),
    g2set!(0, 17, Outline { thick: true, update: true };
           0, 10, Control),
    g2set!(12, 10, Outline { thick: true, update: true };
           12, 25, Outline { thick: true, update: true }),
    g2set!(12, 30, Control;
           1, 28, Outline { thick: true, update: true }),
    g2set!(6, 5, Break;
           7, 5, Outline { thick: true, update: true }),
    g2set!(12, 2, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 25, Control),
    g2set!(12, 23, Outline { thick: true, update: true };
           12, 12, Outline { thick: false, update: true }),
    g2set!(7, 12, Outline { thick: false, update: true };
           7, 25, Break),
    g2set!(6, 27, Outline { thick: false, update: true };
           12, 24, Break),
    g2set!(0, 24, Control;
           0, 17, Outline { thick: true, update: true }),
    g2set!(0, 10, Control;
           12, 10, Outline { thick: true, update: true }),
    g2set!(12, 25, Outline { thick: true, update: true };
           12, 30, Control),
    g2set!(1, 28, Outline { thick: true, update: true };
           7, 4, Break),
    g2set!(5, 7, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 12, Break;
           12, 12, Outline { thick: false, update: true }),
    g2set!(12, 1, Break;
           12, 24, Outline { thick: true, update: true }),
    g2set!(3, 3, Break;
           6, 0, Outline { thick: false, update: true }),
    g2set!(9, 3, Outline { thick: false, update: true };
           2, 4, Break),
    g2set!(2, 24, Outline { thick: true, update: true };
           2, 12, Break),
    g2set!(14, 6, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 4, Break;
           2, 1, Outline { thick: false, update: true }),
    g2set!(5, 4, Outline { thick: false, update: true };
           2, 1, Break),
    g2set!(2, 24, Outline { thick: true, update: true };
           2, 12, Break),
    g2set!(11, 12, Outline { thick: false, update: true };
           11, 1, Break),
    g2set!(11, 24, Outline { thick: true, update: true };
           0, 5, Break),
    g2set!(14, 5, Outline { thick: false, update: true };
           2, 1, Break),
    g2set!(2, 24, Outline { thick: true, update: true };
           2, 12, Break),
    g2set!(14, 6, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 4, Break;
           4, 4, Outline { thick: false, update: true }),
    g2set!(3, 4, Break;
           9, 4, Outline { thick: false, update: true }),
    g2set!(6, 4, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 24, Break;
           9, 24, Outline { thick: false, update: true }),
    g2set!(2, 2, Break;
           4, 0, Outline { thick: false, update: true }),
    g2set!(6, 3, Outline { thick: false, update: true };
           9, 1, Outline { thick: false, update: true }),
    g2set!(6, 6, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 3, Break;
           5, 1, Outline { thick: false, update: true }),
    g2set!(7, 4, Outline { thick: false, update: true };
           9, 2, Outline { thick: false, update: true }),
    g2set!(3, 4, Break;
           9, 4, Outline { thick: false, update: true }),
    g2set!(6, 4, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 24, Break;
           9, 24, Outline { thick: false, update: true }),
    g2set!(2, 2, Break;
           10, 2, Outline { thick: false, update: true }),
    g2set!(6, 6, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 3, Break;
           9, 3, Outline { thick: false, update: true }),
    g2set!(3, 4, Break;
           9, 4, Outline { thick: false, update: true }),
    g2set!(6, 4, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 24, Break;
           9, 24, Outline { thick: false, update: true }),
    g2set!(2, 1, Break;
           6, 3, Outline { thick: false, update: true }),
    g2set!(10, 1, Outline { thick: false, update: true };
           6, 6, Break),
    g2set!(6, 24, Outline { thick: true, update: true };
           3, 3, Break),
    g2set!(6, 5, Outline { thick: false, update: true };
           9, 3, Outline { thick: false, update: true }),
    g2set!(3, 1, Break;
           9, 1, Outline { thick: false, update: true }),
    g2set!(6, 1, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(5, 29, Control;
           9, 29, Outline { thick: false, update: true }),
    g2set!(3, 24, Break;
           9, 24, Outline { thick: false, update: true }),
    g2set!(6, 4, Break;
           6, 5, Outline { thick: true, update: true }),
    g2set!(6, 11, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(4, 29, Control;
           9, 27, Outline { thick: false, update: true }),
    g2set!(3, 4, Break;
           9, 4, Outline { thick: false, update: true }),
    g2set!(6, 4, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(3, 24, Break;
           9, 24, Outline { thick: false, update: true }),
    g2set!(6, 2, Break;
           7, 2, Outline { thick: true, update: true }),
    g2set!(6, 7, Break;
           6, 24, Outline { thick: true, update: true }),
    g2set!(2, 1, Break;
           8, 1, Outline { thick: false, update: true }),
    g2set!(5, 1, Break;
           5, 24, Outline { thick: true, update: true }),
    g2set!(2, 24, Break;
           8, 24, Outline { thick: false, update: true }),
    g2set!(10, 1, Break;
           10, 20, Outline { thick: true, update: true }),
    g2set!(8, 27, Control;
           3, 23, Outline { thick: true, update: true }),
    g2set!(3, 4, Break;
           3, 5, Outline { thick: true, update: true }),
    g2set!(3, 11, Break;
           3, 24, Outline { thick: true, update: true }),
    g2set!(6, 4, Break;
           6, 5, Outline { thick: true, update: true }),
    g2set!(6, 11, Break;
           6, 25, Outline { thick: true, update: true }),
    g2set!(6, 29, Control;
           3, 29, Outline { thick: true, update: true }),
    g2set!(10, 5, Break;
           10, 20, Outline { thick: true, update: true }),
    g2set!(8, 27, Control;
           3, 23, Outline { thick: true, update: true }),
    g2set!(7, 3, Break;
           10, 1, Outline { thick: false, update: true }),
    g2set!(14, 3, Outline { thick: false, update: true };
           6, 11, Break),
    g2set!(6, 25, Outline { thick: true, update: true };
           6, 29, Control),
    g2set!(3, 29, Outline { thick: true, update: true };
           3, 10, Break),
    g2set!(6, 7, Outline { thick: false, update: true };
           9, 10, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 12, Break),
    g2set!(12, 1, Outline { thick: false, update: false };
           12, 24, Outline { thick: false, update: false }),
    g2set!(7, 24, Break;
           4, 28, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 15, Break),
    g2set!(12, 24, Outline { thick: true, update: false };
           12, 8, Outline { thick: false, update: true }),
    g2set!(7, 24, Break;
           4, 28, Outline { thick: false, update: true }),
    g2set!(0, 8, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 15, Break;
           12, 24, Outline { thick: true, update: false }),
    g2set!(12, 8, Outline { thick: false, update: true };
           0, 8, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: false, update: true }),
    g2set!(3, 0, Break;
           0, 4, Outline { thick: false, update: true }),
    g2set!(4, 5, Break;
           7, 5, Outline { thick: false, update: true }),
    g2set!(7, 24, Outline { thick: false, update: true };
           7, 0, Break),
    g2set!(5, 2, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
           7, 26, Break),
    g2set!(5, 29, Outline { thick: false, update: true };
           4, 1, Break),
    g2set!(7, 1, Outline { thick: false, update: true };
           7, 24, Outline { thick: false, update: true }),
    g2set!(9, 25, Break;
           6, 29, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: false, update: true }),
    g2set!(4, 2, Break;
           3, 5, Outline { thick: false, update: true }),
    g2set!(4, 1, Break;
           7, 1, Outline { thick: false, update: true }),
    g2set!(7, 24, Outline { thick: false, update: true };
           11, 1, Break),
    g2set!(10, 5, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: false, update: true };
           4, 11, Break),
    g2set!(4, 12, Outline { thick: true, update: true };
           4, 1, Break),
    g2set!(7, 1, Outline { thick: false, update: true };
           7, 24, Outline { thick: false, update: true }),
    g2set!(10, 12, Break;
           10, 13, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           3, 24, Outline { thick: false, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           5, 11, Break),
    g2set!(1, 13, Outline { thick: false, update: true };
           4, 1, Break),
    g2set!(7, 1, Outline { thick: false, update: true };
           7, 24, Outline { thick: false, update: true }),
    g2set!(10, 8, Break;
           5, 13, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           8, 0, Break),
    g2set!(6, 4, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 10, Break),
    g2set!(14, 2, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(8, 2, Break;
           5, 6, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           8, 23, Break),
    g2set!(5, 27, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 10, Break),
    g2set!(14, 2, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(7, 22, Break;
           5, 28, Outline { thick: false, update: true }),
    g2set!(0, 5, Break;
           0, 24, Outline { thick: true, update: false }),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 5, Outline { thick: true, update: true }),
    g2set!(3, 1, Break;
           6, 4, Outline { thick: false, update: true }),
    g2set!(9, 1, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 10, Break),
    g2set!(14, 2, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(3, 4, Break;
           6, 6, Outline { thick: false, update: true }),
    g2set!(9, 4, Outline { thick: false, update: true };
           2, 7, Break),
    g2set!(2, 24, Outline { thick: true, update: true };
           2, 10, Break),
    g2set!(14, 2, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(2, 2, Break;
           0, 6, Outline { thick: false, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 1, Outline { thick: true, update: false };
           13, 28, Control),
    g2set!(5, 29, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(0, 24, Outline { thick: true, update: true };
           0, 10, Break),
    g2set!(14, 2, Control;
           12, 24, Outline { thick: true, update: true }),
    g2set!(13, 31, Control;
           6, 30, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           2, 2, Break),
    g2set!(10, 2, Outline { thick: false, update: true };
           6, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 16, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(2, 2, Break;
           10, 2, Outline { thick: false, update: true }),
    g2set!(6, 1, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           6, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 16, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(6, 1, Break;
           0, 1, Control),
    g2set!(0, 12, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(6, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 12, Outline { thick: true, update: true };
           12, 1, Control),
    g2set!(6, 1, Outline { thick: true, update: true };
           6, 7, Break),
    g2set!(0, 7, Control;
           0, 16, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 16, Outline { thick: true, update: true }),
    g2set!(12, 7, Control;
           6, 7, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 1, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           12, 24, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 6, Control;
           12, 8, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 1, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           12, 24, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 6, Control;
           12, 8, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: false };
           12, 1, Control),
    g2set!(12, 6, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(0, 14, Outline { thick: true, update: true };
           12, 24, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 24, Outline { thick: true, update: true }),
    g2set!(0, 6, Control;
           12, 8, Outline { thick: true, update: true }),
    g2set!(12, 4, Break;
           0, 3, Control),
    g2set!(0, 8, Outline { thick: false, update: true };
           0, 12, Control),
    g2set!(6, 13, Outline { thick: true, update: true };
           12, 14, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           8, 0, Break),
    g2set!(6, 3, Outline { thick: false, update: true };
           12, 7, Break),
    g2set!(0, 6, Control;
           1, 12, Outline { thick: true, update: true }),
    g2set!(3, 14, Control;
           6, 15, Outline { thick: true, update: true }),
    g2set!(10, 16, Control;
           12, 19, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
           0, 24, Outline { thick: true, update: true }),
    g2set!(9, 2, Break;
           6, 5, Outline { thick: false, update: true }),
    g2set!(12, 2, Break;
           0, 0, Control),
    g2set!(0, 7, Outline { thick: false, update: true };
           0, 11, Control),
    g2set!(6, 12, Outline { thick: true, update: true };
           12, 13, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 6, Control;
           1, 12, Outline { thick: true, update: true }),
    g2set!(3, 14, Control;
           6, 15, Outline { thick: true, update: true }),
    g2set!(10, 16, Control;
           12, 19, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 2, Break;
           0, 0, Control),
    g2set!(0, 7, Outline { thick: false, update: true };
           0, 11, Control),
    g2set!(6, 12, Outline { thick: true, update: true };
           12, 13, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 6, Control;
           1, 12, Outline { thick: true, update: true }),
    g2set!(3, 14, Control;
           6, 15, Outline { thick: true, update: true }),
    g2set!(10, 16, Control;
           12, 19, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 2, Break;
           0, 0, Control),
    g2set!(0, 7, Outline { thick: false, update: true };
           0, 11, Control),
    g2set!(6, 12, Outline { thick: true, update: true };
           12, 13, Control),
    g2set!(12, 18, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(0, 23, Outline { thick: true, update: true };
           12, 7, Break),
    g2set!(0, 6, Control;
           1, 12, Outline { thick: true, update: true }),
    g2set!(3, 14, Control;
           6, 15, Outline { thick: true, update: true }),
    g2set!(10, 16, Control;
           12, 19, Outline { thick: true, update: true }),
    g2set!(12, 25, Control;
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(3, 7, Outline { thick: true, update: true };
           9, 7, Outline { thick: false, update: false }),
    g2set!(2, 25, Control;
           9, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(3, 7, Outline { thick: true, update: true };
           9, 7, Outline { thick: false, update: false }),
    g2set!(2, 25, Control;
           9, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           6, 1, Break),
    g2set!(6, 24, Outline { thick: false, update: true };
           3, 1, Break),
    g2set!(3, 7, Outline { thick: true, update: true };
           9, 7, Outline { thick: false, update: false }),
    g2set!(2, 25, Control;
           9, 24, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           4, 24, Outline { thick: true, update: true }),
    g2set!(8, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 1, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 28, Control;
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 1, Outline { thick: true, update: true }),
    g2set!(2, 2, Break;
           10, 2, Outline { thick: false, update: true }),
    g2set!(0, 7, Break;
           0, 28, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(2, 2, Break;
           10, 2, Outline { thick: false, update: true }),
    g2set!(0, 24, Control;
           4, 24, Outline { thick: true, update: true }),
    g2set!(8, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 1, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 28, Control;
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 1, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 28, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(0, 24, Control;
           4, 24, Outline { thick: true, update: true }),
    g2set!(8, 24, Outline { thick: true, update: true };
           12, 24, Control),
    g2set!(12, 1, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(0, 28, Control;
           12, 24, Outline { thick: false, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           0, 24, Control),
    g2set!(4, 24, Outline { thick: true, update: true };
           8, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Control;
           12, 1, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 28, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Outline { thick: true, update: true }),
    g2set!(1, 24, Outline { thick: true, update: true };
           6, 1, Outline { thick: true, update: true }),
    g2set!(11, 24, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           2, 24, Outline { thick: true, update: true }),
    g2set!(6, 7, Outline { thick: true, update: true };
           10, 24, Outline { thick: true, update: true }),
    g2set!(12, 7, Outline { thick: true, update: true };
           6, 11, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: false };
           6, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           0, 26, Control),
    g2set!(12, 24, Outline { thick: false, update: true };
           12, 7, Break),
    g2set!(12, 24, Outline { thick: true, update: true };
           12, 31, Control),
    g2set!(1, 29, Outline { thick: true, update: true };
           6, 11, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: false };
           6, 24, Outline { thick: true, update: true }),
    g2set!(12, 1, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 7, Break),
    g2set!(12, 7, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           0, 3, Break),
    g2set!(12, 3, Outline { thick: true, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 0, Break),
    g2set!(7, 0, Outline { thick: false, update: true };
           0, 7, Break),
    g2set!(12, 7, Outline { thick: false, update: true };
           0, 24, Outline { thick: true, update: true }),
    g2set!(12, 24, Outline { thick: true, update: true };
           6, 5, Break),
    g2set!(7, 5, Outline { thick: true, update: true };
           12, 1, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(0, 7, Break;
           12, 7, Outline { thick: true, update: true }),
    g2set!(0, 24, Outline { thick: true, update: true };
           12, 24, Outline { thick: true, update: true }),
    g2set!(3, 24, Break;
           3, 12, Outline { thick: true, update: true }),
    g2set!(9, 12, Outline { thick: false, update: false };
           3, 3, Control),
    g2set!(10, 4, Outline { thick: true, update: true };
           0, 0, Break),
];

pub const GLYPHS_LATIN_EXTA_UNIMPL : GlyphTable = GlyphTable {
    basechar: 'Ā',
    addr: & [0, 7, 19, 27, 40, 48, 60, 67, 74, 82, 90, 97, 104, 112, 120, 129, 137, 145, 153, 161, 172, 182, 194, 202, 213, 221, 232, 241, 253, 263, 274, 284, 295, 304, 314, 323, 333, 341, 349, 357, 364, 374, 380, 388, 392, 401, 406, 414, 420, 428, 430, 440, 450, 457, 464, 470, 476, 481, 486, 491, 495, 500, 504, 509, 513, 518, 523, 528, 533, 540, 545, 552, 559, 567, 574, 579, 586, 597, 608, 617, 626, 635, 644, 644, 644, 650, 654, 660, 664, 670, 674, 685, 696, 705, 714, 723, 732, 741, 750, 753, 758, 761, 766, 769, 774, 779, 783, 790, 796, 801, 805, 810, 814, 819, 823, 828, 832, 836, 841, 844, 851, 854, 857, 861, 867, 873, 876, 880, 885],
    data: GlyphData(&_GDATA_GLYPHS_LATIN_EXTA_UNIMPL),
};
