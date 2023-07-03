// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use crate::{menu::DropDownType, interpolate_colour, escape_time};
use std::collections::HashSet;

/// the number of colours per percentage of a palette length
/// for the repeated mapping type
const PALETTE_DEPTH: usize = 5;
/// the minimum distance between percentages for a new point 
/// to be able to be added
const MIN_ADD_PERCENT: f32 = 10.;

#[derive(Clone, PartialEq)]
pub enum MappingType {
    /// the palette stays the same regardless of the max iterations
    Constant,
    /// the palette length stays the same, being extended further with a higher max iterations
    Repeated
}
impl DropDownType<MappingType> for MappingType {
    fn get_variants() -> Vec<MappingType> {
        vec![MappingType::Constant, MappingType::Repeated]
    }

    fn get_string(&self) -> String {
        String::from(match self {
            MappingType::Constant => "Constant",
            MappingType::Repeated => "Repeated"
        })
    }
}

// this has to be 0 and 100 not 0.0 to 1.0 as when sorting the percentage
// needs to be used as a key, which only supports integers
/// a colour and position (between 0 and 100) in a colour map
#[derive(Clone, Copy)]
pub struct ColourPoint {
    pub colour: Color,
    pub percent: f32
}
impl ColourPoint {
    fn prev_percent(&self) -> ColourPoint {
        ColourPoint { colour: self.colour, percent: self.percent - 100. }
    }

    fn next_percent(&self) -> ColourPoint {
        ColourPoint { colour: self.colour, percent: self.percent + 100. }
    }

    /// `clamp()`s the percentage between 0 and 100
    fn valid_percent(&self) -> f32 {
        self.percent.clamp(0., 100.)
    }
}
impl Into<ColourPoint> for (Color, f32) {
    fn into(self) -> ColourPoint {
        ColourPoint { colour: self.0, percent: self.1 }
    }
}

#[derive(Clone)]
pub struct Palette {
    pub colour_map: Vec<ColourPoint>, // this is only used to edit the colour map
    sorted_colour_map: Vec<ColourPoint>,
    pub mapping_type: MappingType,
    /// the percentage of the palette taken up by 1 repetition (from 0 to 100)
    palette_length: f32,
    /// the percentage of the palette length offset between 0 and 100
    offset: f32,
    /// store the previously generated palette 
    pub palette_cache: Vec<Color>
}
impl Palette {
    /// creates a new colour map from the given colour points
    pub fn new(colour_map: Vec<ColourPoint>, mapping_type: MappingType, palette_length: f32, offset: f32) -> Palette {
        assert!(colour_map.len() > 1);
        assert!(0.0 <= palette_length && palette_length <= 100.0);
        assert!(Palette::unique_point_positions(&colour_map));

        Palette { 
            colour_map: colour_map.clone(), sorted_colour_map: Palette::sort_colour_map(&colour_map),
            mapping_type, palette_length, offset, palette_cache: Vec::new() 
        }
    }

    fn sort_colour_map(colour_map: &Vec<ColourPoint>) -> Vec<ColourPoint> {
        let mut sorted = colour_map.clone();
        sorted.sort_by_key(|p| p.percent as u32);

        // add the two extremes to both sides so they link together
        let unique_sorted = sorted.clone();
        sorted.insert(0, unique_sorted[unique_sorted.len()-1].prev_percent());
        sorted.push(unique_sorted[0].next_percent());

        sorted
    }

    fn unique_point_positions(colour_map: &Vec<ColourPoint>) -> bool {
        let mut unique = HashSet::new();
        colour_map.into_iter().all(move |c| unique.insert(c.percent as u32))
    }
 
    /// creates a new colour map with evenly spaced colour points
    pub fn new_even(colours: Vec<Color>, mapping_type: MappingType, palette_length: f32, offset: f32) -> Palette {
        assert!(colours.len() > 1);
        assert!(0.0 <= palette_length && palette_length <= 100.0);

        let pos = 100. / (colours.len()-1) as f32;
        let colour_map: Vec<ColourPoint> = colours.iter().enumerate()
            .map(|(i, c)| (*c, i as f32 * pos).into()).collect();

        Palette { 
            colour_map: colour_map.clone(), sorted_colour_map: Palette::sort_colour_map(&colour_map),
            mapping_type, palette_length, offset, palette_cache: Vec::new() 
        }
    }

    pub fn default() -> Palette {
        Palette {
            colour_map: vec![(BLACK, 0.0).into(), (WHITE, 100.0).into()],
            sorted_colour_map: vec![(BLACK, 0.0).into(), (WHITE, 100.0).into()],
            mapping_type: MappingType::Repeated,
            palette_length: 100.0,
            offset: 0.0,
            palette_cache: Vec::new()
        }
    }

    pub fn get_palette_length(&self) -> f32 {
        self.palette_length
    }

    pub fn set_palette_length(&mut self, new: f32) -> bool {
        assert!(0.0 <= new && new <= 100.);
        if self.palette_length == new { return false }
        self.palette_length = new;
        true
    }

    pub fn get_offset(&self) -> f32 {
        self.offset
    }

    pub fn set_offset(&mut self, new: f32) -> bool {
        assert!(0.0 <= new && new <= 100.);
        if self.offset == new { return false }
        self.offset = new;
        true
    }

    /// add/delete from the palette length by change
    pub fn change_palette_length(&mut self, change: f32) {
        self.palette_length += change;
        self.palette_length = self.palette_length.clamp(0., 100.);
    }

    /// attempts to change a point's percentage
    /// 
    /// # Returns
    /// if it was successeful or not
    pub fn change_point_percent(&mut self, new_percent: f32, point_i: usize) -> bool {
        for (i, point) in self.colour_map.iter().enumerate() {
            if i == point_i { continue }
            if point.percent == new_percent { return false }
        }
        self.colour_map[point_i].percent = new_percent;
        self.sorted_colour_map = Palette::sort_colour_map(&self.colour_map);
        true
    }

