use crate::prelude::*;

// these define the pens and brushes to be used for the various
// blocks.  each block type is a different color
//

pub const PEN: [Color; 8] = [
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.25,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
];

pub const BRUSH: [Color; 8] = [
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.25,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 0.5,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 0.5,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    },
    Color {
        r: 0.5,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    },
    Color {
        r: 0.0,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    },
    Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    },
];
