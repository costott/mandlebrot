// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use crate::{menu::DropDownType, interpolate_colour, escape_time, get_str_between, lerp};
use std::collections::HashSet;

/// the number of iteration values in the palette
/// for a 100% palette length for the repeated mapping type 
/// i.e. 500 => 500 iterations = 100% palette length
/// in other words, what the palette length represents a fraction of
const PALETTE_DEPTH: usize = 500; // make this different to start max iter on release
/// the minimum distance between percentages for a new point 
/// to be able to be added
const MIN_ADD_PERCENT: f32 = 0.1;

#[derive(Clone, PartialEq)]
pub enum MappingType {
    /// the palette stays the same regardless of the max iterations
    Constant,
    /// the palette length stays the same, being extended further with a higher max iterations
    Repeated
}
impl MappingType {
    fn export_num(&self) -> &str {
        match self {
            MappingType::Constant => "0",
            MappingType::Repeated => "1"
        }
    }

    fn import_from_num(num: char) -> MappingType {
        match num {
            '0' => MappingType::Constant,
            '1' => MappingType::Repeated,
            c => panic!("no mapping type for {c}")
        }
    }
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

/// a colour and position (between 0 and 100) in a colour map
#[derive(Clone, Copy)]
pub struct ColourPoint {
    pub colour: Color,
    pub percent: f32
}
impl ColourPoint {
    fn prev_percent(&self) -> ColourPoint {
        ColourPoint { colour: self.colour, percent: self.percent - 1. }
    }

    fn next_percent(&self) -> ColourPoint {
        ColourPoint { colour: self.colour, percent: self.percent + 1. }
    }

    /// `clamp()`s the percentage between 0 and 1
    fn valid_percent(&self) -> f32 {
        self.percent.clamp(0., 1.)
    }

    fn interpolate_points(point1: &ColourPoint, point2: &ColourPoint, percent: f32) -> ColourPoint {
        ColourPoint {
            colour: interpolate_colour(point1.colour, point2.colour, percent),
            percent: lerp(point1.percent, point2.percent, percent)
        } 
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
        assert!(0.0 <= palette_length && palette_length <= 1.0);
        assert!(Palette::unique_point_positions(&colour_map));

        Palette { 
            colour_map: colour_map.clone(), sorted_colour_map: Palette::sort_colour_map(&colour_map),
            mapping_type, palette_length, offset, palette_cache: Vec::new() 
        }
    }

    fn sort_colour_map(colour_map: &Vec<ColourPoint>) -> Vec<ColourPoint> {
        let mut sorted = colour_map.clone();
        sorted.sort_by_key(|p| (p.percent*100.) as u32);

        // add the two extremes to both sides so they link together
        let unique_sorted = sorted.clone();
        sorted.insert(0, unique_sorted[unique_sorted.len()-1].prev_percent());
        sorted.push(unique_sorted[0].next_percent());

        sorted
    }

    fn unique_point_positions(colour_map: &Vec<ColourPoint>) -> bool {
        let mut unique = HashSet::new();
        colour_map.into_iter().all(move |c| unique.insert((c.percent*100.) as u32))
    }
 
    /// creates a new colour map with evenly spaced colour points
    pub fn new_even(colours: Vec<Color>, mapping_type: MappingType, palette_length: f32, offset: f32) -> Palette {
        assert!(colours.len() > 1);
        assert!(0.0 <= palette_length && palette_length <= 1.0);

        let pos = 1. / (colours.len()-1) as f32;
        let colour_map: Vec<ColourPoint> = colours.iter().enumerate()
            .map(|(i, c)| (*c, i as f32 * pos).into()).collect();

        Palette { 
            colour_map: colour_map.clone(), sorted_colour_map: Palette::sort_colour_map(&colour_map),
            mapping_type, palette_length, offset, palette_cache: Vec::new() 
        }
    }

    pub fn default() -> Palette {
        Palette {
            colour_map: vec![(BLACK, 0.0).into(), (WHITE, 1.0).into()],
            sorted_colour_map: vec![(BLACK, 0.0).into(), (WHITE, 1.0).into()],
            mapping_type: MappingType::Repeated,
            palette_length: 1.0,
            offset: 0.0,
            palette_cache: Vec::new()
        }
    }

    pub fn get_palette_length(&self) -> f32 {
        self.palette_length
    }

    pub fn set_palette_length(&mut self, new: f32) -> bool {
        assert!(0.0 <= new && new <= 1.);
        if self.palette_length == new { return false }
        self.palette_length = new;
        true
    }

    pub fn get_offset(&self) -> f32 {
        self.offset
    }

    pub fn set_offset(&mut self, new: f32) -> bool {
        assert!(0.0 <= new && new <= 1.);
        if self.offset == new { return false }
        self.offset = new;
        true
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
            if this_diff == max_percent_diff && (this_add-0.5).abs() < (percent_to_add-0.5).abs() {
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
        let new_colour = self.get_colour_at_percentage(percent, false);

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
            percent += self.offset;
            percent = percent % 1.;
        }

        let mut next_i = self.sorted_colour_map.len()-1;
        for i in 0..self.sorted_colour_map.len() {
            if self.sorted_colour_map[i].percent > percent {
                next_i = i;
                break;
            }
        }

        let (prev, next) = (self.sorted_colour_map[next_i-1], self.sorted_colour_map[next_i]);
        interpolate_colour(
            prev.colour, 
            next.colour, 
            (percent - prev.percent) / (next.percent - prev.percent)
        )
    }