    /// checks if a new point can be added
    /// 
    /// # Returns
    /// Some(percentage) of the new point that should be added
    /// None if a new point can't be added
    pub fn get_add_point_percent(&self) -> Option<f32> {
        // maximum differences between percentages
        let mut max_percent_diff = 0.0;
        // the percentage to add the new number at
        let mut percent_to_add = 0.0;
        for i in 0..self.sorted_colour_map.len()-1 {
            let this_diff = self.sorted_colour_map[i+1].valid_percent() - self.sorted_colour_map[i].valid_percent();
            let this_add = self.sorted_colour_map[i].valid_percent() + this_diff/2.;
            if this_diff < max_percent_diff { continue }

            if this_diff > max_percent_diff {
                max_percent_diff = this_diff;
                percent_to_add = this_add;
            }
            // prioritises percentages closer to the midpoint
            if this_diff == max_percent_diff && (this_add-50.).abs() < (percent_to_add-50.).abs() {
                percent_to_add = this_add;
            }
        }

        match max_percent_diff > MIN_ADD_PERCENT {
            true => Some(percent_to_add),
            false => None
        }
    }

    pub fn add_point(&mut self) {
        // get colour at midpoint
        let percent = match self.get_add_point_percent(){
            None => return,
            Some(p) => p
        };
        let new_colour = self.get_colour_at_percentage(percent/100., false);

        self.colour_map.push((new_colour, percent).into());
        self.sorted_colour_map = Palette::sort_colour_map(&self.colour_map);
    }

    pub fn can_delete_point(&self) -> bool {
        self.colour_map.len() > 2
    }

    pub fn delete_point(&mut self, index: usize) {
        if !self.can_delete_point() { return }
        self.colour_map.remove(index);
        self.sorted_colour_map = Palette::sort_colour_map(&self.colour_map);
    }

    pub fn update_colour(&mut self, index: usize, r: Option<f32>, g: Option<f32>, b: Option<f32>, a: Option<f32>) {
        self.colour_map[index].colour.r = r.unwrap_or(self.colour_map[index].colour.r);
        self.colour_map[index].colour.g = g.unwrap_or(self.colour_map[index].colour.g);
        self.colour_map[index].colour.b = b.unwrap_or(self.colour_map[index].colour.b);
        self.colour_map[index].colour.a = a.unwrap_or(self.colour_map[index].colour.a);

        self.sorted_colour_map = Palette::sort_colour_map(&self.colour_map);
    }

    /// # Params
    /// 
    /// percent given as a float between 0.0 and 1.0
    pub fn get_colour_at_percentage(&self, mut percent: f32, offset: bool) -> Color {
        assert!(0.0 <= percent && percent <= 1.0);

        // apply offset
        if offset {
            percent += self.offset/100.;
            percent = percent % 1.;
        }

        let mut next_i = self.sorted_colour_map.len()-1;
        for i in 0..self.sorted_colour_map.len() {
            if self.sorted_colour_map[i].percent/100. > percent {
                next_i = i;
                break;
            }
        }

        let (prev, next) = (self.sorted_colour_map[next_i-1], self.sorted_colour_map[next_i]);
        interpolate_colour(
            prev.colour, 
            next.colour, 
            (percent - prev.percent/100.) / (next.percent/100. - prev.percent/100.)
        )
    }

    fn get_constant_palette(&self, max_iterations: usize) -> Vec<Color> {
        let mut palette = Vec::with_capacity(max_iterations+1);

        for i in 0..=max_iterations {
            let total_percent = i as f32 / max_iterations as f32;
            // takes the total percent and converts it to a fraction of the palette length
            let mut length_percent = ((total_percent * 100.) % self.palette_length) / self.palette_length;
            if self.palette_length == 0.0 {length_percent = 0.}
            palette.push(self.get_colour_at_percentage(length_percent, true));
        }

        palette
    }

    fn get_repeated_palette(&self, max_iterations: usize) -> Vec<Color> {
        let mut palette = Vec::with_capacity(max_iterations+1);

        let colours_per_i = self.palette_length * PALETTE_DEPTH as f32;

        for i in 0..=max_iterations {
            let mut percent = (i as f32 % colours_per_i) / colours_per_i;
            if self.palette_length == 0.0 {percent = 0.}
            palette.push(self.get_colour_at_percentage(percent, true));
        }

        palette
    }

    /// returns the palette of length max iterations
    pub fn generate_palette(&mut self, max_iterations: f32) {
        self.palette_cache = match self.mapping_type {
            MappingType::Constant => self.get_constant_palette(max_iterations as usize),
            MappingType::Repeated => self.get_repeated_palette(max_iterations as usize)
        };
    }

    /// returns the full gradient as a texture of the required size
    pub fn get_full_gradient(&self, width: f32, height: f32) -> Texture2D {
        let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);

        for i in 0..width as u32 {
            for j in 0..height as u32 {
                let colour = self.get_colour_at_percentage(i as f32 / (width - 1.), false);
                image.set_pixel(i, j, colour);
            }
        }

        Texture2D::from_image(&image)
    }   

    pub fn get_full_palette(&self, width: f32, height: f32, max_iterations: f32) -> Texture2D {
        let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);

        for i in 0..width as u32 { 
            let iteration = max_iterations * (i as f32 / (width - 1.));
            for j in 0..height as u32 {
                let colour = escape_time(iteration as f64, &self.palette_cache);
                image.set_pixel(i, j, colour);
            }
        }

        Texture2D::from_image(&image)
    }
}

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