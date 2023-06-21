// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

#[allow(unused)]

//pub const COLOUR_MAP: [Color; 3] = [
//     DARKBLUE,
//     PINK,
//     WHITE
// ];
pub const RAINBOW: [Color; 8] = [
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    BLUE,
    PURPLE,
    PINK,
    RED
];
pub const ICE: [Color; 11] = [
    WHITE,
    WHITE,
    WHITE,
    WHITE,
    WHITE,
    Color { r: 0.0, g: 0.686, b: 1.0, a: 1.0},
    DARKBLUE,
    DARKBLUE,
    DARKBLUE,
    Color { r: 0.0, g: 0.686, b: 1.0, a: 1.0},
    WHITE,
];
pub const WPB: [Color; 5] = [
    WHITE,
    PINK,
    Color { r: 0.0, g: 0.686, b: 1.0, a: 1.0},
    PINK,
    WHITE
];
pub const MAIN: [Color; 10] = [
    WHITE,
    DARKBLUE,
    BLUE,
    DARKBROWN,
    BROWN,
    ORANGE,
    YELLOW,
    RED,
    PINK,
    WHITE
];
pub const EARTHANDSKY: [Color; 6] = [
    WHITE,
    YELLOW,
    RED,
    DARKBLUE,
    BLUE,
    WHITE
];
pub const MIDNIGHT: [Color; 5] = [
    WHITE,
    BLUE,
    BLACK,
    PINK,
    WHITE
];
pub const CHERRY: [Color; 8] = [
    WHITE,
    WHITE,
    WHITE,
    WHITE,
    PINK,
    RED,
    PINK,
    WHITE,
];
pub const CHAMPAGNE: [Color; 5] = [
    Color {r: 9./255., g: 43./255., b: 56./255., a: 1.0},
    Color {r: 156./255., g: 87./255., b: 115./255., a:1.0},
    WHITE,
    Color {r: 156./255., g: 87./255., b: 115./255., a:1.0},
    Color {r: 9./255., g: 43./255., b: 56./255., a: 1.0},
];
pub const CHERRY2: [Color; 13] = [
    WHITE,
    WHITE,
    WHITE,
    WHITE,
    WHITE,
    PINK,
    Color {r: 156./255., g: 87./255., b: 115./255., a:1.0},
    Color {r: 156./255., g: 87./255., b: 115./255., a:1.0},
    Color {r: 9./255., g: 43./255., b: 56./255., a: 1.0},
    Color {r: 156./255., g: 87./255., b: 115./255., a:1.0},
    Color {r: 156./255., g: 87./255., b: 115./255., a:1.0},
    PINK,
    WHITE,
];