    fn get_constant_palette(&self, max_iterations: usize) -> Vec<Color> {
        let mut palette = Vec::with_capacity(max_iterations+1);

        for i in 0..=max_iterations {
            let total_percent = i as f32 / max_iterations as f32;
            // takes the total percent and converts it to a fraction of the palette length
            let mut length_percent = (total_percent % self.palette_length) / self.palette_length;
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

    pub fn get_full_palette(&self, width: f32, height: f32) -> Texture2D {
        let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);

        let max_iterations = self.palette_cache.len()-1;

        for i in 0..width as u32 { 
            let iteration = max_iterations as f32 * (i as f32 / (width - 1.));
            for j in 0..height as u32 {
                let colour = escape_time(iteration as f64, &self.palette_cache);
                image.set_pixel(i, j, colour);
            }
        }

        Texture2D::from_image(&image)
    }

    pub fn similar_palette(&self, other: &Self) -> bool {
        self.mapping_type == other.mapping_type &&
            self.colour_map.len() == other.colour_map.len()
    }

    fn remove_extremes(sorted_colour_map: &Vec<ColourPoint>) -> Vec<ColourPoint> {
        if sorted_colour_map.len() == 2 { return sorted_colour_map.clone();}

        let mut map = sorted_colour_map.clone();
        map.remove(0);
        map.remove(map.len()-1);

        map
    }

    fn interpolate_colour_maps(map1: &Vec<ColourPoint>, map2: &Vec<ColourPoint>, percent: f32) -> Vec<ColourPoint> {
        let mut colour_map = Vec::with_capacity(map1.len());
        for i in 0..map1.len() {
            colour_map.push(ColourPoint::interpolate_points(&map1[i], &map2[i], percent));
        }

        colour_map
    }

    pub fn interpolate_palettes(palette1: &Palette, palette2: &Palette, percent: f32) -> Palette {
        Palette::new(
            Palette::interpolate_colour_maps(
                &Palette::remove_extremes(&palette1.sorted_colour_map), 
                &Palette::remove_extremes(&palette2.sorted_colour_map), 
                percent
            ),
            palette1.mapping_type.clone(),
            lerp(palette1.palette_length, palette2.palette_length, percent),
            lerp(palette1.offset, palette2.offset, percent)
        )
    }   

    fn get_export_map_string(&self) -> String {
        let mut contents = String::from("");
        for point in self.colour_map.iter() {
            contents.push_str(&format!("([{}][{}][{}][{}],{})", 
                point.colour.r.to_string(),
                point.colour.g.to_string(),
                point.colour.b.to_string(),
                point.colour.a.to_string(),
                point.percent.to_string()
            ))
        }
        contents
    }

    pub fn get_export_string(&self) -> String {
        format!["#{}#{}[{}][{}]",
            self.get_export_map_string(),
            self.mapping_type.export_num(),
            self.palette_length.to_string(),
            self.offset.to_string()
        ]   
    }

    fn import_colour_map_from_str(text: &str) -> Vec<ColourPoint> {
        let mut colour_map = Vec::new();

        let points: Vec<&str> = text.split(")(").collect();
        for point in points.iter() {
            let mut point = String::from(*point);
            point = point.replace("(", "");
            point = point.replace(")", "");

            let mut colours: Vec<&str> = point.split("][").collect();
            let end = colours.pop().unwrap();
            let end = end.split(",").collect::<Vec<&str>>()[0];
            colours.push(end);
            let mut colour = [0.0; 4];
            for (i, colour_str) in colours.iter().enumerate() {
                let mut colour_str = String::from(*colour_str);
                colour_str = colour_str.replace("[", "");
                colour_str = colour_str.replace("]", "");    
                colour[i] = colour_str.parse::<f32>().unwrap();
            }
            let colour = Color::new(colour[0], colour[1], colour[2], colour[3]);

            let percent = point.split(",").last().unwrap().parse::<f32>().unwrap();

            colour_map.push(ColourPoint { colour, percent })
        }

        colour_map
    }

    pub fn import_from_str(text: &str) -> Palette {
        let mapping_num = text[1..].find("#").unwrap() + 2;

        let start_len = text[mapping_num..].find("]").unwrap() + mapping_num + 1;

        Palette::new(
            Palette::import_colour_map_from_str(get_str_between(text, "#", "#")),
            MappingType::import_from_num(text.chars().nth(mapping_num).unwrap()),
            get_str_between(&text[mapping_num..], "[", "]").parse::<f32>().unwrap(),
            get_str_between(&text[start_len..], "[", "]").parse::<f32>().unwrap()
        )
    }
}

#[allow(unused_variables)]

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
pub const ROYAL: [Color; 5] = [
    BLACK,
    Color {r: 212.7/255., g: 156.3/255., b: 79.9/255., a: 1.},
    WHITE,
    Color {r: 70.9/255., g: 125.4/255., b: 255./255., a: 1.},
    BLACK
];
pub const TEST: [Color;5] =[
    PINK,
    BLACK,
    BLUE,
    ORANGE,
    WHITE
];