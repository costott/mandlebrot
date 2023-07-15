// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use std::sync::{Arc, Mutex};
use native_dialog::FileDialog;

use super::{
    ScreenDimensions, Visualiser, interpolate_colour, 
    Fractal, JuliaSeed,
    complex::{ComplexType, Complex},
    layers::*,
    orbit_trap::*,
    palettes::*,
    VideoRecorder, VideoTimestamp
};

/// the proportion of the screen width taken over by the menu
const MENU_SCREEN_PROPORTION: f32 = 0.25;
// the next menu variables apply to all menus apart from the layer editor
/// the proprotion of the screen width for the horizontal padding either size of the menu
const MENU_HOR_PADDING: f32 = MENU_SCREEN_PROPORTION/50.;
/// the proportion of the screen height for the vertical padding in menus
const MENU_VERT_PADDING: f32 = 1./50.;

const HOVER_WHITE_OVERLAY: Color =  Color { r: 1., g: 1., b: 1., a: 0.3};
const HOVER_BLACK_OVERLAY: Color = Color { r: 0., g: 0., b: 0., a: 0.3};
const HOVER_RED_OVERLAY: Color =  Color { r: 1., g: 0., b: 0., a: 0.3};

/// the proportion of the screen height taken over by the navbar
const NAVBAR_HEIGHT_PROPORTION: f32 = 0.08;
/// the proportion of the screen width taken over by the navbar border 
const NAVBAR_BORDER_WIDTH_PROPORTION: f32 = NAVBAR_HEIGHT_PROPORTION*0.04;

/// padding before the title for the state proportional to the screen
const STATE_TEXT_PADDING_PROPORTION: f32 = 1./50.;
/// proportion of the screen width for the size of the state text
const STATE_TEXT_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/10.;

/// the proportion of the screen width for the start value of the default input boxes
const DEFAULT_INPUT_BOX_START_X: f32 = MENU_SCREEN_PROPORTION*0.4;
/// proportion of the screen height for the height of the default input boxes
const DEFAULT_INPUT_BOX_HEIGHT: f32 = 1./20.;
/// proportion of the screen height for the size of the default input box borders
const DEFAULT_INPUT_BOX_BORDER_SIZE: f32 = DEFAULT_INPUT_BOX_HEIGHT/20.;
/// propotion of the screen height for the padding between the default input boxes
const DEFAULT_INPUT_BOX_VERT_PADDING: f32 = DEFAULT_INPUT_BOX_HEIGHT/4.;
/// proportion of the screen width for the size of the default input box label text
const DEFAULT_INPUT_BOX_LABEL_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen width for the padding between the defualt input box labels and boxes
const DEFAULT_INPUT_BOX_LABEL_PADDING: f32 = MENU_SCREEN_PROPORTION/100.;
/// proportion of the screen width for the size of the default input box content text
const DEFAULT_INPUT_BOX_CONTENT_FONT_PROPORTION: f32 = DEFAULT_INPUT_BOX_LABEL_FONT_PROPORTION;
/// proportion of the screen width for the padding between the left of the input box content and the border
const DEFAULT_INPUT_BOX_CONTENT_HOR_PADDING: f32 = MENU_SCREEN_PROPORTION/100.;

/// proportion of the screen width for the padding on the right of text boxes
const TEXTBOX_RIGHT_PADDING: f32 = MENU_SCREEN_PROPORTION/50.;
/// proportion of the screen width for the padding between the text box and content inside
const TEXTBOX_CONTENT_PADDING: f32 = MENU_SCREEN_PROPORTION/100.;
/// time between blinks of the text box cursor
const TEXTBOX_CURSOR_BLINK_TIME: f32 = 0.5;

/// proportion of the screen width for the padding either side of the julia editor
const JULIAEDITOR_HOR_PADDING: f32 = MENU_SCREEN_PROPORTION/8.;

/// padding before the layer managers proportional to the screen
const LAYERMANAGER_LEFT_PADDING: f32 = 0.;// 1./200.;
/// proportion of the scren for the padding to the right of the layer managers
const LAYERMANAGER_RIGHT_PADDING: f32 = 1./75.;
/// proportion of the screen height for the height of layer managers
const LAYERMANAGER_HEIGHT: f32 = 1./7.;
// const LAYEMANAGER_HEIGHT: f32 = 1./8.;
/// proportion of the screen height for the height of the layer manager's palette
const LAYERMANAGER_PALETTE_HEIGHT_PROPORTION: f32 = LAYERMANAGER_HEIGHT*0.6;
/// propotion of the screen height for the bottom padding at the bottom layer manager
const LAYERMANAGER_BOTTOM_PADDING: f32 = 1./200.;
/// proportion of the screen height for the padding between layer managers
const LAYERMANAGER_TOP_PADDING: f32 = 1./200.;
/// proportion of the screen width for the left padding inside the layer managers
const LAYERMANAGER_INNER_LEFT_PADDING: f32 = 1./400.;
/// proportion of the screen height for the top padding inside the layer managers
const LAYERMANAGER_INNER_TOP_PADDING: f32 = 1./200.;
/// proportion of the screen height for the border of the layer managers
const LAYERMANAGER_BORDER_PROPORTION: f32 = LAYERMANAGER_HEIGHT/30.;
/// proportion of the **width of the LayerManager inside** where the first half ends
const LAYERMANAGER_HALF_END_PROPORION: f32 = 0.6;
/// proprtion of the screen width for the padding on the right of the layer manager's palette
const LAYERMANAGER_PALETTE_RIGHT_PADDING: f32 = 1./300.;
/// proportion of the screen height for the height of the name textbox
const LAYERMANAGER_NAME_TEXTBOX_HEIGHT: f32 = 1./25.;
/// proportion of the screen width for the size of the name text 
const LAYERMANAGER_NAME_TEXT_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/30.;
/// proportion of the screen width for the size of the layer type text 
const LAYERMANAGER_LAYER_TYPE_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/30.;
const LAYERMANAGER_LAYER_TYPE_COLOUR: Color = Color { r: 0.55, g: 0.55, b: 0.55, a: 1.};
/// proportion of the screen height for the height of the strength slider
const LAYERMANAGER_STRENGTH_SLIDER_HEIGHT: f32 = LAYERMANAGER_HEIGHT/20.;
/// proportion of the screen width for the size of the strength text 
const LAYERMANAGER_STRENGTH_TEXT_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/25.;
/// proportion of the screen height for the border size of the edit button
const LAYERMANAGER_EDIT_BUTTON_BORDER_HEIGHT: f32 = LAYERMANAGER_PALETTE_HEIGHT_PROPORTION/30.;
/// propotion of the screen width for the size of the layer range font
const LAYERMANAGER_LAYER_RANGE_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/30.;
/// proportion of the screen width for the padding between the border of the layer range box and the contents
const LAYERMANAGER_LAYER_RANGE_CONTENT_HOR_PADDING: f32 = MENU_SCREEN_PROPORTION/100.;
/// proportion of the screen height for the size of the exit button
const LAYERMANAGER_DELETE_BUTTON_SIZE: f32 = LAYERMANAGER_HEIGHT/6.;

/// proportion of the screen height for the height of the layer carousel
const LAYEREDITOR_CAROUSEL_HEIGHT: f32 = 1./15.;
/// proportion of the screen width for the size of the layer carousel
const LAYEREDITOR_CAROUSEL_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/15.;
/// proportion of the screen height for the vertical padding around the type text box
const LAYEREDITOR_INPUT_BOX_VERT_PADDING: f32 = 1./50.;
/// proportion of the screen height for the bar at the top of the specific menu
const LAYEREDTIOR_SPECIFIC_MENU_BAR_HEIGHT: f32 = NAVBAR_BORDER_WIDTH_PROPORTION;
/// proportion of the screen width for the size of the title of the specific menu
const LAYEREDITOR_SPECIFIC_MENU_TITLE_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/12.;

/// proportion of the screen width for the horizontal padding between the edges of the menu
const PALETTEEDITOR_HOR_PADDING: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen height for the vertical padding between sections of the menu
const PALETTEEDITOR_VERT_PADDING: f32 = 1./50.;
/// proportion of the screen height for the height of the palette display
const PALETTEEDITOR_PALETTE_HEIGHT: f32 = 1./10.;
/// proportion of the screen width for the width of the colour points
const PALETTEEDITOR_COLOUR_POINT_WIDTH: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen width for the width of the select box on the palette editor
const PALETTEEDITOR_COLOUR_POINT_SELECT_WIDTH: f32 = MENU_SCREEN_PROPORTION/40.;
/// proportion of the screen width for the width of the select box border on the palette editor
const PALETTEEDITOR_COLOUR_POINT_SELECT_BORDER_WIDTH: f32 = PALETTEEDITOR_COLOUR_POINT_SELECT_WIDTH/3.;
/// proportion of the screen width for the width of the buttons on the palette editor
const PALETTEEDITOR_BUTTON_WIDTH: f32 = MENU_SCREEN_PROPORTION/7.;
/// proportion of the screen width for the border width of the palette editor buttons
const PALETTEEDIOR_BUTTON_BORDER_WIDTH: f32 = PALETTEEDITOR_BUTTON_WIDTH/10.;
/// proportion of the screen width for the size of the palette editor text
const PALETTEEDTIOR_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen width for the width of the text boxes on the palette editor
const PALETTEEDITOR_TEXTBOX_WIDTH: f32 = MENU_SCREEN_PROPORTION/6.;
/// proportion of the screen height for the height of the text boxes on the palette editor
const PALETTEEDITOR_TEXTBOX_HEIGHT: f32 = 1./23.;
/// proportion of the screen height for the padding between text boxes on the palette editor
const PALETTEEDITOR_TEXTBOX_VERT_PADDING: f32 = PALETTEEDITOR_TEXTBOX_HEIGHT/4.;
/// proportion of the screen width for the start x of the colour sliders
const PALETTEEDITOR_COLOUR_SLIDER_START_X: f32 = MENU_SCREEN_PROPORTION/4.;
/// proportion of the screen height for the height of the colour slider
const PALETTEEDITOR_COLOUR_SLIDER_HEIGHT: f32 = 1./50.;
/// proportion of the screen height for the bar in the palette editor
const PALETTEEDITOR_BAR_HEIGHT: f32 = NAVBAR_BORDER_WIDTH_PROPORTION;
/// proportion of the screen width for the width of the mapping type dropdown
const PALETTEEDITOR_MAPPING_DROPDOWN_WIDTH: f32 = PALETTEEDITOR_TEXTBOX_WIDTH*2.2;

/// proportion of the screen height for the vertical padding between sections of the menu
const SCREENSHOT_VERT_PADDING: f32 = 1./50.;
/// proportion of the screen height for the height of the bar in the screenshot menu
const SCREENSHOT_BAR_HEIGHT: f32 = NAVBAR_BORDER_WIDTH_PROPORTION;
/// proportion of the screen width for the width of the buttons on the screenshot menu
const SCREENSHOT_BUTTON_WIDTH: f32 = MENU_SCREEN_PROPORTION/5.;
/// proportion of the screen width for the border width of the screenshot buttons
const SCREENSHOT_BUTTON_BORDER_WIDTH: f32 = SCREENSHOT_BUTTON_WIDTH/10.;

/// proportion of the screen width for the width of the progress bar
const PROGRESS_BAR_WIDTH: f32 = MENU_SCREEN_PROPORTION*0.8;
/// proportion of the screen height for the height of the progress bar
const PROGRESS_BAR_HEIGHT: f32 = 1./40.;
/// proportion of the screen height for the vertical padding around the progress bar
const PROGRESS_BAR_VERT_PADDING: f32 = 1./15.;
const PROGRESS_BAR_COLOUR: Color = LAYERMANAGER_LAYER_TYPE_COLOUR;
/// proportion of the screen width for the size of the progress bar text
const PROGRESS_BAR_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen height for the padding between the progress bar and percent text
const PROGRERSS_BAR_TEXT_PADDING: f32 = 1./75.;

/// proportion of the screen height for the height of the timeline
const VIDEORECORDER_TIMELINE_HEIGHT: f32 = 1./8.;
/// proportion of the screen height for the vertical padding around the timeline
const VIDEORECORDER_TIMELINE_VERT_PADDING: f32 = 1./40.;
/// proportion of the screen height for the height of the timeline line
const VIDEORECORDER_TIMELINE_LINE_HEIGHT: f32 = VIDEORECORDER_TIMELINE_HEIGHT/20.;
/// proportion of the screen height for the height of the small bars
const VIDEORECORDER_TIMELINE_SMALL_BAR_HEIGHT: f32 = VIDEORECORDER_TIMELINE_HEIGHT/2.;
const VIDEORECORDER_TIMELINE_PREVIEW_BAR_COLOUR: Color = Color { r: 1., g: 1., b: 1., a: 0.5 };
/// proportion of the screen width for the size of the timeline text
const VIDEORECORDER_TIMELINE_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/30.;
/// proportion of the screen width for the vertical padding between the timeline text and timeline
const VIDEORECORDER_TIMELINE_TEXT_PADDING: f32 = VIDEORECORDER_TIMELINE_FONT_PROPORTION/2.;
/// proportion of the screen width for the width of the timestamp editors
const VIDEORECORDER_TIMESTAMP_WIDTH: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen width for the start x of the the textboxes
const VIDEORECORDER_TEXTBOX_START_X: f32 = MENU_SCREEN_PROPORTION*0.52;

/// proportion of the screen width for the width of the leave menu
const LEAVEMENU_WIDTH: f32 = 0.5;
/// proportion of the screen height for the height of the leave menu
const LEAVEMENU_HEIGHT: f32 = 0.8;
/// proportion of the screen width for the size of the leave menu borders
const LEAVEMENU_BORDER_SIZE: f32 = LEAVEMENU_WIDTH/150.;
const LEAVEMENU_BACK_OVERLAY: Color = Color { r: 1., g: 1., b: 1., a: 0.5};
/// porportion of the screen width for the width of the leave menu option buttons
const LEAVEMENU_OPTION_WIDTH: f32 = LEAVEMENU_WIDTH*0.9;
/// proportion of the screen height for the height of the leave menu option buttons
const LEAVEMENU_OPTION_HEIGHT: f32 = LEAVEMENU_HEIGHT/8.;
/// proportion of the screen width for the size of the leave menu font
const LEAVEMENU_OPTION_FONT_PROPORTION: f32 = LEAVEMENU_OPTION_WIDTH/25.;
/// proportion of the screen height for the vertical padding between elements in the leave menu
const LEAVEMENU_VERT_PADDING: f32 = LEAVEMENU_HEIGHT/20.;
/// proportion of the screen width for the size of the exit button on the leave menu
const LEAVEMENU_EXIT_SIZE: f32 = LEAVEMENU_WIDTH/10.;

/// gives a texture which is a snippet of the gradient for the menu at the given place
fn get_back_gradient(visualiser: &Visualiser, start_x: u16, width: u16, height: u16) -> Texture2D {
    let mut image = Image::gen_image_color(width, height, BLACK);
    let grad_width = screen_width()*MENU_SCREEN_PROPORTION;

    for i in 0..width {
        let percent = ((start_x + i) as f32) % grad_width / grad_width;
        let mut colour = None;
        for layer in visualiser.layers.layers.iter() {
            // has to be a colour layer out of the set
            if layer.layer_type.shading_layer() || !layer.layer_range.layer_applies(false) {
                continue
            }
            let layer_colour = layer.palette.get_colour_at_percentage(percent, false);
            colour = match colour {
                None => Some(layer_colour),
                Some(c) => Some(interpolate_colour(c, layer_colour, 0.9 * layer.strength))
            };
        }
        for j in 0..height {
            image.set_pixel(i as u32, j as u32, 
                match colour {
                    Some(c) => c,
                    None => WHITE
                }
            );
        }
    }

    Texture2D::from_image(&image)
}

/// returns the colour with the highest luminance in the texture
/// if the brigtest colour is too dark, white is returned
/// 
/// # Params
/// `gradient`: a texture where the colours are all in vertical bars
fn get_brightest_colour(gradient: Texture2D) -> Color {
    let image = gradient.get_texture_data();

    let mut brightest_colour: Color = BLACK;
    let mut largest_luminance = 0.0;
    for i in 0..gradient.width() as u32 {
        let colour = image.get_pixel(i, 0);
        let luminance = 0.299*colour.r + 0.587*colour.g + 0.114*colour.r;
        if luminance < largest_luminance { continue }
        largest_luminance = luminance;
        brightest_colour = colour;
    }

    if largest_luminance < 0.1 {
        brightest_colour = WHITE;
    }

    brightest_colour
}

fn translate_rect(rect: &mut Rect, translate: (f32, f32)) {
    rect.x += translate.0;
    rect.y += translate.1;
}

/// Returns a new `Rect` representing the given `Rect` after
/// being stretched by a size at each border
fn inflate_rect(rect: &Rect, size: f32) -> Rect {
    Rect::new(
        rect.x - size,
        rect.y - size,
        rect.w + 2. * size,
        rect.h + 2. * size
    )
}

fn draw_rect(rect: &Rect, colour: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, colour);
}

/// returns the y position of the bottom of the navbar
fn navbar_bottom() -> f32 {
    screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) + 
        screen_width()*STATE_TEXT_FONT_PROPORTION
}

enum MenuSignal {
    None,
    OpenEditor(usize),
    OpenPalette(usize),
    RecordVideo,
    RefreshGradients,
    Import
}

#[derive(Clone, Copy, PartialEq)]
enum MenuState {
    Closed,
    General,
    Layers,
    LayerEditor,
    Screenshot,
    Video,
    PaletteEditor,
    VideoRecorder,
    /// integer specifies the index of the next menu
    UpdateGradient(usize)
}
impl MenuState {
    fn map_button_states(button_i: isize) -> MenuState {
        match button_i {
            0 => MenuState::General,
            1 => MenuState::Layers,
            2 => MenuState::LayerEditor,
            3 => MenuState::Screenshot,
            4 => MenuState::Video,
            _ => MenuState::General
        }
    }

    fn no_navbar(&self) -> bool {
        self == &MenuState::PaletteEditor || self == &MenuState::VideoRecorder
    }

    fn map_state_indexes(&self) -> usize {
        match self {
            MenuState::General => 0,
            MenuState::Layers => 1,
            MenuState::LayerEditor => 2,
            MenuState::Screenshot => 3,
            MenuState::Video => 4,
            MenuState::PaletteEditor => 5,
            MenuState::VideoRecorder => 6,
            MenuState::Closed => 7,
            MenuState::UpdateGradient(_) => 8
        }
    }

    fn get_string(&self) -> String {
        String::from(match self {
            MenuState::General => "GENERAL",
            MenuState::Layers => "LAYERS",
            MenuState::LayerEditor => "LAYER EDITOR",
            MenuState::Screenshot => "SCREENSHOT",
            MenuState::Video => "VIDEO",
            // not actually needed
            MenuState::PaletteEditor => "PALETTE EDITOR",
            MenuState::VideoRecorder => "VIDEO RECORDER",
            _ => ""
        })
    }

    /// draw the state to the screen 
    fn draw_state(&self, font: Font, colour: Color) {
        if self == &MenuState::PaletteEditor { return}

        let text = &self.get_string();
        let font_size = (screen_width() * STATE_TEXT_FONT_PROPORTION) as u16;
        let dims = measure_text(text, Some(font), font_size, 1.0);
        draw_rectangle(
            screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION,
            screen_height()*NAVBAR_HEIGHT_PROPORTION,
            screen_width()*(MENU_SCREEN_PROPORTION-2.*NAVBAR_BORDER_WIDTH_PROPORTION),
            screen_width()*(STATE_TEXT_FONT_PROPORTION-NAVBAR_BORDER_WIDTH_PROPORTION) + screen_height()*2.*STATE_TEXT_PADDING_PROPORTION,
            BLACK
        );
        draw_text_ex(
            text,
            screen_width()*MENU_SCREEN_PROPORTION*0.5 - dims.width*0.5,
            screen_height()*NAVBAR_HEIGHT_PROPORTION + dims.height + screen_height()*STATE_TEXT_PADDING_PROPORTION,
            TextParams {font, font_size, color: colour, ..Default::default()}
        );
    }

    async fn create_menu(&self, visualiser: &mut Visualiser, index: usize) -> Option<Box<dyn MenuType>> {
        match index {
            0 => Some(Box::new(GeneralMenu::new(visualiser).await)),
            1 => Some(Box::new(LayersMenu::new(visualiser).await)),
            2 => Some(Box::new(LayerEditorMenu::new(visualiser).await)),
            3 => Some(Box::new(ScreenshotMenu::new(&visualiser).await)),
            4 => Some(Box::new(VideoMenu::new(&visualiser).await)),
            5 => Some(Box::new(PaletteEditor::new(&visualiser).await)),
            6 => Some(Box::new(VideoRecorderMenu::new(&visualiser).await)),
            _ => None
        }
    }

    fn refresh_gradients(&mut self, menus: &mut [Option<Box<dyn MenuType>>; 7], visualiser: &mut Visualiser) {
        for menu in menus {
            match menu {
                Some(m) => m.refresh_gradients(visualiser),
                None => {}
            }
        }
        *self = match self {
            MenuState::PaletteEditor => MenuState::UpdateGradient(0),
            _ => MenuState::UpdateGradient(self.map_state_indexes())
        }
    }

    async fn process_signal(
        &mut self, 
        menus: &mut [Option<Box<dyn MenuType>>; 7], 
        visualiser: &mut Visualiser, 
        signal: MenuSignal
    ) {
        match signal {
            MenuSignal::None => {},
            MenuSignal::OpenEditor(index) => {
                if menus[2].is_none() {
                    menus[2] = self.create_menu(visualiser, 2).await
                }
                match &mut menus[2] {
                    None => panic!("layer editor menu failed to be created"),
                    Some(m) => m.as_mut().open_layer_to_edit(index, &visualiser)
                }
                *self = MenuState::LayerEditor;
            },
            MenuSignal::OpenPalette(index) => {
                if menus[5].is_none() {
                    menus[5] = self.create_menu(visualiser, 5).await
                }
                match &mut menus[5] {
                    None => panic!("palette editor menu failed to be created"),
                    Some(m) => m.as_mut().open_layer_to_edit(index, &visualiser)
                }
                *self = MenuState::PaletteEditor;
            },
            MenuSignal::RecordVideo => {
                if menus[6].is_none() {
                    menus[6] = self.create_menu(visualiser, 6).await
                }
                match &mut menus[6] {
                    None => panic!("video recorder menu failed to be created"),
                    Some(m) => m.as_mut().open_layer_to_edit(0, visualiser)
                }
                *self = MenuState::VideoRecorder;
            }
            MenuSignal::RefreshGradients => self.refresh_gradients(menus, visualiser),
            MenuSignal::Import => {
                menus[1] = self.create_menu(visualiser, 1).await;
                self.refresh_gradients(menus, visualiser)
            }
        }
    }

    async fn update_state_menu(&mut self, menus: &mut [Option<Box<dyn MenuType>>; 7], visualiser: &mut Visualiser, index: usize) {
        match &mut menus[index] {
            None => {
                menus[index] = self.create_menu(visualiser, index).await;
            },
            Some(m) => {
                let signal = m.as_mut().update(visualiser);
                self.process_signal(menus, visualiser, signal).await;
            }
        }
    }

    /// updates the menu for the current state
    async fn update_state(&mut self, menus: &mut [Option<Box<dyn MenuType>>; 7], visualiser: &mut Visualiser) {
        self.update_state_menu(menus, visualiser, self.map_state_indexes()).await;
    }

    fn get_editing_menu(&self, menus: &mut [Option<Box<dyn MenuType>>; 7], index: usize) -> bool {
        match &mut menus[index] {
            None => {false},
            Some(m) => m.as_mut().get_editing()
        }
    }

    fn get_editing(&self, menus: &mut [Option<Box<dyn MenuType>>; 7]) -> bool {
        match self {
            MenuState::Closed => {false},
            _ => self.get_editing_menu(menus, self.map_state_indexes())
        }
    }
}

/// The menu for the user to edit the visualiser
pub struct Menu {
    /// size of the visualiser when the menu is open
    visualiser_menu_size: ScreenDimensions,
    state: MenuState,
    gradient: Texture2D,
    text_colour: Color,
    state_font: Font,
    open_button: Button,
    close_button: Button,
    navbar: Navbar,
    menus: [Option<Box<dyn MenuType>>; 7],
    pub leave_menu: LeaveMenu,
    updated_gradient: bool
}
impl Menu {
    pub async fn new(visualiser: &Visualiser) -> Menu {
        let button_border = screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION;
        let open_rect = Rect::new(0., 0., 
            screen_height()*NAVBAR_HEIGHT_PROPORTION,
            screen_height()*NAVBAR_HEIGHT_PROPORTION
        );
        let mut close_rect = open_rect.clone();
        close_rect.x = screen_width()*MENU_SCREEN_PROPORTION;

        Menu { 
            state: MenuState::Closed,
            visualiser_menu_size: ScreenDimensions::new(
                ((1.0-MENU_SCREEN_PROPORTION)*screen_width()) as usize,
                screen_height() as usize
            ),
            gradient: Texture2D::empty(),
            text_colour: BLACK,
            state_font: load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap(),
            open_button: Button::gradient_border_and_image(visualiser, &open_rect, button_border, 
                load_image("assets/triangle.png").await.unwrap(), DrawTextureParams { flip_x: true, ..Default::default() }, 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            close_button: Button::gradient_border_and_image(visualiser, &close_rect, button_border, 
                load_image("assets/triangle.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            navbar: Navbar::new().await,
            menus: [None, None, None, None, None, None, None],
            leave_menu: LeaveMenu::new(visualiser).await,
            updated_gradient: false
        }
    }

    /// what the menu does when closed
    fn menu_state_closed(&mut self, visualiser: &mut Visualiser) {
        if visualiser.moving {return}
        self.open_button.update();
        if self.open_button.clicked {
            self.open_menu(visualiser);
        }
    }

    fn update_gradient(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.gradient);
        self.gradient = get_back_gradient(
            &visualiser, 
            0, 
            (MENU_SCREEN_PROPORTION*screen_width()) as u16, 
            (screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) +
                    screen_width()*STATE_TEXT_FONT_PROPORTION) as u16
        );
        self.navbar.back = self.gradient;
        self.text_colour = get_brightest_colour(self.gradient);

        self.leave_menu.refresh_gradients(visualiser);

        self.open_button.refresh_gradient(visualiser);
        self.close_button.refresh_gradient(visualiser);

        self.updated_gradient = true;
    }

    /// updates the menu every frame
    pub async fn update(&mut self, visualiser: &mut Visualiser) {
        if !self.updated_gradient {
            self.update_gradient(visualiser);
        }

        if self.state == MenuState::Closed {
            self.menu_state_closed(visualiser);
            return;
        }

        self.state.update_state(&mut self.menus, visualiser).await;
        match self.state {
            MenuState::UpdateGradient(next_i) => {
                self.update_gradient(visualiser);
                self.state = MenuState::map_button_states(next_i as isize);
            },
            _ => {}
        }

        self.state = self.navbar.update(self.state, self.state_font, self.text_colour);

        if self.state.no_navbar() { return }

        self.close_button.update();
        if self.close_button.clicked {
            self.close_menu(visualiser);
        }
    }

    pub fn get_editing(&mut self) -> bool {
        self.state.get_editing(&mut self.menus)
    }

    fn open_menu(&mut self, visualiser: &mut Visualiser) {
        visualiser.quality = crate::MAX_QUALITY;
        visualiser.set_view_dimensions(&self.visualiser_menu_size);
        self.state = MenuState::General;
    }

    fn close_menu(&mut self, visualiser: &mut Visualiser) {
        visualiser.quality = crate::MAX_QUALITY;
        visualiser.set_view_dimensions(&ScreenDimensions::screen_size());
        self.state = MenuState::Closed;
    }
}

trait ButtonElement: ButtonElementClone { 
    fn draw(&self, button_rect: &Rect);
    /// lower draw order => drawn first
    fn get_draw_order(&self) -> usize;
    fn refresh_gradient(&mut self, _visualiser: &Visualiser) {}
    fn refresh_from_container(&mut self, _visualiser: &Visualiser, _container: &Rect) {}
    fn drop_textures(&self) {}
    fn gradient_change_layer_i(&mut self, _layer_i: usize) {}
}

trait ButtonElementClone {
    fn clone_box(&self) -> Box<dyn ButtonElement>;
}

impl<T> ButtonElementClone for T
where
    T: 'static + ButtonElement + Clone,
{ 
    fn clone_box(&self) -> Box<dyn ButtonElement> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ButtonElement> {
    fn clone(&self) -> Box<dyn ButtonElement> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct ButtonImageElement {
    image: Texture2D,
    params: DrawTextureParams,
    alpha_colour: Color,
    /// offset from the topleft of the button
    offset: (f32, f32),
    draw_order: usize
}
impl ButtonImageElement {
    fn new(image: Image, alpha: f32, params: DrawTextureParams, offset: (f32, f32), draw_order: usize) -> ButtonImageElement {
        ButtonImageElement { 
            image: Texture2D::from_image(&image), 
            alpha_colour: Color::new(1., 1., 1., alpha),
            params: params, 
            offset, 
            draw_order
        }
    }
}
impl ButtonElement for ButtonImageElement {
    fn draw(&self, button_rect: &Rect) {
        draw_texture_ex(
            self.image, 
            button_rect.x+self.offset.0, 
            button_rect.y+self.offset.1, 
            self.alpha_colour, 
            self.params.clone()
        );
    }

    fn get_draw_order(&self) -> usize {
        self.draw_order
    }

    fn drop_textures(&self) {
        Texture2D::delete(&self.image);
    }
}

#[derive(Clone)]
struct ButtonGradientElement {
    layer_i: Option<usize>,
    gradient: Texture2D,
    offset: (f32, f32),
    alpha_colour: Color,
    screen_rect: Rect,
    draw_order: usize
}
impl ButtonGradientElement{
    /// if a layer_i is provided, the gradient is the full palette
    /// if not, the gradient is the back gradient
    fn new(
        visualiser: &Visualiser,
        layer_i: Option<usize>,
        button_topleft: (f32, f32), 
        size: (f32, f32), 
        offset: (f32, f32), 
        alpha_colour: Color, 
        draw_order: usize
    ) -> ButtonGradientElement {
        let screen_rect = Rect::new(button_topleft.0+offset.0, button_topleft.1+offset.1, size.0, size.1);
        ButtonGradientElement { 
            gradient: match &layer_i {
                None => get_back_gradient(visualiser, screen_rect.x as u16, screen_rect.w as u16, screen_rect.h as u16),
                Some(i) => visualiser.layers.layers[*i].palette.get_full_gradient(size.0, size.1)
            },
            layer_i,
            offset, 
            alpha_colour, 
            screen_rect, 
            draw_order
        }
    }

    /// if a layer_i is provided, the gradient is the full palette
    /// if not, the gradient is the back gradient
    fn full_back(
        visualiser: &Visualiser,
        layer_i: Option<usize>,
        button_rect: &Rect,
        alpha_colour: Color,
        draw_order: usize
    ) -> ButtonGradientElement {
        ButtonGradientElement::new(
            visualiser, layer_i,
            button_rect.point().into(), button_rect.size().into(),
            (0., 0.), alpha_colour, draw_order
        )
    }
}
impl ButtonElement for ButtonGradientElement {
    fn draw(&self, button_rect: &Rect) {
        draw_texture(self.gradient, button_rect.x+self.offset.0, 
                     button_rect.y+self.offset.1, self.alpha_colour);
    }

    fn get_draw_order(&self) -> usize {
        self.draw_order
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.gradient);
        self.gradient = match &self.layer_i {
            None => get_back_gradient(visualiser, self.screen_rect.x as u16, self.screen_rect.w as u16, self.screen_rect.h as u16),
            Some(i) => visualiser.layers.layers[*i].palette.get_full_gradient(self.screen_rect.w, self.screen_rect.h)
        };
    }

    fn gradient_change_layer_i(&mut self, layer_i: usize) {
        self.layer_i = Some(layer_i);
    }

    fn drop_textures(&self) {
        Texture2D::delete(&self.gradient);
    }
}

#[derive(Clone)]
struct ButtonColourElement {
    colour: Color,
    size: (f32, f32),
    /// offset from the topleft of the button
    offset: (f32, f32),
    draw_order: usize
}
impl ButtonColourElement {
    fn new(colour: Color, size: (f32, f32), offset: (f32, f32), draw_order: usize) -> ButtonColourElement {
        ButtonColourElement { colour, size, offset, draw_order }
    }

    fn full_button(button_rect: &Rect, colour: Color, draw_order: usize) -> ButtonColourElement {
        ButtonColourElement::new(colour, button_rect.size().into(), (0., 0.), draw_order)
    }

    fn inner_from_border(button_rect: &Rect, border_size: f32, draw_order: usize) -> ButtonColourElement {
        let inner_rect = inflate_rect(button_rect, -border_size);
        ButtonColourElement::new(BLACK, inner_rect.size().into(), (border_size, border_size), draw_order)
    }
}
impl ButtonElement for ButtonColourElement {
    fn draw(&self, button_rect: &Rect) {
        draw_rectangle(button_rect.x+self.offset.0, button_rect.y+self.offset.1, 
                       self.size.0, self.size.1, self.colour);
    }

    fn get_draw_order(&self) -> usize {
        self.draw_order
    }
}

#[derive(Clone)]
struct ButtonTextElement {
    text: InputLabel,
    draw_order: usize
}
impl ButtonTextElement {
    fn new(text: &str, font: Font, font_size: f32, color: Color, padding: f32, alignment: TextAlign, draw_order: usize) -> ButtonTextElement {
        ButtonTextElement { 
            text: InputLabel::new(text, font, font_size, color, true, padding, alignment),
            draw_order
        }
    }

    fn get_gradient_colour(visualiser: &Visualiser, container: &Rect) -> Color {
        let gradient = get_back_gradient(visualiser, container.x as u16, container.w as u16, container.h as u16);
        let colour = get_brightest_colour(gradient);
        Texture2D::delete(&gradient);
        colour
    }

    fn new_gradient_colour(
        visualiser: &Visualiser, 
        text: &str, font: Font, font_size: f32, padding: f32, 
        container: &Rect,
        alignment: TextAlign, draw_order: usize
    ) -> ButtonTextElement {
        ButtonTextElement::new(text, font, font_size, 
            ButtonTextElement::get_gradient_colour(visualiser, container), 
            padding, alignment, draw_order
        )
    }
}
impl ButtonElement for ButtonTextElement {
    fn draw(&self, button_rect: &Rect) {
        let container = InputBox::from_outer_rect(button_rect.clone(), 0.);
        self.text.draw(&container);
    }

    fn get_draw_order(&self) -> usize {
        self.draw_order
    }

    fn refresh_from_container(&mut self, visualiser: &Visualiser, container: &Rect) {
        self.text.refresh_gradient(ButtonTextElement::get_gradient_colour(visualiser, container));
    }
}

#[derive(Clone)]
pub struct Button {
    rect: Rect,
    back_elements: Vec<Box<dyn ButtonElement>>,
    hover_elements: Vec<Box<dyn ButtonElement>>,
    hold_elements: Vec<Box<dyn ButtonElement>>,
    clicked: bool,
    hovering: bool,
    holding: bool
}
impl Button {
    fn new(
        size: (f32, f32),
        topleft: (f32, f32), 
        back_elements: Vec<Box<dyn ButtonElement>>,
        hover_elements: Vec<Box<dyn ButtonElement>>,
        hold_elements: Vec<Box<dyn ButtonElement>>,
    ) -> Button {
        let rect = Rect::new(topleft.0, topleft.1, size.0, size.1);
        Button::from_rect(&rect, back_elements, hover_elements, hold_elements)
    }

    fn from_rect(
        rect: &Rect,
        back_elements: Vec<Box<dyn ButtonElement>>,
        hover_elements: Vec<Box<dyn ButtonElement>>,
        hold_elements: Vec<Box<dyn ButtonElement>>,
    ) -> Button {
        Button {
            rect: rect.clone(),
            back_elements, hover_elements, hold_elements,
            clicked: false, hovering: false, holding: false
        }
    }

    /// returns a new button which has a gradient border
    /// with a given image inside
    /// 
    /// the dest size of image params can be ignored
    fn gradient_border_and_image(
        visualiser: &Visualiser,
        rect: &Rect,
        border_size: f32,
        image: Image,
        image_params: DrawTextureParams,
        hover_colour: Color,
        hold_colour: Color
    ) -> Button {
        let inner_rect = inflate_rect(rect, -border_size);
        let mut image_params = image_params.clone();
        image_params.dest_size = Some(inner_rect.size());

        Button::from_rect(
            rect,
            vec![
                Box::new(ButtonGradientElement::full_back(visualiser, None, rect, WHITE, 0)),
                Box::new(ButtonColourElement::inner_from_border(rect, border_size, 1)),
                Box::new(ButtonImageElement::new(image, 1., image_params, (border_size, border_size), 2))
            ],
            vec![Box::new(ButtonColourElement::full_button(rect, hover_colour, 3))],
            vec![Box::new(ButtonColourElement::full_button(rect, hold_colour, 4))]
        )
    }

    fn gradient_border_and_alternating_image(
        visualiser: &Visualiser,
        rect: &Rect,
        border_size: f32,
        main_image: Image,
        main_params: DrawTextureParams,
        hover_image: Image,
        hover_params: DrawTextureParams,
        hover_colour: Color,
        hold_colour: Color
    ) -> Button {
        let inner_rect = inflate_rect(rect, -border_size);
        let mut main_params = main_params.clone();
        main_params.dest_size = Some(inner_rect.size());
        let mut hover_params = hover_params.clone();
        hover_params.dest_size = Some(inner_rect.size());

        Button::from_rect(
            rect,
            vec![
                Box::new(ButtonGradientElement::full_back(visualiser, None, rect, WHITE, 0)),
                Box::new(ButtonColourElement::inner_from_border(rect, border_size, 1)),
                Box::new(ButtonImageElement::new(main_image, 1., main_params, (border_size, border_size), 2))
            ],
            vec![
                Box::new(ButtonColourElement::inner_from_border(rect, border_size, 3)),
                Box::new(ButtonImageElement::new(hover_image, 1., hover_params, (border_size, border_size), 4)),
                Box::new(ButtonColourElement::full_button(rect, hover_colour, 5))
            ],
            vec![Box::new(ButtonColourElement::full_button(rect, hold_colour, 6))]
        )
    }

    fn gradient_border_and_text(
        visualiser: &Visualiser,
        rect: &Rect,
        border_size: f32,
        text: &str, font: Font, font_size: f32,
        text_padding: f32, text_align: TextAlign,
        hover_colour: Color, hold_colour: Color
    ) -> Button {
        Button::from_rect(
            rect, 
            vec![
                Box::new(ButtonGradientElement::full_back(visualiser, None, rect, WHITE, 0)),
                Box::new(ButtonColourElement::inner_from_border(rect, border_size, 1)),
                Box::new(ButtonTextElement::new_gradient_colour(visualiser, text, font, font_size, text_padding, rect, text_align, 2))
            ], 
            vec![Box::new(ButtonColourElement::full_button(rect, hover_colour, 3))],
            vec![Box::new(ButtonColourElement::full_button(rect, hold_colour, 4))]
        )
    }

    /// call while button is active to carry out its tasks
    fn update(&mut self) {
        self.draw();
        self.mouse_interact();
    }

    /// draw the button to the screen
    fn draw(&self) {
        let mut draw_elements_vec: Vec<&Box<dyn ButtonElement>> = match (self.hovering, self.holding) {
            (true, false) => self.back_elements.iter().chain(self.hover_elements.iter()).collect(),
            (true, true) => self.back_elements.iter().chain(self.hover_elements.iter()).chain(self.hold_elements.iter()).collect(),
            _ => self.back_elements.iter().collect()
        };
        draw_elements_vec.sort_by_key(|k| k.get_draw_order());

        for element in draw_elements_vec {
            element.as_ref().draw(&self.rect);
        }
    }

    /// controls mouse interaction with the buttons
    fn mouse_interact(&mut self) {
        if !self.rect.contains(Vec2::from(mouse_position())) {
            self.clicked = false;
            self.hovering = false;
            self.holding = false;
            return;
        }

        self.hovering = true; 
        self.clicked = false;
        if is_mouse_button_pressed(MouseButton::Left) {
            self.holding = true;
        } 
        if self.holding && is_mouse_button_down(MouseButton::Left) {
            self.holding = true;
        }
        if self.holding && is_mouse_button_released(MouseButton::Left) {
            self.clicked = true;
            self.holding = false;
        }
    }

    fn translate(&mut self, translate: (f32, f32)) {
        translate_rect(&mut self.rect, translate);
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        let all_elements: Vec<&mut Box<dyn ButtonElement>> = self.back_elements.iter_mut()
            .chain(self.hover_elements.iter_mut())
            .chain(self.hold_elements.iter_mut()).collect();
        for element in all_elements {
            element.refresh_gradient(visualiser);
            element.refresh_from_container(visualiser, &self.rect)
        }
    }

    fn drop_textures(&self) {
        let all_elements: Vec<&Box<dyn ButtonElement>> = self.back_elements.iter()
            .chain(self.hover_elements.iter())
            .chain(self.hold_elements.iter()).collect();
        for element in all_elements {
            element.drop_textures();
        }
    }
}

/// Navbar used to let the user switch between menus
struct Navbar {
    back: Texture2D,
    buttons: [NavbarButton; 5]
}
impl Navbar {
    async fn new() -> Navbar {
        Navbar {
            back: Texture2D::empty(),
            buttons: [
                NavbarButton::new("assets/general.png", 0).await,
                NavbarButton::new("assets/layers.png", 1).await,
                NavbarButton::new("assets/editlayers.png", 2).await,
                NavbarButton::new("assets/screenshot.png", 3).await,
                NavbarButton::new("assets/video.png", 4).await
            ],
        }
    }

    fn set_active_button(&mut self, menu_state: MenuState) {
        for button in self.buttons.iter_mut() { button.set_active(false) };
        self.buttons[menu_state.map_state_indexes()].set_active(true)
    }

    /// called once per frame to draw and change the navbar
    /// 
    /// returns the menu state the menu should be in
    fn update(&mut self, menu_state: MenuState, state_font: Font, text_colour: Color) -> MenuState {
        if menu_state.no_navbar() {
            return menu_state;
        }
        
        draw_texture(self.back, 0., 0., WHITE);
        for button in self.buttons.iter_mut() {
            button.update()
        }

        menu_state.draw_state(state_font, text_colour);
        self.set_active_button(menu_state);

        if !is_mouse_button_pressed(MouseButton::Left) { return menu_state; }

        let (m_x, m_y) = mouse_position();
        // mouse is outside of the navbar
        if  m_x > self.back.width() || m_y > self.back.height() { return menu_state; }

        // holds the index of the navbar button clicked
        let mut clicked_button = -1;
        for (i, button) in self.buttons.iter().enumerate() {
            if button.mouse_hovering() {
                clicked_button = i as isize;
                break;
            }
        }
        if clicked_button == -1 { return menu_state; }

        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.set_active(i == clicked_button as usize);
        }
        MenuState::map_button_states(clicked_button)
    }
}

/// a button on the navbar 
struct NavbarButton {
    image: Texture2D,
    image_params: DrawTextureParams,
    active: bool,
    interact_rect: Rect,
    background_rect: Rect,
    inactive_size: (f32, f32),
    active_size: (f32, f32),
    hovering: bool,
    hover_shade: Color
}
impl NavbarButton {
    async fn new(image_path: &str, button_num: u32) -> NavbarButton {
        let border_width = screen_width() * NAVBAR_BORDER_WIDTH_PROPORTION;

        let width = (screen_width() * MENU_SCREEN_PROPORTION - 5.*2.*border_width) / 5.;
        let inactive_height = screen_height() * NAVBAR_HEIGHT_PROPORTION - 2.*border_width;
        let active_height = inactive_height + border_width;

        let x = screen_width() * NAVBAR_BORDER_WIDTH_PROPORTION + button_num as f32*( 2.*border_width + width );

        NavbarButton {
            image: Texture2D::from_image(&load_image(image_path).await.unwrap()),
            image_params: DrawTextureParams { dest_size: Some(vec2(width, inactive_height)), ..Default::default()},
            active: button_num == 0,
            interact_rect: Rect::new(x-border_width, 0., width+2.*border_width, active_height+border_width),
            background_rect: Rect::new(x, border_width, width, if button_num != 0 {inactive_height} else {active_height}),
            inactive_size: (width, inactive_height),
            active_size: (width, active_height),
            hovering: false,
            hover_shade: Color::new(0., 0., 0., 0.5)
        }
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
        if self.active {
            self.background_rect.w = self.active_size.0;
            self.background_rect.h = self.active_size.1;
        } else {
            self.background_rect.w = self.inactive_size.0;
            self.background_rect.h = self.inactive_size.1;
        }
    }

    /// called once per frame to draw and change the navbar button
    fn update(&mut self) {
        self.hovering = self.mouse_hovering();
        self.draw();
    }
    
    // draw the navbar button to the screen
    fn draw(&self) {
        draw_rectangle(self.background_rect.x, self.background_rect.y, self.background_rect.w,self.background_rect.h, BLACK);
        draw_texture_ex(self.image, self.background_rect.x, self.background_rect.y, WHITE, self.image_params.clone());

        if !self.hovering { return }
        draw_rectangle(self.interact_rect.x, self.interact_rect.y, self.interact_rect.w, self.interact_rect.h, self.hover_shade);
    }

    /// returns whether the mouse is hovering over the navbar
    fn mouse_hovering(&self) -> bool {
        self.interact_rect.contains(Vec2::from(mouse_position()))
    }
}

/// a box for data input elements to use
#[derive(Clone)]
struct InputBox {
    /// a rectangle which contains the whole input box
    outer_rect: Rect,
    /// a rectangle which contains the input box inside the border
    inner_rect: Rect,
    border_size: f32
}
impl InputBox {
    #[allow(unused)]
    /// the x, y, width, and height are all for the outer rect
    fn new(x: f32, y: f32, width: f32, height: f32, border_size: f32) -> InputBox {
        let outer_rect = Rect::new(x, y, width, height);
        InputBox::from_outer_rect(outer_rect, border_size)
    }

    fn from_outer_rect(outer_rect: Rect, border_size: f32) -> InputBox {
        let inner_rect = inflate_rect(&outer_rect, -border_size);
        InputBox { outer_rect, inner_rect, border_size }
    }

    fn move_to(&mut self, new_topleft: (f32, f32)) {
        self.outer_rect.move_to(new_topleft.into());
        *self = InputBox::from_outer_rect(self.outer_rect, self.border_size);
    }

    fn translate(&mut self, translate: (f32, f32)) {
        translate_rect(&mut self.outer_rect, translate);
        *self = InputBox::from_outer_rect(self.outer_rect, self.border_size);
    }

    fn get_gradient(&self, visualiser: &Visualiser) -> Texture2D {
        get_back_gradient(
            visualiser, 
            self.outer_rect.x as u16, 
            self.outer_rect.w as u16, 
            self.outer_rect.h as u16
        )
    }

    /// get the next vertical input box 
    fn next_vert(&self, vert_padding: f32, down: bool) -> InputBox {
        let mut outer_rect = self.outer_rect.clone();
        outer_rect.move_to(Vec2::new(
            self.outer_rect.x,
            match down {
                true => self.outer_rect.bottom() + vert_padding,
                false => self.outer_rect.y - self.outer_rect.h - vert_padding
            }
        ));
        InputBox::from_outer_rect(outer_rect, self.border_size)
    }

    /// get the next vertial input box by jumping a given amount of space
    fn skip_space_vert(&self, skip_size: f32, down: bool) -> InputBox {
        let mut outer_rect = self.outer_rect.clone();
        outer_rect.move_to(Vec2::new(
            self.outer_rect.x,
            match down {
                true => self.outer_rect.bottom() + skip_size,
                false => self.outer_rect.y - self.outer_rect.h - skip_size
            }
        ));
        InputBox::from_outer_rect(outer_rect, self.border_size)
    }
    
    #[allow(unused)]
    /// get the next horizontal input box
    fn next_hor(&self, hor_padding: f32, right: bool) -> InputBox {
        let mut outer_rect = self.outer_rect.clone();
        outer_rect.move_to(Vec2::new(
            match right {
                true => self.outer_rect.right() + hor_padding,
                false => self.outer_rect.x - self.outer_rect.w - hor_padding
            },
            self.outer_rect.y
        ));
        InputBox::from_outer_rect(outer_rect, self.border_size)
    }
}

/// a box for data input elements to use,
/// that has a border with a gradient
#[derive(Clone)] // avoid as best as possible due to gradient
struct GradientInputBox {
    input_box: InputBox,
    gradient: Texture2D
}
impl GradientInputBox {
    fn new(visualiser: &Visualiser, x: f32, y: f32, width: f32, height: f32, border_size: f32) -> GradientInputBox {
        let outer_rect = Rect::new(x, y, width, height);
        GradientInputBox::from_outer_rect(visualiser, outer_rect, border_size)
    }

    fn from_input_box(visualiser: &Visualiser, input_box: InputBox) -> GradientInputBox {
        // creates a new one instead of cloning as cloning creates memory leaks
        // that are a hassle to fix
        let gradient = input_box.get_gradient(visualiser);
        GradientInputBox { input_box, gradient }
    }

    fn from_outer_rect(visualiser: &Visualiser, outer_rect: Rect, border_size: f32) -> GradientInputBox {
        let input_box = InputBox::from_outer_rect(outer_rect, border_size);
        GradientInputBox::from_input_box(visualiser, input_box)
    }

    /// an input box for most input boxes, with a given y value
    fn default(visualiser: &Visualiser, y: f32) -> GradientInputBox {
        let start_x = screen_width()*DEFAULT_INPUT_BOX_START_X;
        let outer_rect = Rect::new(
            start_x,
            y,
            screen_width()*(MENU_SCREEN_PROPORTION - 2.*MENU_HOR_PADDING) - start_x,
            screen_height()*DEFAULT_INPUT_BOX_HEIGHT
        );
        GradientInputBox::from_outer_rect(visualiser, outer_rect, screen_height()*DEFAULT_INPUT_BOX_BORDER_SIZE)
    }

    /// a default input box at the top of the menu
    fn default_top(visualiser: &Visualiser) -> GradientInputBox {
        GradientInputBox::default(visualiser, navbar_bottom()+screen_height()*MENU_VERT_PADDING)
    }

    fn sealed_clone(&self, visualiser: &Visualiser) -> GradientInputBox {
        GradientInputBox::from_input_box(visualiser, self.input_box.clone())
    }

    fn outer_rect(&self) -> Rect {
        self.input_box.outer_rect
    }

    fn inner_rect(&self) -> Rect {
        self.input_box.inner_rect
    }

    fn border_size(&self) -> f32 {
        self.input_box.border_size
    }

    fn move_to(&mut self, new_topleft: (f32, f32), visualiser: &Visualiser) {
        self.input_box.move_to(new_topleft);
        Texture2D::delete(&self.gradient);
        *self = GradientInputBox::from_input_box(visualiser, self.input_box.clone());
    } 

    fn translate(&mut self, translate: (f32, f32), visualiser: &Visualiser) {
        self.input_box.translate(translate);
        Texture2D::delete(&self.gradient);
        *self = GradientInputBox::from_input_box(visualiser, self.input_box.clone())
    }

    fn next_vert(&self, visualiser: &Visualiser, vert_padding: f32, down: bool) -> GradientInputBox {
        GradientInputBox::from_input_box(visualiser, self.input_box.next_vert(vert_padding, down))
    }

    #[allow(unused)]
    fn skip_space_vert(&self, visualiser: &Visualiser, skip_size: f32, down: bool) -> GradientInputBox {
        GradientInputBox::from_input_box(visualiser, self.input_box.skip_space_vert(skip_size, down))
    }

    fn draw_gradient(&self) {
        draw_texture(self.gradient, self.input_box.outer_rect.x, self.input_box.outer_rect.y, WHITE);
    }

    fn draw(&self, selected_shade: Option<Color>) {
        self.draw_gradient();
        if let Some(shade) = selected_shade {
            draw_rect(&self.input_box.outer_rect, shade);
        }
        draw_rect(&self.input_box.inner_rect, BLACK);
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.gradient);
        self.gradient = self.input_box.get_gradient(visualiser);
    }
}

// TODO: rect alignment
// enum RectAlign {
//     Centre,
//     Left(bool),
//     Right(bool),
//     Top(bool),
//     Bottom(bool)
// }
// impl RectAlign {
//     fn get_topleft(alignment: RectAlign, rect: &Rect, container: &InputBox) -> (f32, f32) {
//         todo!()
//     }
// }

/// alignement of text relative to a text box
/// 
/// the boolean represents if it's inside (true) or outside (false) the input box.
/// inside => positioned relative to the inner rect of the input box
/// outside => positioned relative to the outer rect of the input box
#[derive(Clone, Copy)]
#[allow(unused)]
enum TextAlign {
    Centre,
    Left(bool),
    Right(bool),
    Top(bool),
    Bottom(bool),
    StartX(f32)
}
impl TextAlign {
    fn centre_x(label: &InputLabel, container: &InputBox) -> f32 {
        container.outer_rect.center().x - label.label_dims.width/2.
    }
    
    fn centre_y(label: &InputLabel, container: &InputBox) -> f32 {
        container.outer_rect.center().y + label.label_dims.height/2.
    }

    fn pos_centre(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (TextAlign::centre_x(label, container),TextAlign::centre_y(label, container))
    }

    fn pos_outer_left(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (container.outer_rect.x - label.label_dims.width - label.padding,
         TextAlign::centre_y(label, container))
    }

    fn pos_inner_left(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (container.inner_rect.x + label.padding,
         TextAlign::centre_y(label, container))
    }

    fn pos_outer_right(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (container.outer_rect.right() + label.padding,
         TextAlign::centre_y(label, container))
    }

    fn pos_inner_right(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (container.inner_rect.right() - label.label_dims.width - label.padding,
         TextAlign::centre_y(label, container))
    }

    fn pos_outer_top(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (TextAlign::centre_x(label, container),
         container.outer_rect.y - label.padding)
    }

    fn pos_inner_top(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (TextAlign::centre_x(label, container),
         container.inner_rect.y + label.label_dims.height + label.padding)
    }

    fn pos_outer_bottom(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (TextAlign::centre_x(label, container),
         container.outer_rect.bottom() + label.label_dims.height + label.padding)
    }

    fn pos_inner_bottom(label: &InputLabel, container: &InputBox) -> (f32, f32) {
        (TextAlign::centre_x(label, container),
         container.inner_rect.bottom() - label.padding)
    }

    fn pos_start_x(label: &InputLabel, container: &InputBox, start_x: f32) -> (f32, f32) {
        (start_x, TextAlign::centre_y(label, container))
    }

    fn draw(&self, label: &InputLabel, container: &InputBox) {
        let pos = match self {
            TextAlign::Centre => TextAlign::pos_centre(label, container),
            TextAlign::Left(false) => TextAlign::pos_outer_left(label, container),
            TextAlign::Left(true) => TextAlign::pos_inner_left(label, container),
            TextAlign::Right(false) => TextAlign::pos_outer_right(label, container),
            TextAlign::Right(true) => TextAlign::pos_inner_right(label, container),
            TextAlign::Top(false) => TextAlign::pos_outer_top(label, container),
            TextAlign::Top(true) => TextAlign::pos_inner_top(label, container),
            TextAlign::Bottom(false) => TextAlign::pos_outer_bottom(label, container),
            TextAlign::Bottom(true) => TextAlign::pos_inner_bottom(label, container),
            TextAlign::StartX(start_x) => TextAlign::pos_start_x(label, container, *start_x)
        };
        draw_text_ex(
            &label.text, 
            pos.0, pos.1,
            label.label_params.clone()
        );
    }
}

#[derive(Clone)]
/// a label for an input field
struct InputLabel {
    text: String,
    label_dims: TextDimensions,
    label_params: TextParams,  
    padding: f32,
    alignment: TextAlign,
    /// whether or not the colour is dependent on the gradient
    gradient_colour: bool
}
impl InputLabel {
    fn new(text: &str, font: Font, font_size: f32, color: Color, gradient_colour: bool, padding: f32, alignment: TextAlign) -> InputLabel {
        let params = TextParams { font, font_size: font_size as u16, color, ..Default::default()};
        let measure = measure_text(
            text, 
            Some(font),
            params.font_size,
            params.font_scale
        );

        InputLabel { 
            text: String::from(text), 
            label_dims: measure,
            label_params: params,
            padding,
            alignment,
            gradient_colour
        }
    }

    fn change_text(&mut self, new_text: &str) {
        self.text = new_text.to_owned();
        self.label_dims = measure_text(
            new_text, 
            Some(self.label_params.font), 
            self.label_params.font_size, 
            self.label_params.font_scale
        );
    }

    fn default_input_box_content(font: Font) -> InputLabel {
        InputLabel::new(
            "e", 
            font, 
            screen_width() * DEFAULT_INPUT_BOX_CONTENT_FONT_PROPORTION,
            WHITE,
            false,
            screen_width() * DEFAULT_INPUT_BOX_CONTENT_HOR_PADDING,
            TextAlign::Left(true),
        )
    }

    fn default_input_box_label(visualiser: &Visualiser, font: Font, text: &str, gradient_colour: bool) -> Option<InputLabel> {
        let colour = match gradient_colour {
            true => {
                let gradient = GradientInputBox::default_top(visualiser).gradient;
                let colour = get_brightest_colour(gradient);
                Texture2D::delete(&gradient);
                colour
            },
            false => WHITE
        };
        
        Some(InputLabel::new(
            text, 
            font, 
            screen_width() * DEFAULT_INPUT_BOX_LABEL_FONT_PROPORTION,
            colour,
            gradient_colour,
            screen_width() * DEFAULT_INPUT_BOX_LABEL_PADDING,
            TextAlign::StartX(screen_width() * MENU_HOR_PADDING),
        ))
    }

    fn draw(&self, container: &InputBox) {
        self.alignment.draw(&self, container);
    }

    fn refresh_gradient(&mut self, colour: Color) {
        if self.gradient_colour {
            self.label_params.color = colour;
        }
    }
}

#[derive(Clone)]
struct DataInfo {
    content: String,
    content_dims: TextDimensions,
    content_params: TextParams,
    letters: usize
}
impl DataInfo {
    fn new(data: &str, content_label: &InputLabel) -> DataInfo {
        let content = data.to_owned();
        let letters = content.chars().count();
        DataInfo { 
            content, 
            content_dims: content_label.label_dims, 
            content_params: content_label.label_params, 
            letters,  
        }
    }

    fn remeasure(&mut self) {
        self.content_dims = measure_text(&self.content, Some(self.content_params.font), 
            self.content_params.font_size, 1.0);
        self.letters = self.content.chars().count();
    }
}

#[derive(Clone)]
struct TextBox {
    label: Option<InputLabel>,
    data: String,
    data_info: DataInfo,
    grad_input_box: GradientInputBox,
    content_label: InputLabel,
    selected: bool,
    selected_shade: Color,
    start_pos: usize,
    cursor_pos: usize,
    cursor_visible: bool,
    cursor_blink_timer: f32
}
impl TextBox {
    fn new(
        input_box: GradientInputBox,
        label: Option<InputLabel>,
        content_label: InputLabel,
        default_data: &str
    ) -> TextBox {
        TextBox {
            label, 
            data: String::from(default_data),
            data_info: DataInfo::new(default_data, &content_label),
            grad_input_box: input_box,
            content_label,
            selected: false,
            selected_shade: Color::new(1., 1., 1., 0.5),
            start_pos: 0,
            cursor_pos: 0,
            cursor_visible: true,
            cursor_blink_timer: TEXTBOX_CURSOR_BLINK_TIME
        }
    }

    /// draw and update the text box
    fn update(&mut self, data: String) -> Option<String> {
        if !self.selected {
            if self.data_info.content != data {  
                self.data = data.clone();
                self.data_info = DataInfo::new(&data, &self.content_label);
            }
            self.start_pos = 0;
        }

        self.check_clicked();
        let output = self.keyboard_entry();
        self.update_cursor();

        // width of label never used so this is fine
        self.content_label.text = self.data_info.content[self.start_pos..self.start_pos+self.get_to_use()].to_owned();
        self.draw();

        output
    }

    fn draw(&self) {
        if let Some(label) = &self.label {
            label.draw(&self.grad_input_box.input_box);
        }
        self.grad_input_box.draw(if self.selected {Some(self.selected_shade)} else {None});
       
        self.content_label.draw(&self.grad_input_box.input_box);
        if self.selected && self.cursor_visible {
            draw_rectangle(
                self.get_cursor_x(),
                self.grad_input_box.inner_rect().y + self.grad_input_box.inner_rect().h / 10.,
                2.0,
                self.grad_input_box.inner_rect().h - self.grad_input_box.inner_rect().h / 5.,
                WHITE
            );
        }
    }

    fn get_to_use(&self) -> usize {
        let to_end = self.data_info.letters - self.start_pos;
        for i in 0..=to_end {
            let measure = measure_text(
                &self.data_info.content[self.start_pos..self.start_pos+i], 
                Some(self.data_info.content_params.font), 
                self.data_info.content_params.font_size, 
                1.0
            );
            if measure.width > self.grad_input_box.inner_rect().w - 2.*screen_width()*TEXTBOX_CONTENT_PADDING {
                return i-1;
            }
        }
        to_end
    }

    fn get_cursor_x(&self) -> f32 {
        self.grad_input_box.inner_rect().x + measure_text(
            &self.data_info.content[self.start_pos..self.cursor_pos],
            Some(self.data_info.content_params.font), 
            self.data_info.content_params.font_size, 
            1.0
        ).width + self.content_label.label_dims.height/3.
    }

    fn check_clicked(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) { return }
        if !self.grad_input_box.outer_rect().contains(Vec2::from(mouse_position())) { 
            self.selected = false;
            return;    
        }

        self.selected = true;
        self.reset_cursor_blink();

        // determine cursor position
        let mut found = false;
        for i in self.start_pos+1..self.start_pos+1+self.get_to_use() {
            let measure = measure_text(
                &self.data_info.content[self.start_pos..i], 
                Some(self.data_info.content_params.font), 
                self.data_info.content_params.font_size, 
                1.0
            );
            if mouse_position().0 - self.grad_input_box.inner_rect().x < measure.width {
                self.cursor_pos = i-1;
                found = true;
                break;
            }
        }
        if !found {
            let to_use = self.get_to_use();
            self.cursor_pos = to_use;
        }
    }

    fn keyboard_entry(&mut self) -> Option<String> {
        if !self.selected { return None }

        let key_pressed = get_char_pressed();
        if key_pressed.is_none() && !is_key_pressed(KeyCode::Right) && !is_key_pressed(KeyCode::Left) {
            return None
        }

        let mut output: Option<String> = None;

        if is_key_pressed(KeyCode::Right) && self.cursor_pos < self.data_info.letters {
            self.cursor_pos += 1;
        }
        if is_key_pressed(KeyCode::Left) && self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
        
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Enter) { 
            self.selected = false;
            output = Some(self.data_info.content.to_owned());
        }
        else if let Some(c) = key_pressed {
            if c == '\u{0008}' { // backspace
                if self.cursor_pos > 0  {
                    self.data_info.content.remove(self.cursor_pos-1);
                    self.cursor_pos -= 1;
                }
            } else {
                self.data_info.content.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
            }
            self.data_info.remeasure();
        }    

        if self.cursor_pos < self.start_pos {
            self.start_pos -= 1;
            if self.start_pos > 0 {
                self.start_pos -= 1;
            }
        }
        while self.cursor_pos - self.start_pos > self.get_to_use() {
            self.start_pos += 1;
        }

        self.reset_cursor_blink();

        output
    } 

    fn reset_cursor_blink(&mut self) {
        self.cursor_visible = true;
        self.cursor_blink_timer = TEXTBOX_CURSOR_BLINK_TIME;
    }

    fn update_cursor(&mut self) {
        if !self.selected { return }

        self.cursor_blink_timer -= get_frame_time();
        if self.cursor_blink_timer <= 0.0 {
            self.cursor_visible = !self.cursor_visible;
            self.cursor_blink_timer = TEXTBOX_CURSOR_BLINK_TIME;
        }
    }

    fn translate(&mut self, translate: (f32, f32), visualiser: &Visualiser) {
        self.grad_input_box.translate(translate, visualiser);
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        self.grad_input_box.refresh_gradient(visualiser);
        if let Some(label) = &mut self.label {
            label.refresh_gradient(get_brightest_colour(self.grad_input_box.gradient));
        }
    }
}

trait SliderBar: SliderBarClone {
    fn draw(&self, rect: Rect);
    fn make_gradient(&mut self, _rect: Rect, _left_colour: Color, _right_colour: Color) {}
}

trait SliderBarClone {
    fn clone_box(&self) -> Box<dyn SliderBar>;
}
impl<T> SliderBarClone for T
where
    T: 'static + SliderBar + Clone,
{
    fn clone_box(&self) -> Box<dyn SliderBar> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SliderBar> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct SolidSliderBar {
    colour: Color
}
impl SolidSliderBar {
    fn new(colour: Color) -> SolidSliderBar {
        SolidSliderBar { colour }
    }
}
impl SliderBar for SolidSliderBar {
    fn draw(&self, rect: Rect) {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, self.colour);
        draw_circle(rect.x, rect.center().y, rect.h/2., self.colour);
        draw_circle(rect.right(), rect.center().y, rect.h/2., self.colour);
    }
}

#[derive(Clone)]
struct GradientSliderBar {
    left_colour: Color,
    right_colour: Color,
    gradient_texture: Texture2D
}
impl GradientSliderBar {
    fn empty() -> GradientSliderBar {
        GradientSliderBar { left_colour: BLACK, right_colour: BLACK, gradient_texture: Texture2D::empty() }
    }
}
impl SliderBar for GradientSliderBar {
    fn draw(&self, rect: Rect) {
        draw_circle(rect.x, rect.center().y, rect.h/2., self.left_colour);
        draw_circle(rect.right(), rect.center().y, rect.h/2., self.right_colour);
        // turn the circles into semicircles
        draw_rectangle(rect.x, rect.y, rect.w.round(), rect.h, BLACK);

        draw_texture(self.gradient_texture, rect.x, rect.y, WHITE);
    }

    fn make_gradient(&mut self, rect: Rect, left_colour: Color, right_colour: Color) {
        Texture2D::delete(&self.gradient_texture);

        let mut image = Image::gen_image_color(rect.w as u16, rect.h as u16, BLACK);
        for i in 0..rect.w as u32 {
            let fraction = i as f32 / rect.w;
            for j in 0..rect.h as u32 {
                image.set_pixel(i, j, interpolate_colour(left_colour, right_colour, fraction));
            }
        }

        self.left_colour = left_colour;
        self.right_colour = right_colour;
        self.gradient_texture = Texture2D::from_image(&image);
    }
}

#[derive(Clone)]
struct Slider {
    label: Option<InputLabel>,
    /// percentage is between 0 and 1
    percentage: f32,
    conversion: f32,
    percentage_label_params: Option<TextParams>,
    percentage_text_box: Option<TextBox>,
    rect: Rect,
    inflated_rect: Rect,
    slider_bar: Box<dyn SliderBar>,
    head_radius: f32,
    active: bool,
    inactive_head_colour: Color,
    active_head_colour: Color
}
impl Slider {
    /// if label given, it's placed to the left of the provided x and y values
    /// if percentage label given, it's placed to the right of the slider
    /// 
    /// neither detract/change the dimensions of the slider
    fn new(
        label: Option<InputLabel>, 
        start_percentage: f32, 
        percentage_conversion: f32,
        percentage_label_params: Option<TextParams>,
        percentage_text_box: Option<TextBox>,
        x: f32, y: f32, width: f32, height: f32, 
        slider_bar: Box<dyn SliderBar>,
        head_radius: f32,
        inactive_head_colour: Color,
        active_head_colour: Color
    ) -> Slider {
        let rect = Rect::new(x, y, width, height);
        let inflated_rect = inflate_rect(&rect, head_radius);

        Slider {
            label,
            percentage: start_percentage,
            conversion: percentage_conversion,
            percentage_label_params,
            percentage_text_box,
            rect, inflated_rect,
            slider_bar,
            head_radius, inactive_head_colour, active_head_colour,
            active: false
        }
    }

    /// draw and update the slider
    fn update(&mut self) {
        self.check_user_select();
        self.user_slide();

        self.draw();

        // update text box
        if let Some(Ok(new)) = self.percentage_text_box.as_mut()
                                    .and_then(|tb| tb.update((self.percentage * self.conversion).to_string()))
                                    .and_then(|new_percent| Some(new_percent.parse::<f32>())) {
            if new < 0. || new > self.conversion { return }
            self.percentage = new / self.conversion;
        }
    }

    fn check_user_select(&mut self) {
        if !is_mouse_button_down(MouseButton::Left) { 
            self.active = false;
            return;
        }
        if self.active { return }
        
        let mut inflated_rect = self.rect.clone();
        let extra = (self.head_radius - self.rect.h * 0.5)*1.5;
        inflated_rect.h += 2.0*extra;
        inflated_rect.y -= extra;
        inflated_rect.w += 2.0*extra;
        inflated_rect.x -= extra;

        if inflated_rect.contains(Vec2::from(mouse_position())) && is_mouse_button_pressed(MouseButton::Left) {
            self.active = true;
        }
    }

    fn user_slide(&mut self) {
        if !self.active { return }

        let mouse_x = mouse_position().0;
        self.percentage = (mouse_x-self.rect.x) / self.rect.w;
        self.percentage = self.percentage.clamp(0.0, 1.0);
    }

    fn draw(&self) {
        if let Some(label) = &self.label {
            label.draw(&InputBox::from_outer_rect(self.rect, 0.))
        }

        // draw bar
        self.slider_bar.draw(self.rect);

        // draw head
        if self.active {
            draw_circle(self.rect.x + self.percentage * self.rect.w, 
                        self.rect.center().y, self.head_radius, self.active_head_colour);
        } else{
            draw_circle(self.rect.x + self.percentage * self.rect.w, 
                self.rect.center().y, self.head_radius, self.inactive_head_colour);
            if !self.inflated_rect.contains(Vec2::from(mouse_position())) {
                draw_circle(self.rect.x + self.percentage * self.rect.w, 
                    self.rect.center().y, self.head_radius*0.5, BLACK);
            }
        }

        // draw percentage label
        if let Some(percentage_params) = &self.percentage_label_params {
            let percentage_string = (self.percentage * self.conversion).round().to_string();
            let percentage_string = format!("{}%", &percentage_string);
            let measure = measure_text(
                &percentage_string, 
                Some(percentage_params.font), 
                percentage_params.font_size, percentage_params.font_scale, 
            );
            draw_text_ex(
                &percentage_string,
                self.rect.right() + measure.width * 0.25 + self.rect.h/2.,
                self.rect.bottom() + measure.height/4.,
                *percentage_params
            );
        }
        // draw text box
        if let Some(textbox) = &self.percentage_text_box {
            textbox.draw();
        }
    }

    fn translate(&mut self, translate: (f32, f32)) {
        translate_rect(&mut self.rect, translate);
        translate_rect(&mut self.inflated_rect, translate);
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        if let Some(tb) = &mut self.percentage_text_box {
            tb.refresh_gradient(visualiser);
        }
    }
}

pub trait DropDownType<T> {
    fn get_variants() -> Vec<T>;
    fn get_string(&self) -> String;
}

#[derive(Clone)]
struct DropDown<T: DropDownType<T> + std::cmp::PartialEq + Clone> {
    variants: Vec<T>,
    /// box to contain the currently selected variant
    closed_grad_input_box: GradientInputBox,
    /// box to contain the extra variants
    open_grad_input_box: GradientInputBox,
    label: Option<InputLabel>,
    content_label: InputLabel,
    arrow_image: Texture2D,
    open: bool,
    hovering: bool,
    hover_index: usize
}
impl<T: DropDownType<T> + std::cmp::PartialEq + Clone> DropDown<T> {
    async fn new(
        visualiser: &Visualiser, 
        closed_grad_input_box: GradientInputBox,
        label: Option<InputLabel>,
        content_label: InputLabel
    ) -> DropDown<T> {
        let variants = T::get_variants();

        let open_rect = DropDown::get_open_rect(&closed_grad_input_box, &variants);
        let open_grad_input_box = GradientInputBox::from_outer_rect(visualiser, open_rect, closed_grad_input_box.border_size());

        DropDown {
            variants, 
            closed_grad_input_box, open_grad_input_box,
            label, content_label,
            arrow_image: Texture2D::from_image(&load_image("assets/down.png").await.unwrap()),
            open: false, hovering: false, hover_index: 0
        }
    }

    fn get_open_rect(closed_grad_input_box: &GradientInputBox, variants: &Vec<T>) -> Rect {
        let variant_extension = closed_grad_input_box.outer_rect().h - closed_grad_input_box.border_size();
        let extra_height = (variants.len()-1) as f32 * variant_extension;
        
        Rect::new(
            closed_grad_input_box.outer_rect().x,
            closed_grad_input_box.outer_rect().y + 
                if closed_grad_input_box.outer_rect().bottom() + extra_height <= screen_height() 
                    { variant_extension } else { -extra_height },
            closed_grad_input_box.outer_rect().w,
            extra_height+closed_grad_input_box.border_size()
        )
    }

    /// updates + draws the dropdown 
    /// 
    /// # Returns
    /// None if the value wasn't changed
    /// Some(T) if changed
    fn update(&mut self, current_variant: &T) -> Option<T> {
        self.content_label.change_text(&current_variant.get_string());
        self.draw(current_variant);
        if self.open {
            self.interact_open(current_variant)
        } else {
            self.interact_closed();
            None
        }
    }

    fn interact_closed(&mut self) {
        if !self.closed_grad_input_box.outer_rect().contains(Vec2::from(mouse_position())) {
            self.hovering = false;
            return;
        }

        self.hovering = true;
        if is_mouse_button_pressed(MouseButton::Left) {
            self.open = true;
        }
    }

    fn interact_open(&mut self, current_variant: &T) -> Option<T> {
        let closed_contain = self.closed_grad_input_box.outer_rect().contains(Vec2::from(mouse_position()));
        let open_contain = self.open_grad_input_box.outer_rect().contains(Vec2::from(mouse_position()));
        if !(closed_contain || open_contain) {
            self.hovering = false;
            if is_mouse_button_pressed(MouseButton::Left) {
                self.open = false;
            }
            return None;
        }

        self.hovering = true;
        if closed_contain {
            self.hover_index = 0;
        } else {
            self.hover_index = (((mouse_position().1 - self.open_grad_input_box.outer_rect().y).abs() / self.open_grad_input_box.outer_rect().h) *
                (self.variants.len()-1) as f32).ceil() as usize;
        }

        if !is_mouse_button_pressed(MouseButton::Left) { return None }

        if closed_contain {
            self.open = false;
            return None;
        } 

        let non_current: Vec<&T> = self.variants.iter().filter(|x| **x != *current_variant).collect();
        Some(non_current[self.hover_index-1].clone())
    }

    fn draw(&mut self, current_variant: &T) {
        self.closed_grad_input_box.draw(None);

        self.content_label.draw(&self.closed_grad_input_box.input_box);

        let arrow_size =  self.closed_grad_input_box.inner_rect().h;

        if !self.open { 
            draw_texture_ex(
                self.arrow_image,
                self.closed_grad_input_box.inner_rect().right() - arrow_size,
                self.closed_grad_input_box.inner_rect().y,
                WHITE,
                DrawTextureParams { 
                    dest_size: Some(Vec2::new(arrow_size, arrow_size)), 
                    ..Default::default() 
                }
            );
        }
        if self.hovering && (!self.open || self.hover_index == 0) {
            draw_rect(&self.closed_grad_input_box.outer_rect(), HOVER_WHITE_OVERLAY);
        }

        if let Some(label) = &self.label {
            label.draw(&self.closed_grad_input_box.input_box);
        }

        if !self.open { return }

        // draw open
        draw_texture_ex(
            self.arrow_image,
            self.closed_grad_input_box.inner_rect().right() - arrow_size,
            self.closed_grad_input_box.inner_rect().y,
            WHITE,
            DrawTextureParams { 
                dest_size: Some(Vec2::new(arrow_size, arrow_size)), 
                flip_y: true,
                ..Default::default() 
            }
        );

        self.open_grad_input_box.draw_gradient();
        let non_current: Vec<&T> = self.variants.iter().filter(|x| **x != *current_variant).collect();
        for i in 0..non_current.len() {
            let mut container = self.open_grad_input_box.inner_rect().clone();
            let index_y_add = i as f32 * ( self.closed_grad_input_box.outer_rect().h - self.closed_grad_input_box.border_size());
            container.y += index_y_add;
            container.h = self.closed_grad_input_box.inner_rect().h;
            draw_rect(&container, BLACK);
            
            let mut label = self.content_label.clone();
            label.change_text(&non_current[i].get_string());
            draw_text_ex(
                &label.text, 
                container.x + label.padding,
                container.center().y + label.label_dims.height/2., 
                label.label_params
            );
            if self.hovering && i+1 == self.hover_index {
                draw_rectangle(
                    self.open_grad_input_box.outer_rect().x, 
                    self.open_grad_input_box.outer_rect().y + index_y_add, 
                    self.open_grad_input_box.outer_rect().w, 
                    self.closed_grad_input_box.outer_rect().h, 
                    HOVER_WHITE_OVERLAY
                );
            }
        }
    }

    fn translate(&mut self, translate: (f32, f32), visualiser: &Visualiser) {
        self.closed_grad_input_box.translate(translate, visualiser);
        let open_rect = DropDown::get_open_rect(&self.closed_grad_input_box, &self.variants);
        self.open_grad_input_box.move_to(open_rect.point().into(), visualiser);
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        self.closed_grad_input_box.refresh_gradient(visualiser);
        self.open_grad_input_box.refresh_gradient(visualiser);
        if let Some(label) = &mut self.label {
            label.refresh_gradient(get_brightest_colour(self.closed_grad_input_box.gradient));
        }
    }
}

trait CarouselType {
    fn get_string(&self) -> String;
}

#[derive(Clone)]
struct Carousel {
    /// rect to contain the currently selected variant
    grad_input_box: GradientInputBox,
    content_label: InputLabel,
    left_arrow_rect: Rect,
    right_arrow_rect: Rect,
    right_arrow_image: Texture2D,
    hovering: bool
}
impl Carousel {
    async fn new(
        grad_input_box: GradientInputBox,
        content_label: InputLabel
    ) -> Carousel {
        let inner_rect = grad_input_box.inner_rect();
        Carousel {
            grad_input_box, content_label,
            left_arrow_rect: Rect::new(inner_rect.x, inner_rect.y, inner_rect.h, inner_rect.h),
            right_arrow_rect: Rect::new(inner_rect.right()-inner_rect.h, inner_rect.y, inner_rect.h, inner_rect.h),
            right_arrow_image:  Texture2D::from_image(&load_image("assets/forward.png").await.unwrap()),
            hovering: false
        }
    }

    /// updates + draws the carousel
    ///
    /// # Returns
    /// None if the value wasn't changed
    /// Some(index) if changed (index of the new layer to select)
    fn update<T: CarouselType>(&mut self, variants: Vec<T>, index: usize) -> Option<usize> {
        let allowed_left = index > 0;
        let allowed_right = index < variants.len()-1;

        self.content_label.change_text(&variants[index].get_string());
        self.draw(allowed_left, allowed_right);

        self.mouse_interact(index, allowed_left, allowed_right)
    }

    fn mouse_interact(&mut self, index: usize, allowed_left: bool, allowed_right: bool) -> Option<usize> {
        let left_contains = self.left_arrow_rect.contains(mouse_position().into()) && allowed_left;
        let right_contains = self.right_arrow_rect.contains(mouse_position().into()) && allowed_right;

        if !left_contains && !right_contains {
            self.hovering = false;
            return None;
        }

        self.hovering = true;
        if self.hovering && is_mouse_button_pressed(MouseButton::Left) {
            if left_contains {
                Some(index-1)
            } else {
                Some(index+1)
            }
        } else {
            None
        }
    }

    fn draw(&self, allowed_left: bool, allowed_right: bool) {
        self.grad_input_box.draw(None);

        self.content_label.draw(&self.grad_input_box.input_box);

        if self.hovering {
            draw_rect(&self.grad_input_box.outer_rect(), HOVER_WHITE_OVERLAY);
        }

        if allowed_left {
            draw_texture_ex(self.right_arrow_image, self.left_arrow_rect.x, self.left_arrow_rect.y, WHITE, 
                DrawTextureParams { 
                    flip_x: true, 
                    dest_size: Some((self.left_arrow_rect.w, self.left_arrow_rect.h).into()), 
                    ..Default::default()
                }
            );
        }
        if allowed_right {
            draw_texture_ex(self.right_arrow_image, self.right_arrow_rect.x, self.right_arrow_rect.y, WHITE, 
                DrawTextureParams { 
                    dest_size: Some((self.right_arrow_rect.w, self.right_arrow_rect.h).into()), 
                    ..Default::default()
                }
            );
        }
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        self.grad_input_box.refresh_gradient(visualiser);
    }
}

struct ProgressBar {
    rect: Rect,
    gradient: Image,
    label: Option<InputLabel>,
    bar: Texture2D,
    percent_cache: f32,
    image_cache: Image
}
impl ProgressBar {
    fn new(visualiser: &Visualiser, rect: Rect, label: Option<InputLabel>) -> ProgressBar {
        let gradient = get_back_gradient(visualiser, rect.x as u16, rect.w as u16, rect.h as u16).get_texture_data();
        ProgressBar {
            rect,
            gradient: gradient.clone(),
            label,
            bar: Texture2D::empty(),
            percent_cache: 1., image_cache: gradient
        }
    }

    fn draw_not_active(&mut self) {
        let image = self.gradient.clone();
        Texture2D::delete(&self.bar);
        self.bar = Texture2D::from_image(&image);

        draw_circle(self.rect.x, self.rect.center().y, self.rect.h/2., image.get_pixel(0, 0));
        draw_circle(self.rect.right(), self.rect.center().y, self.rect.h/2., image.get_pixel(image.width as u32-1, 0));
        draw_texture(self.bar, self.rect.x, self.rect.y, WHITE);
    }

    fn draw(&mut self, current_percent: f32, active: bool, draw_label: bool) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, PROGRESS_BAR_COLOUR);
        draw_circle(self.rect.x, self.rect.center().y, self.rect.h/2., PROGRESS_BAR_COLOUR);
        draw_circle(self.rect.right(), self.rect.center().y, self.rect.h/2., PROGRESS_BAR_COLOUR);


        let bar_rect = Rect::new(0., 0., self.rect.w * current_percent, self.rect.h);
        if !active && bar_rect.w as usize > 0 {
            self.draw_not_active();
        } else if bar_rect.w as usize > 0 {
            let image = if current_percent < 1. {
                match current_percent == self.percent_cache {
                    true => self.image_cache.clone(),
                    false => self.gradient.sub_image(bar_rect)
                }
            } else {
                self.gradient.clone()
            };
            Texture2D::delete(&self.bar);
            self.bar = Texture2D::from_image(&image);
            self.percent_cache = current_percent;
            self.image_cache = image.clone();

            draw_circle(self.rect.x, self.rect.center().y, self.rect.h/2., image.get_pixel(0, 0));
            draw_circle(self.rect.x + bar_rect.w, self.rect.center().y, self.rect.h/2., image.get_pixel(image.width as u32-1, 0));
            draw_texture(self.bar, self.rect.x, self.rect.y, WHITE);
        }

        if !draw_label { return }

        if let Some(label) = &self.label {
            let text = format!["{}%", (current_percent*100.).trunc()];
            let measure = measure_text(&text, Some(label.label_params.font), 
                label.label_params.font_size, label.label_params.font_scale);
            draw_text_ex(
                &text,
                self.rect.center().x - measure.width/2.,
                self.rect.y - screen_height()*PROGRERSS_BAR_TEXT_PADDING,
                label.label_params
            );
        }
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        self.gradient = get_back_gradient(visualiser, self.rect.x as u16, 
            self.rect.w as u16, self.rect.h as u16).get_texture_data();
        // make sure it won't use the cache
        self.image_cache = Image::empty();
        self.percent_cache = -1.;
    }
}

trait MenuType {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal;
    fn get_editing(&mut self) -> bool;
    fn open_layer_to_edit(&mut self, _index: usize, _visualiser: &Visualiser) {}
    fn refresh_gradients(&mut self, visualiser: &Visualiser);
}

struct JuliaEditor {
    seed_re: TextBox,
    seed_im: TextBox,
    dims: ScreenDimensions,
    select_rect: Rect,
    select_image: Texture2D,
    request_render: bool,
    rendering_image: Arc<Mutex<Image>>,
    pixel_step: f64,
    progress_tracker: Arc<Mutex<usize>>,
    saved_julia_seed: Complex,
    prev_julia_seed: Complex
}
impl JuliaEditor {
    async fn new(visualiser: &Visualiser, seed_re_input_box: GradientInputBox) -> JuliaEditor {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        let box_vert_padding = screen_height() * DEFAULT_INPUT_BOX_VERT_PADDING;

        let seed_im_input_box = seed_re_input_box.next_vert(visualiser, box_vert_padding, true);

        let select_rect = Rect::new(
            screen_width()*JULIAEDITOR_HOR_PADDING, 
            seed_im_input_box.outer_rect().bottom() + screen_height()*MENU_VERT_PADDING,
            screen_width()*(MENU_SCREEN_PROPORTION - 2.*JULIAEDITOR_HOR_PADDING),
            screen_width()*(MENU_SCREEN_PROPORTION - 2.*JULIAEDITOR_HOR_PADDING),
        );

        JuliaEditor {
            seed_re: TextBox::new(seed_re_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "seed (re)", true),
                InputLabel::default_input_box_content(font), "0"
            ),
            seed_im: TextBox::new(seed_im_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "seed (im)", true),
                InputLabel::default_input_box_content(font), "0"
            ),
            dims: ScreenDimensions::new(select_rect.w as usize, select_rect.h as usize),
            select_rect,
            select_image: Texture2D::empty(),
            request_render: true,
            rendering_image: Arc::new(Mutex::new(Image::empty())),
            pixel_step: 3.5 / select_rect.w as f64,
            progress_tracker: Arc::new(Mutex::new(0)),
            saved_julia_seed: Complex::new(0., 0.),
            prev_julia_seed: Complex::new(0., 0.)
        }
    }

    fn update(&mut self, visualiser: &mut Visualiser) {
        if !visualiser.fractal.is_julia() { return }

        let seed = visualiser.fractal.unwrap_julia_seed();

        let mut changed_seed = false;
        if let Some(Ok(new)) = self.seed_re.update(seed.double.real.to_string())
                                   .and_then(|r| Some(r.parse::<f64>())) {
            seed.set_real(new);
            changed_seed = true;
        }

        if let Some(Ok(new)) = self.seed_im.update(seed.double.im.to_string())
                                   .and_then(|i| Some(i.parse::<f64>())) {
            seed.set_im(new);
            changed_seed = true;
        }

        if changed_seed {
            self.saved_julia_seed = seed.double.clone();
            visualiser.generate_image();
        }

        draw_rect(&self.select_rect, WHITE);
        draw_texture(self.select_image, self.select_rect.x, self.select_rect.y, WHITE);
        let selected_pixel = self.get_selected_pixel().offset(Vec2::new(self.select_rect.x, self.select_rect.y));
        draw_rect(&selected_pixel, RED);
        
        self.process_click(visualiser);

        if self.request_render && self.progress_tracker.lock().unwrap().clone() == 0 {
            self.rendering_image = Arc::new(Mutex::new(Image::gen_image_color(
                self.dims.x as u16, self.dims.y as u16, WHITE
            )));
            self.progress_tracker = Arc::new(Mutex::new(0));
            visualiser.generate_given_image(
                Arc::clone(&self.rendering_image), 
                self.dims.clone(), 
                Some(Fractal::Mandelbrot),
                self.pixel_step, 
                Some(ComplexType::Double(Complex::new(-0.5, 0.))),
                1,
                Arc::clone(&self.progress_tracker),
                false
            );
            self.request_render = false;
        }

        if self.progress_tracker.lock().unwrap().clone() <= self.dims.numpixels() {
            Texture2D::delete(&self.select_image);
            self.select_image = Texture2D::from_image(&self.rendering_image.lock().unwrap().clone());
            self.progress_tracker = Arc::new(Mutex::new(0));
        }
    }

    fn get_selected_pixel(&self) -> Rect {
        Rect::new(
            (((self.saved_julia_seed.real+0.5) / self.pixel_step + self.dims.x as f64 / 2.) as f32).clamp(0., self.select_rect.w),
            ((self.saved_julia_seed.im / self.pixel_step + self.dims.y as f64 / 2.) as f32).clamp(0., self.select_rect.h),
            2., 2.
        )
    }

    fn process_click(&mut self, visualiser: &mut Visualiser) {
        let seed = visualiser.fractal.unwrap_julia_seed();

        self.prev_julia_seed = seed.double.clone();

        if !self.select_rect.contains(mouse_position().into()) {
            if seed.double != self.saved_julia_seed {
                *seed = JuliaSeed::new(self.saved_julia_seed.real, self.saved_julia_seed.im);
            }
            return;
        }

        *seed = JuliaSeed::new(
            ( mouse_position().0 - self.select_rect.center().x ) as f64 * self.pixel_step - 0.5,
            ( mouse_position().1 - self.select_rect.center().y ) as f64 * self.pixel_step
        );
        if seed.double != self.prev_julia_seed {
            visualiser.generate_image();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.saved_julia_seed = visualiser.fractal.unwrap_julia_seed().double;
        }
    }

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        self.seed_re.refresh_gradient(visualiser);
        self.seed_im.refresh_gradient(visualiser);
        self.request_render = true;
    }

    fn get_editing(&self) -> bool {
        self.seed_re.selected || self.seed_im.selected
    }
}

struct GeneralMenu {
    center_re: TextBox,
    center_im: TextBox,
    magnification: TextBox,
    max_iterations: TextBox,
    bailout: TextBox,
    julia_editor: JuliaEditor,
    progress_bar: ProgressBar
}
impl GeneralMenu {
    async fn new(visualiser: &Visualiser) -> GeneralMenu {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        let box_vert_padding = screen_height() * DEFAULT_INPUT_BOX_VERT_PADDING;

        let center_re_input_box = GradientInputBox::default_top(visualiser);
        let center_im_input_box = center_re_input_box.next_vert(visualiser, box_vert_padding, true);
        let magnification_input_box = center_im_input_box.next_vert(visualiser, box_vert_padding, true);
        let max_iter_input_box = magnification_input_box.next_vert(visualiser, box_vert_padding, true);
        let bailout_input_box = max_iter_input_box.next_vert(visualiser, box_vert_padding, true);

        let seed_re_input_box = bailout_input_box.next_vert(visualiser, box_vert_padding, true);

        let input_boxes = vec![
            center_re_input_box, center_im_input_box, magnification_input_box, max_iter_input_box, bailout_input_box
        ];

        GeneralMenu { 
            center_re: GeneralMenu::create_textbox(visualiser, &input_boxes, font, 0),
            center_im: GeneralMenu::create_textbox(visualiser, &input_boxes, font, 1),
            magnification: GeneralMenu::create_textbox(visualiser, &input_boxes, font, 2),
            max_iterations: GeneralMenu::create_textbox(visualiser, &input_boxes, font, 3),
            bailout: GeneralMenu::create_textbox(visualiser, &input_boxes, font, 4),
            julia_editor: JuliaEditor::new(visualiser, seed_re_input_box).await,
            progress_bar: ProgressBar::new(
                visualiser, 
                Rect::new(
                    screen_width()*(MENU_SCREEN_PROPORTION/2.-PROGRESS_BAR_WIDTH/2.),
                    screen_height()*(1. - PROGRESS_BAR_VERT_PADDING),
                    screen_width()*PROGRESS_BAR_WIDTH,
                    screen_height()*PROGRESS_BAR_HEIGHT
                ),
                Some(InputLabel::new(
                    "0%", 
                    font, 
                    screen_width()*PROGRESS_BAR_FONT_PROPORTION, 
                    WHITE, false,
                    0., TextAlign::Centre // these don't matter for progress bars (yet)
                ))
            ),
        }
    }

    fn create_textbox(
        visualiser: &Visualiser,
        input_boxes: &Vec<GradientInputBox>, 
        font: Font, 
        i: usize
    ) -> TextBox {
        TextBox::new(
            input_boxes[i].clone(),
            InputLabel::default_input_box_label(visualiser, font, 
                ["center (re)", "center (im)", "magnification", "max iterations", "bailout"][i], 
                true),
            InputLabel::default_input_box_content(font),
            ""
        )
    }

    fn all_text_boxes(&mut self) -> [&mut TextBox; 5] {
        [
            &mut self.center_re, &mut self.center_im, &mut self.magnification, &mut self.max_iterations, &mut self.bailout
        ]
    }

    fn get_data(visualiser: &Visualiser, i: usize) -> String {
        if i == 0 {
            visualiser.center.real_string()
        } else if i == 1 {
            visualiser.center.im_string() 
        } else if i == 2 {
            visualiser.get_magnification().to_string()
        } else if i == 3 {
            (visualiser.max_iterations as u32).to_string()
        } else {
            visualiser.bailout2.sqrt().to_string()
        }
    }

    fn update_data(visualiser: &mut Visualiser, i: usize, new: String) {
        if i == 0 {
            visualiser.center.update_real_from_string(new);
        } else if i == 1 {
            visualiser.center.update_im_from_string(new);
        } else if i == 2 {
            if let Ok(new) = new.parse::<f64>() {
                if new <= 0.0 { return };
                visualiser.set_pixel_step(0.005/new);
            }
        } else if i == 3 {
            if let Ok(new) = new.parse::<u32>() {
                if new < 1 { return };
                visualiser.max_iterations = new as f32;
            }
        } else {
            if let Ok(new) = new.parse::<f64>() {
                if new <= 0.0 { return };
                visualiser.bailout2 = new.powi(2);
            }
        }
    }
}
impl MenuType for GeneralMenu {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        for (i, text_box) in self.all_text_boxes().iter_mut().enumerate() {
            let output = text_box.update(GeneralMenu::get_data(visualiser, i));
            if let Some(new) = output {
                GeneralMenu::update_data(visualiser, i, new);
                visualiser.generate_image();
            }
        }

        self.julia_editor.update(visualiser);

        if visualiser.rendering {
            self.progress_bar.draw(
                visualiser.progress_tracker.lock().unwrap().clone() as f32 / 
                    visualiser.current_dimensions.numpixels() as f32,
                true, true
            );
        } else {
            self.progress_bar.draw(1., false, true);
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for text_box in self.all_text_boxes().iter() {
            if text_box.selected { return true }
        }
        self.julia_editor.get_editing()
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        for text_box in self.all_text_boxes().iter_mut() {
            text_box.refresh_gradient(visualiser);
        }
        self.julia_editor.refresh_gradient(visualiser);
        self.progress_bar.refresh_gradient(visualiser);
    }
}

fn generate_strength_slider(strength_slider_text_params: TextParams, inner_rect: Rect, layer_strength: f32) -> Slider {
    let strength_measure = measure_text(
        "strength", 
        Some(strength_slider_text_params.font),
        strength_slider_text_params.font_size, 
        strength_slider_text_params.font_scale
    );
    // the top of the 'box' the slider is in
    let strength_boxtop = inner_rect.y + screen_height()*(LAYERMANAGER_INNER_TOP_PADDING+LAYERMANAGER_PALETTE_HEIGHT_PROPORTION);
    let strength_mid = strength_boxtop + (inner_rect.bottom() - strength_boxtop)*0.5;
    let strength_height = screen_height() * LAYERMANAGER_STRENGTH_SLIDER_HEIGHT;
    let strength_x = inner_rect.x + 
        screen_width()*(LAYERMANAGER_INNER_LEFT_PADDING+LAYERMANAGER_PALETTE_RIGHT_PADDING) + 
        screen_height()*LAYERMANAGER_PALETTE_HEIGHT_PROPORTION;
    let measure = measure_text(
        &"100%", 
        Some(strength_slider_text_params.font), 
        strength_slider_text_params.font_size, strength_slider_text_params.font_scale, 
    );
    let strength_width = inner_rect.w * LAYERMANAGER_HALF_END_PROPORION - strength_x - measure.width*1.25;
    // let strength_width = inner_rect.w - strength_x - measure.width*1.25;

    let mut strength_label_params = strength_slider_text_params.clone();
    strength_label_params.color = WHITE;

    Slider::new(
        Some(InputLabel { 
            text: String::from("strength"),
            label_dims: TextDimensions { 
                width: strength_measure.width, 
                height: strength_measure.height, offset_y: 0.0 },
            label_params: strength_label_params,
            padding: screen_width() * DEFAULT_INPUT_BOX_LABEL_PADDING,
            alignment: TextAlign::StartX(
                screen_width()*(LAYERMANAGER_LEFT_PADDING+LAYERMANAGER_INNER_LEFT_PADDING)+
                screen_height()*LAYERMANAGER_BORDER_PROPORTION
            ),
            gradient_colour: false
        }),
        layer_strength,
        100.,
        Some(strength_slider_text_params),
        None,
        strength_x,
        strength_mid - strength_height*0.5,
        strength_width, strength_height, 
        Box::new(SolidSliderBar::new(LAYERMANAGER_LAYER_TYPE_COLOUR)),
        strength_height * 1.5,
        LAYERMANAGER_LAYER_TYPE_COLOUR, WHITE
    )
}

#[derive(Clone)]
struct LayerManager {
    border_back: Texture2D,
    outer_rect: Rect,
    inner_rect: Rect,
    palette_button: Button,
    name: TextBox,
    layer_type_text_params: TextParams,
    edit_button: Button,
    strength_slider: Slider,
    layer_range_dropdown: DropDown<LayerRange>,
    delete_button: Button,
    drag_rect: Rect,
    translated: bool,
    hovering: bool,
    dragging: bool,
    /// the manager has been released from a drag
    released: bool
}
impl LayerManager {
    async fn new(
        visualiser: &Visualiser, 
        layer: &Layer, 
        layer_num: usize,
        name_text_params: TextParams, 
        layer_type_text_params: TextParams,
        strength_slider_text_params: TextParams
    ) -> LayerManager {
        let border_width = screen_height()*LAYERMANAGER_BORDER_PROPORTION;
        let outer_rect = Rect::new(
            screen_width()*LAYERMANAGER_LEFT_PADDING,
            screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - layer_num as f32 * 
                (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING)),
            screen_width()*(MENU_SCREEN_PROPORTION-LAYERMANAGER_LEFT_PADDING-LAYERMANAGER_RIGHT_PADDING),
            screen_height()*LAYERMANAGER_HEIGHT
        );
        let inner_rect = Rect::new(
            border_width,
            border_width,
            outer_rect.w - 2.0*border_width,
            outer_rect.h - 2.0*border_width
        );

        let name_textbox_width = inner_rect.w * LAYERMANAGER_HALF_END_PROPORION - 
            screen_width()*(LAYERMANAGER_INNER_LEFT_PADDING+LAYERMANAGER_PALETTE_RIGHT_PADDING) - 
            screen_height()*LAYERMANAGER_PALETTE_HEIGHT_PROPORTION;
        let name_textbox_start_x = screen_width()*(LAYERMANAGER_INNER_LEFT_PADDING+LAYERMANAGER_PALETTE_RIGHT_PADDING) +
            screen_height()*LAYERMANAGER_PALETTE_HEIGHT_PROPORTION;
        let name_textbox_height = screen_height()*LAYERMANAGER_NAME_TEXTBOX_HEIGHT;

        let palette_size = screen_height()*LAYERMANAGER_PALETTE_HEIGHT_PROPORTION;
        let pallete_button_rect = Rect::new(
            screen_width()*LAYERMANAGER_INNER_LEFT_PADDING, 
            screen_height()*LAYERMANAGER_INNER_TOP_PADDING,
            palette_size, palette_size
        );
        let edit_button_x = inner_rect.w*LAYERMANAGER_HALF_END_PROPORION + 
            screen_width()*LAYERMANAGER_INNER_LEFT_PADDING;
        let edit_button_border = screen_height()*LAYERMANAGER_EDIT_BUTTON_BORDER_HEIGHT;
        let mut edit_button_rect = pallete_button_rect.clone();
        edit_button_rect.x = edit_button_x;

        let layer_range_dropdown_y = palette_size + 2.*screen_height()*LAYERMANAGER_INNER_TOP_PADDING;
        // let layer_range_dropdown_y = inner_rect.y + screen_height()*LAYERMANAGER_INNER_TOP_PADDING;

        let drag_x = edit_button_x + palette_size + screen_width()*LAYERMANAGER_LEFT_PADDING;

        let delete_button_size = screen_height()*LAYERMANAGER_DELETE_BUTTON_SIZE;
        let delete_button_x_offset = inner_rect.w-delete_button_size-screen_height()*LAYERMANAGER_INNER_LEFT_PADDING;
        let delete_button_rect = Rect::new(
            delete_button_x_offset, 
            screen_height()*LAYERMANAGER_INNER_TOP_PADDING,
            delete_button_size, delete_button_size
        );

        LayerManager { 
            border_back: get_back_gradient(
                visualiser, 
                outer_rect.x as u16, 
                outer_rect.w as u16, 
                outer_rect.h as u16
            ), 
            outer_rect, inner_rect,
            palette_button: Button::from_rect(
                &pallete_button_rect,
                vec![Box::new(ButtonGradientElement::full_back(
                    visualiser, Some(layer_num), &pallete_button_rect, WHITE, 0
                ))],
                vec![
                    Box::new(ButtonColourElement::full_button(&pallete_button_rect, Color::new(0., 0., 0., 0.5), 1)),
                    Box::new(ButtonImageElement::new(
                        load_image("assets/wrench.png").await.unwrap(), 0.7,
                        DrawTextureParams { dest_size: Some(pallete_button_rect.size()), ..Default::default() },
                        (0., 0.), 2
                    ))
                ],
                vec![]
            ),
            name: TextBox::new(
                GradientInputBox::new(
                    visualiser, 
                    name_textbox_start_x, 
                    screen_height()*LAYERMANAGER_INNER_TOP_PADDING,
                    name_textbox_width,
                    name_textbox_height,
                    screen_height() * DEFAULT_INPUT_BOX_BORDER_SIZE
                ),
                None,
                InputLabel::new(
                    &layer.name, 
                    name_text_params.font, 
                    name_text_params.font_size as f32,
                    WHITE, false,
                    screen_width() * DEFAULT_INPUT_BOX_CONTENT_HOR_PADDING,
                    TextAlign::Left(true)
                ), ""
            ),
            layer_type_text_params,
            edit_button: Button::gradient_border_and_image(
                visualiser, &edit_button_rect, edit_button_border, 
                load_image("assets/wrench.png").await.unwrap(), DrawTextureParams::default(),
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            strength_slider: generate_strength_slider(strength_slider_text_params, inner_rect, layer.strength),
            layer_range_dropdown: DropDown::new(
                visualiser,
                GradientInputBox::new(
                    visualiser,
                    edit_button_x, layer_range_dropdown_y,
                    palette_size, palette_size*0.4,
                    edit_button_border
                ),
                None,
                InputLabel::new(
                    "", name_text_params.font, screen_width()*LAYERMANAGER_LAYER_RANGE_FONT_PROPORTION, 
                    WHITE, false,
                    screen_width() * LAYERMANAGER_LAYER_RANGE_CONTENT_HOR_PADDING,
                TextAlign::Left(true)
                )
            ).await,
            delete_button: Button::gradient_border_and_image(
                visualiser, &delete_button_rect, edit_button_border, 
                load_image("assets/cross.png").await.unwrap(), DrawTextureParams::default(),
                HOVER_WHITE_OVERLAY, HOVER_RED_OVERLAY
            ),
            drag_rect: Rect::new(
                drag_x,
                0.,
                inner_rect.right() - drag_x,
                inner_rect.h
            ),
            translated: false,
            hovering: false,
            dragging: false,
            released: false
        }
    }

    /// creates a new LayerManager using another one to avoid being async
    fn new_copy(visualiser: &Visualiser, copy: &LayerManager, scroll: f32) -> LayerManager {
        let mut palette_button = copy.palette_button.clone();
        // this still leaks a small amount of memory as you can't drop the gradient in 
        // ButtonGradientElement, but can be fixed once async trait functions are released
        palette_button.back_elements[0] = Box::new(ButtonGradientElement::new(
            visualiser,
            Some(visualiser.layers.layers.len()-1),
            (palette_button.rect.x, palette_button.rect.y),
            (palette_button.rect.w, palette_button.rect.h),
            (0., 0.), WHITE, 0 
        ));

        let mut strength_slider = copy.strength_slider.clone();
        strength_slider.percentage = 0.;

        let mut manager = LayerManager { 
            border_back: get_back_gradient(visualiser, copy.outer_rect.x as u16, copy.outer_rect.w as u16, copy.outer_rect.h as u16), 
            outer_rect: copy.outer_rect,
            inner_rect: copy.inner_rect, 
            palette_button, 
            name: copy.name.clone(), 
            layer_type_text_params: copy.layer_type_text_params, 
            edit_button: copy.edit_button.clone(), 
            strength_slider, 
            layer_range_dropdown: copy.layer_range_dropdown.clone(), 
            delete_button: copy.delete_button.clone(),
            drag_rect: copy.drag_rect, 
            translated: false, 
            hovering: false, 
            dragging: false, 
            released: false 
        };

        manager.undo_translation(visualiser);

        let outer_rect = Rect::new(
            screen_width()*LAYERMANAGER_LEFT_PADDING,
            screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - (visualiser.layers.layers.len()-1) as f32 * 
                (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING)) + scroll,
            screen_width()*(MENU_SCREEN_PROPORTION-LAYERMANAGER_LEFT_PADDING-LAYERMANAGER_RIGHT_PADDING),
            screen_height()*LAYERMANAGER_HEIGHT
        );
        manager.outer_rect = outer_rect;
        manager
    }

    fn translate(&mut self, new_outer_rect_pos: (f32, f32), visualiser: &Visualiser) {
        if self.translated {
            if self.outer_rect.x == new_outer_rect_pos.0 && self.outer_rect.y == new_outer_rect_pos.1 {
                return;
            }
            self.undo_translation(visualiser);
        }

        self.outer_rect.x = new_outer_rect_pos.0;
        self.outer_rect.y = new_outer_rect_pos.1;
        self.perform_translation(visualiser);

        self.translated = true;
    }

    fn translate_items(&mut self, translate: (f32, f32), visualiser: &Visualiser) {
        self.palette_button.translate(translate);
        self.name.translate(translate, visualiser);
        self.edit_button.translate(translate);
        self.strength_slider.translate(translate);
        self.layer_range_dropdown.translate(translate, visualiser);
        self.delete_button.translate(translate);
        translate_rect(&mut self.drag_rect, translate);
    }

    fn perform_translation(&mut self, visualiser: &Visualiser) {
        let translate = (self.outer_rect.x, self.outer_rect.y);
        translate_rect(&mut self.inner_rect, translate);

        let translate = (self.inner_rect.x, self.inner_rect.y);
        self.translate_items(translate, visualiser);
    }

    fn undo_translation(&mut self, visualiser: &Visualiser) {
        let translate = (-self.outer_rect.x, -self.outer_rect.y);
        let old_inner = self.inner_rect.clone();
        translate_rect(&mut self.inner_rect, translate);

        let translate = (-old_inner.x, -old_inner.y);
        self.translate_items(translate, visualiser);
        
        self.translated = false;
    }

    fn update(&mut self, visualiser: &mut Visualiser, layer_i: usize, update_edit_button: bool) -> bool {
        if !self.translated {
            self.translate((self.outer_rect.x, self.outer_rect.y), visualiser);
        }

        let layer = &mut visualiser.layers.layers[layer_i];

        self.draw(layer);

        let mut changed: bool = false;

        self.palette_button.update();
        if update_edit_button {
            self.edit_button.update();
        } else {
            self.edit_button.draw();
        }

        if let Some(new_name) = self.name.update(layer.name.clone()) {
            layer.name = new_name;
        }

        if let Some(new) = self.layer_range_dropdown.update(&layer.layer_range) {
            layer.change_range(new);
            changed = true;
        }

        self.strength_slider.update();
        if layer.change_strength(self.strength_slider.percentage) {
            changed = true;
        }
        
        self.mouse_interact();
        // draw overlay over whole manager
        if self.delete_button.holding {
            draw_rectangle(self.outer_rect.x, self.outer_rect.y, self.outer_rect.w, self.outer_rect.h, HOVER_RED_OVERLAY);
        } else if self.dragging {
            draw_rectangle(self.outer_rect.x, self.outer_rect.y, self.outer_rect.w, self.outer_rect.h, HOVER_WHITE_OVERLAY);
        }
        
        if layer.can_delete() && !self.dragging {
            self.delete_button.update();
        }

        changed
    }

    fn draw(&mut self, layer: &mut Layer) {
        // draw background and border
        draw_texture(self.border_back, self.outer_rect.x, self.outer_rect.y, WHITE);
        draw_rectangle(self.inner_rect.x, self.inner_rect.y, self.inner_rect.w, self.inner_rect.h, BLACK);

        // draw layer type
        let measure = measure_text(&layer.layer_type.get_string(), 
            Some(self.layer_type_text_params.font), self.layer_type_text_params.font_size, 1.0);
        draw_text_ex(
            &layer.layer_type.get_string(), 
            self.inner_rect.x + screen_width()*(LAYERMANAGER_INNER_LEFT_PADDING+LAYERMANAGER_PALETTE_RIGHT_PADDING) + 
                screen_height()*(LAYERMANAGER_PALETTE_HEIGHT_PROPORTION),
            self.inner_rect.y + screen_height()*(2.0*LAYERMANAGER_INNER_TOP_PADDING+LAYERMANAGER_NAME_TEXTBOX_HEIGHT) + 
                measure.height*0.7,
            self.layer_type_text_params
        );

        // draw drag circles
        for i in 0..3 {
            draw_circle(
                self.drag_rect.center().x, 
                self.drag_rect.center().y + (i as f32 - 1.) * self.drag_rect.h/9.,
                if self.hovering || self.dragging {self.drag_rect.h/22.} else {self.drag_rect.h/25.},
                if self.hovering || self.dragging {WHITE} else {LAYERMANAGER_LAYER_TYPE_COLOUR}
            );
        }
    }

    fn mouse_interact(&mut self) {
        if self.delete_button.hovering { 
            self.hovering = false;
            return;
        }

        self.hovering = self.drag_rect.contains(Vec2::from(mouse_position()));

        if self.dragging && !is_mouse_button_down(MouseButton::Left) {
            self.released = true;
        }

        if !self.dragging && self.hovering && is_mouse_button_pressed(MouseButton::Left) {
            // start drag
            self.dragging = true;
        } else if self.dragging && is_mouse_button_down(MouseButton::Left) {
            // continue drag
            self.dragging = true;
        } else {
            self.dragging = false;
        }
    }   

    fn refresh_gradient(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.border_back);
        self.border_back = get_back_gradient(visualiser, self.outer_rect.x as u16, self.outer_rect.w as u16, self.outer_rect.h as u16);
    
        self.palette_button.refresh_gradient(visualiser);
        self.edit_button.refresh_gradient(visualiser);
        self.layer_range_dropdown.refresh_gradient(visualiser);
        self.delete_button.refresh_gradient(visualiser);

        self.name.refresh_gradient(visualiser);
    }
}

struct LayersMenu {
    layer_managers: Vec<LayerManager>,
    add_button: Button,
    scroll: f32,
    /// if the scroll bar is being dragged
    drag_scroll: bool,
    /// the original scroll at the start of a drag
    orig_drag_scroll: f32,
    /// the y position of the mouse when a drag was started
    bar_drag_start: f32
}
impl LayersMenu {
    async fn new(visualiser: &Visualiser) -> LayersMenu {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();

        let name_text_params = TextParams { font, 
            font_size: (screen_width()*LAYERMANAGER_NAME_TEXT_FONT_PROPORTION) as u16, color: WHITE, ..Default::default()};
        let layer_type_text_params = TextParams { font, 
            font_size: (screen_width()*LAYERMANAGER_LAYER_TYPE_FONT_PROPORTION) as u16, color: LAYERMANAGER_LAYER_TYPE_COLOUR, 
            ..Default::default()};
        let strength_slider_text_params = TextParams { font,
            font_size: (screen_width()*LAYERMANAGER_STRENGTH_TEXT_FONT_PROPORTION) as u16, color: LAYERMANAGER_LAYER_TYPE_COLOUR,
            ..Default::default()};

        let mut layer_managers = Vec::new();
        for (i, layer) in visualiser.layers.layers.iter().enumerate() {
            layer_managers.push(
                LayerManager::new(visualiser, layer, i, name_text_params, layer_type_text_params, strength_slider_text_params).await
            );
        }

        let add_rect = layer_managers[0].outer_rect.clone();
        let inner_rect = layer_managers[0].inner_rect.clone();
        let cutout_width = add_rect.w/7.;

        let mut add_button = Button::new(
            add_rect.size().into(),
            (0., 0.),
            vec![
                Box::new(ButtonGradientElement::new(
                    visualiser,
                    None,
                    (0., 0.),
                    add_rect.size().into(),
                    (0., 0.), WHITE, 0
                )),
                Box::new(ButtonColourElement::new(
                    BLACK,
                    inner_rect.size().into(),
                    (screen_height()*LAYERMANAGER_BORDER_PROPORTION, screen_height()*LAYERMANAGER_BORDER_PROPORTION),
                    1
                )),
                Box::new(ButtonColourElement::new(
                    BLACK,
                    (cutout_width, add_rect.h),
                    (cutout_width, 0.),
                    2
                )),
                Box::new(ButtonColourElement::new(
                    BLACK,
                    (cutout_width, add_rect.h),
                    (3. * cutout_width, 0.),
                    3
                )),
                Box::new(ButtonColourElement::new(
                    BLACK,
                    (cutout_width, add_rect.h),
                    (5. * cutout_width, 0.),
                    4
                )),
                Box::new(ButtonColourElement::new(
                    BLACK,
                    (add_rect.w, add_rect.h/3.),
                    (0., add_rect.h/3.),
                    5
                )),
                Box::new(ButtonImageElement::new(
                    load_image("assets/plus.png").await.unwrap(),
                    1.,
                    DrawTextureParams { dest_size: Some((cutout_width, cutout_width).into()), ..Default::default() },
                    (3. * cutout_width, (add_rect.h - cutout_width)/2.),
                    6
                ))
            ],
            vec![Box::new(ButtonColourElement::new(
                HOVER_WHITE_OVERLAY, add_rect.size().into(), (0.,0.), 7
            ))],
            vec![]
        );
        add_button.translate(LayersMenu::get_add_topleft(layer_managers.len()));

        LayersMenu { 
            layer_managers,
            add_button,
            scroll: 0.0,
            drag_scroll: false,
            orig_drag_scroll: 0.0,
            bar_drag_start: 0.0
        }
    }

    fn update_add_button_pos(&mut self, layers_num: usize, prev_num_offset: isize) {
        self.add_button.translate((
            LayersMenu::get_add_topleft((layers_num as isize+prev_num_offset) as usize).0 * -1.,
            LayersMenu::get_add_topleft((layers_num as isize+prev_num_offset) as usize).1 * -1.   
        ));
        self.add_button.translate(LayersMenu::get_add_topleft(layers_num));
    }

    fn update_manager_positions(&mut self, visualiser: &Visualiser) {
        for (i, layer_manager) in self.layer_managers.iter_mut().enumerate() {
            layer_manager.translate((
                screen_width()*LAYERMANAGER_LEFT_PADDING,
                screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - i as f32 * 
                    (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING)) + self.scroll
            ), visualiser);
            for element in layer_manager.palette_button.back_elements.iter_mut() {
                element.gradient_change_layer_i(i);
            }
        }
    }
    
    fn add_layer(&mut self, visualiser: &mut Visualiser) {
        let mut new_layer =  Layer::default();
        new_layer.palette.generate_palette(visualiser.max_iterations);
        visualiser.layers.add_layer(&new_layer);

        self.layer_managers.push(LayerManager::new_copy(&visualiser, &self.layer_managers[0], self.scroll));

        self.update_add_button_pos(visualiser.layers.layers.len(), -1);
    }

    fn delete_layer(&mut self, visualiser: &mut Visualiser, i: usize) {
        visualiser.layers.delete_layer(i);
        let deleted = self.layer_managers.remove(i);
        drop(deleted);

        self.update_manager_positions(&visualiser);
        self.update_add_button_pos(visualiser.layers.layers.len(), 1);
        self.update_scroll(&visualiser, true);
    }

    /// gets the index of the layer manager the mouse is dragging over
    fn get_mouse_drag_i(&self) -> usize {
        let mut i = self.layer_managers.len()-1;
        while self.layer_managers[i].outer_rect.center().y < mouse_position().1 && i > 0 {
            i -= 1;
        }
        i += 1;
        if mouse_position().1 > self.layer_managers[0].outer_rect.center().y {
            i = 0;
        }

        i
    }

    fn draw_drag(&self, drag_i: usize, visualiser: &Visualiser) {
        if mouse_position().0 > screen_width()*MENU_SCREEN_PROPORTION { return }

        let i = self.get_mouse_drag_i();

        if i == drag_i+1 || i == drag_i { return }
        if !visualiser.layers.layers[drag_i].position_allowed(i) {
            return
        }

        draw_rectangle(
            screen_width()*LAYERMANAGER_LEFT_PADDING, 
            screen_height() - screen_height()*LAYERMANAGER_BOTTOM_PADDING - i as f32 * (self.layer_managers[0].outer_rect.h + screen_height()*LAYERMANAGER_TOP_PADDING) + self.scroll, 
            screen_width()*MENU_SCREEN_PROPORTION - screen_width()*LAYERMANAGER_LEFT_PADDING, 
            screen_height()*LAYERMANAGER_TOP_PADDING, 
            WHITE
        );
    }

    fn release(&mut self, release_i: usize, visualiser: &mut Visualiser) {
        let mut dest_i = self.get_mouse_drag_i();

        if dest_i == release_i { return }

        if !visualiser.layers.layers[release_i].position_allowed(dest_i) {
            return
        }

        if dest_i > release_i {
            dest_i -= 1;
        }

        // rearrange visulaiser layers
        //        r     d  
        // [0, 1, 2, 3, 4, 5]
        // [0, 1, 3, 4, 5]
        // [0, 1, 3, 2, 4, 5]
        visualiser.layers.reorder_layer(release_i, dest_i);

        // rearrange managers
        //     r     d              d        r
        // [0, 1, 2, 3, 4, 5]    | [0, 1, 2, 3, 4, 5]
        // [0, 3, 2, 1, 4, 5]    | [3, 1, 2, 0, 4, 5]
        // [0, 3, 1, 2, 4, 4]    | [3, 0, 2, 1, 4, 5]
        //                         [3, 0, 1, 2, 4, 5]
        //
        // [0, 2, 3, 1, 4, 5]    | [3, 0, 1, 2, 3, 4]
        let mut ptr = dest_i;
        while ptr != release_i {
            self.layer_managers.swap(release_i, ptr);
            if release_i < dest_i {
                ptr -= 1
            } else {
                ptr += 1;
            }
        }
        self.update_manager_positions(&visualiser);
    }

    fn get_add_topleft(layers_num: usize) -> (f32, f32) {
        (
            screen_width()*LAYERMANAGER_LEFT_PADDING,
            screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - layers_num as f32 * 
                    (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING))
        )
    }

    /// draw + update scroll
    fn update_scroll(&mut self, visualiser: &Visualiser, just_deleted: bool) {
        let top_y = screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) +
            screen_width()*STATE_TEXT_FONT_PROPORTION;
        let menu_height = screen_height()-top_y;
        let total_height = self.layer_managers[0].outer_rect.bottom()-self.add_button.rect.y;

        if self.add_button.rect.y > top_y  && self.scroll == 0. {
            return;
        }

        let bar_height = menu_height.powi(2) / total_height;

        let bar = Rect::new(
            screen_width()*(MENU_SCREEN_PROPORTION-LAYERMANAGER_RIGHT_PADDING),
            top_y + menu_height - bar_height - (menu_height-bar_height) * (self.scroll / (total_height - menu_height)),
            screen_width()*LAYERMANAGER_RIGHT_PADDING,
            bar_height,
        );

        if bar.contains(mouse_position().into()) && is_mouse_button_pressed(MouseButton::Left) {
            self.bar_drag_start = mouse_position().1;
            self.drag_scroll = true;
            self.orig_drag_scroll = self.scroll;
        } 
        if !is_mouse_button_down(MouseButton::Left) {
            self.drag_scroll = false;
        }

        if mouse_position().0 <= screen_width()*MENU_SCREEN_PROPORTION || self.drag_scroll {
            draw_rectangle(bar.x, bar.y, bar.w, bar.h,
                if bar.contains(mouse_position().into()) || self.drag_scroll { WHITE } else { GRAY }
            );
        }

        if mouse_wheel().1 == 0. && !self.drag_scroll &&!just_deleted { return }

        // undo last scroll
        for manager in self.layer_managers.iter_mut() {
            manager.translate((manager.outer_rect.x, manager.outer_rect.y - self.scroll), visualiser);
        }
        self.add_button.translate((0., -self.scroll));

        if self.drag_scroll {
            self.scroll = self.orig_drag_scroll + (self.bar_drag_start - mouse_position().1) * (total_height / menu_height);
        } else {
            self.scroll += mouse_wheel().1 / 4.
        }
        if total_height < menu_height + screen_width()*LAYERMANAGER_TOP_PADDING {
            self.scroll = 0.;
        } else {
            self.scroll = self.scroll.clamp(0., total_height - menu_height + screen_width()*LAYERMANAGER_TOP_PADDING);
        }

        for manager in self.layer_managers.iter_mut() {
            manager.translate((manager.outer_rect.x, manager.outer_rect.y + self.scroll), visualiser);
        }
        self.add_button.translate((0., self.scroll));
    }
}
impl MenuType for LayersMenu {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        // first iteration just finds inactive dropdowns
        let mut inactive_dropdowns = Vec::with_capacity(self.layer_managers.len());
        for (i, manager) in self.layer_managers.iter().enumerate() {
            if !manager.layer_range_dropdown.open { continue }
            if i > 0 {
                inactive_dropdowns.push(i-1);
            }
            if manager.edit_button.rect.overlaps(&manager.layer_range_dropdown.open_grad_input_box.outer_rect()) {
                inactive_dropdowns.push(i);
            }
        }

        // iterate and updated the layer managers
        let mut changed = false;
        let mut drag_i: Option<usize> = None;
        let mut released_i: Option<usize> = None;
        let mut delete_i: Option<usize> = None;
        for (i, manager) in self.layer_managers.iter_mut().enumerate() {
            let this_changed = manager.update(
                visualiser,
                i, 
                !inactive_dropdowns.contains(&i)
            );
            if this_changed { changed = true }

            if manager.dragging {
                drag_i = Some(i);
            }
            if manager.released {
                released_i = Some(i);
                manager.released = false;
            }

            if manager.delete_button.clicked {
                delete_i = Some(i);
            }

            if manager.edit_button.clicked {
                return MenuSignal::OpenEditor(i);
            }

            if manager.palette_button.clicked {
                return MenuSignal::OpenPalette(i);
            }
        }

        if let Some(i) = drag_i {
            self.draw_drag(i, &visualiser);
        }
        if let Some(i) = released_i {
            self.release(i, visualiser);
            changed = true;
        }

        if let Some(i) = delete_i {
            self.delete_layer(visualiser, i);
            changed = true;
        }

        self.add_button.update();
        if self.add_button.clicked {
            self.add_layer(visualiser);
            changed = true;
        }

        // cutoff before navbar/heading
        draw_rectangle(0., 0., 
            screen_width()*MENU_SCREEN_PROPORTION, 
            screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION, 
            BLACK
        );

        if drag_i.is_none() {
            self.update_scroll(&visualiser, false);
        }

        if changed {
            Layers::place_constraints(&mut visualiser.layers.layers);
            visualiser.generate_image();
            return MenuSignal::RefreshGradients;
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for manager in self.layer_managers.iter() {
            if manager.name.selected { return true }
        }
        false
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        self.add_button.refresh_gradient(visualiser);
        for manager in self.layer_managers.iter_mut() {
            manager.refresh_gradient(visualiser);
        }
    }
}
impl Drop for LayersMenu {
    fn drop(&mut self) {
        self.add_button.drop_textures();
        for manager in self.layer_managers.iter() {
            Texture2D::delete(&manager.border_back);
            manager.palette_button.drop_textures();
            manager.edit_button.drop_textures();
            manager.delete_button.drop_textures();
        }
    }
}

struct OrbitTrapEditor {
    top_bar: Texture2D,
    top_bar_y: f32,
    title_params: TextParams,
    trap_type: DropDown<OrbitTrapType>,
    analysis: DropDown<OrbitTrapAnalysis>,
    center_re: TextBox,
    center_im: TextBox,
    radius: TextBox,
    arm_length: TextBox
}
impl OrbitTrapEditor {
    async fn new(visualiser: &Visualiser) -> OrbitTrapEditor {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        let font_size = (screen_width()*LAYEREDITOR_SPECIFIC_MENU_TITLE_FONT_PROPORTION) as u16;

        let title_height = measure_text(
            "Orbit Trap", 
            Some(font), 
            (screen_width()*LAYEREDITOR_SPECIFIC_MENU_TITLE_FONT_PROPORTION) as u16, 
            1.0
        ).height;

        let top_bar = get_back_gradient(visualiser, 0, 
            (screen_width()*MENU_SCREEN_PROPORTION) as u16, 
            (screen_height()*LAYEREDTIOR_SPECIFIC_MENU_BAR_HEIGHT) as u16
        );
        let top_bar_y = navbar_bottom() + 
            screen_height()*(LAYEREDITOR_CAROUSEL_HEIGHT+DEFAULT_INPUT_BOX_HEIGHT+2.*LAYEREDITOR_INPUT_BOX_VERT_PADDING);

        let vert_padding = screen_height() * LAYEREDITOR_INPUT_BOX_VERT_PADDING;

        let trap_type_input_box = GradientInputBox::default(visualiser, 
            top_bar_y + title_height + screen_height()*(2.*LAYEREDITOR_INPUT_BOX_VERT_PADDING+LAYEREDTIOR_SPECIFIC_MENU_BAR_HEIGHT)
        );
        let analysis_input_box = trap_type_input_box.next_vert(visualiser, vert_padding, true);
        let center_re_input_box = analysis_input_box.next_vert(visualiser, vert_padding, true);
        let center_im_input_box = center_re_input_box.next_vert(visualiser, vert_padding, true);
        let specific_input_box = center_im_input_box.next_vert(visualiser, vert_padding, true);

        OrbitTrapEditor {
            top_bar,
            top_bar_y,
            title_params: TextParams { 
                font: font, 
                font_size, 
                color: get_brightest_colour(top_bar),
                ..Default::default()
            },
            trap_type: DropDown::new(visualiser, trap_type_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "type", true), 
                InputLabel::default_input_box_content(font)).await,
            analysis: DropDown::new(visualiser, analysis_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "analysis", true), 
                InputLabel::default_input_box_content(font)).await,
            center_re: TextBox::new(center_re_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "center (re)", true), 
                InputLabel::default_input_box_content(font), ""),
            center_im: TextBox::new(center_im_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "center (im)", true), 
                InputLabel::default_input_box_content(font), ""),
            radius: TextBox::new(specific_input_box.sealed_clone(visualiser), 
                InputLabel::default_input_box_label(visualiser, font, "radius", true), 
                InputLabel::default_input_box_content(font), ""),
            arm_length: TextBox::new(specific_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "arm length", true), 
                InputLabel::default_input_box_content(font), "")
        }
    }

    /// updates the orbit trap and returns whether it has been changed
    fn update(&mut self, orbit_trap: &mut OrbitTrapType, editing_layer_type: bool) -> bool { 
        draw_texture(self.top_bar, 0., self.top_bar_y, WHITE);
        draw_text_ex("Orbit Trap", 0., self.top_bar_y+self.title_params.font_size as f32, self.title_params);

        let mut changed = false;

        if let Some(new_re) = self.center_re.update(orbit_trap.get_center_re().to_string()) {
            if let Ok(new) = new_re.parse::<f64>() {
                orbit_trap.set_center_re(new);
                changed = true;
            }
        }
        // if let Some(new_im) = self.center_im.update(orbit_trap.get_center_im().to_string()) {
        //     if let Ok(new) = new_im.parse::<f64>() {
        //         orbit_trap.set_center_im(new);
        //         changed = true;
        //     }
        // }
        if let Some(Ok(new)) = self.center_im
                                    .update(orbit_trap.get_center_im().to_string())
                                    .and_then(|new_im| Some(new_im.parse::<f64>())) {
            orbit_trap.set_center_im(new);
            changed = true;
        }
                            

        match orbit_trap {
            OrbitTrapType::Point(_) => {},
            OrbitTrapType::Cross(cross) => {
                if let Some(Ok(new)) = self.arm_length
                                            .update(cross.arm_length.to_string())
                                            .and_then(|new_arm_len| Some(new_arm_len.parse::<f64>())) {
                    cross.arm_length = new;
                    changed = true;
                }
            },
            OrbitTrapType::Circle(circle) => {
                if let Some(Ok(new)) = self.radius
                                            .update(circle.radius.to_string())
                                            .and_then(|new_rad| Some(new_rad.parse::<f64>())) {
                    circle.radius = new;
                    changed = true;
                }
            }
        }

        if !self.trap_type.open && !editing_layer_type {
            if let Some(new) = self.analysis.update(&orbit_trap.get_analysis()) {
                orbit_trap.set_analysis(new);
                changed = true;
            }
        } else {
            self.analysis.draw(&orbit_trap.get_analysis());
        }
        if !editing_layer_type {
            if let Some(new) = self.trap_type.update(orbit_trap) {
                *orbit_trap = new;
                changed = true;
            }
        } else {
            self.trap_type.draw(&orbit_trap);
        }

        changed
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.top_bar);
        self.top_bar = get_back_gradient(visualiser, 0, 
            (screen_width()*MENU_SCREEN_PROPORTION) as u16, 
            (screen_height()*LAYEREDTIOR_SPECIFIC_MENU_BAR_HEIGHT) as u16
        );

        self.trap_type.refresh_gradient(visualiser);
        self.analysis.refresh_gradient(visualiser);
        self.center_re.refresh_gradient(visualiser);
        self.center_im.refresh_gradient(visualiser);
        self.radius.refresh_gradient(visualiser);
        self.arm_length.refresh_gradient(visualiser);
    }
}

struct LayerCarouselType<'a> {
    layer: &'a mut Layer
}
impl<'a> CarouselType for LayerCarouselType<'a> {
    fn get_string(&self) -> String {
        self.layer.name.clone()
    }
}

struct LayerEditorMenu {
    layer_carousel: Carousel,
    layer_type: DropDown<LayerType>,
    current_index: usize,
    orbit_trap_editor: OrbitTrapEditor
}
impl LayerEditorMenu {
    async fn new(visualiser: &Visualiser) -> LayerEditorMenu {
        let carousel_font_size = screen_width()*LAYEREDITOR_CAROUSEL_FONT_PROPORTION;
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        
        let carousel_start_y = screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION;
        let carousel_height = screen_height()*LAYEREDITOR_CAROUSEL_HEIGHT;

        let type_textbox_start_y = carousel_start_y + carousel_height + screen_height()*LAYEREDITOR_INPUT_BOX_VERT_PADDING;

        LayerEditorMenu { 
            layer_carousel: Carousel::new(
                GradientInputBox::new(
                    visualiser, 
                    0., carousel_start_y,
                    screen_width()*MENU_SCREEN_PROPORTION, carousel_height, 
                    screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION
                ),
                InputLabel::new("", font, carousel_font_size, WHITE,  false, 
                    0., TextAlign::Centre)
            ).await,
            layer_type: DropDown::new(
                visualiser,
                GradientInputBox::default(visualiser, type_textbox_start_y),
                InputLabel::default_input_box_label(visualiser, font, "type", true),
                InputLabel::default_input_box_content(font)
            ).await,
            current_index: 0,
            orbit_trap_editor: OrbitTrapEditor::new(visualiser).await
        }
    }

    fn set_layer_to_edit(&mut self, index: usize) {
        self.current_index = index;
    }

    fn get_layers(visualiser: &mut Visualiser) -> Vec<LayerCarouselType> {
        visualiser.layers.layers.iter_mut().map(|layer| LayerCarouselType {layer}).collect()
    }

    fn update_orbit_trap(&mut self, orbit_trap: &mut OrbitTrapType, editing_layer_type: bool) -> bool {
        self.orbit_trap_editor.update(orbit_trap, editing_layer_type)
    }
}
impl MenuType for LayerEditorMenu {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        let mut changed: bool = false;

        self.current_index = self.current_index.clamp(0, visualiser.layers.layers.len()-1);

        if let Some(index) = self.layer_carousel.update(LayerEditorMenu::get_layers(visualiser), self.current_index) {
            self.set_layer_to_edit(index);
        }

        if visualiser.layers.layers[self.current_index].layer_type.is_orbit_trap() {
            if self.update_orbit_trap(
                visualiser.layers.layers[self.current_index].layer_type.get_orbit_trap().unwrap(),
                self.layer_type.open
            ) {
                changed = true;
            }
        }

        if let Some(new_type) = self.layer_type.update(&visualiser.layers.layers[self.current_index].layer_type) {
            visualiser.layers.change_layer_type(self.current_index, new_type);
            changed = true;
        }

        if changed {
            Layers::place_constraints(&mut visualiser.layers.layers);
            visualiser.layers.update_implementors();
            visualiser.generate_image();
            return MenuSignal::RefreshGradients;
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        self.orbit_trap_editor.center_im.selected || self.orbit_trap_editor.center_re.selected ||
        self.orbit_trap_editor.radius.selected || self.orbit_trap_editor.arm_length.selected
    }

    fn open_layer_to_edit(&mut self, index: usize, _visualiser: &Visualiser) {
        self.set_layer_to_edit(index);
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        self.layer_carousel.refresh_gradient(visualiser);
        self.layer_type.refresh_gradient(visualiser);
        self.orbit_trap_editor.refresh_gradients(visualiser);
    }
}

fn color_with_params(colour: &Color, r: Option<f32>, g: Option<f32>, b: Option<f32>, a: Option<f32>) -> Color {
    Color {
        r: r.unwrap_or(colour.r),
        g: g.unwrap_or(colour.g),
        b: b.unwrap_or(colour.b),
        a: a.unwrap_or(colour.a)
    }
}

struct PercentageEditor {
    domain_rect: Rect,
    rect: Rect,
    selected: bool,
    deselect_y: f32,
    selected_x_offset: f32
}
impl PercentageEditor {
    fn new(domain_rect: &Rect, rect: Rect) -> PercentageEditor {
        PercentageEditor { 
            domain_rect: domain_rect.clone(), 
            rect, 
            selected: false, 
            deselect_y: domain_rect.bottom() + rect.h + screen_height()*PALETTEEDITOR_VERT_PADDING,
            selected_x_offset: 0.0
         }
    }

    /// # Returns
    /// 
    /// None if the percentage was unchanged
    /// Some(new percentage) if the percentage was changed
    fn update(&mut self, other_selected: bool) -> Option<f32> {
        self.mouse_interact(other_selected)
    }

    fn mouse_interact(&mut self, other_selected: bool) -> Option<f32> {
        if !self.rect.contains(mouse_position().into()) 
                && is_mouse_button_pressed(MouseButton::Left) 
                && (mouse_position().1 <= self.deselect_y || mouse_position().0 > screen_width()*MENU_SCREEN_PROPORTION) {
            self.selected = false;
        }

        if other_selected { return None }

        if self.rect.contains(mouse_position().into()) && is_mouse_button_pressed(MouseButton::Left) {
            self.selected = true;
            self.selected_x_offset = self.rect.x - mouse_position().0;
        }

        if self.selected && is_mouse_button_down(MouseButton::Left) && (mouse_position().1 <= self.deselect_y) {
            self.translate_to(mouse_position().0 + self.selected_x_offset)
        } else {
            None
        }
    }

    fn translate_to(&mut self, new_x: f32) -> Option<f32> {
        let old_x = self.rect.x;

        self.rect.x = new_x;
        self.rect.x = self.rect.x.clamp(self.domain_rect.x - self.rect.w/2., self.domain_rect.right() - self.rect.w/2.);

        let delta = self.rect.x - old_x;

        match delta == 0.0 {
            true => None,
            false => Some( self.get_percent() )
        }
    }

    fn get_percent(&self) -> f32 {
        (self.rect.center().x - self.domain_rect.x) / self.domain_rect.w
    }
}

struct ColourPointEditor {
    percentage_editor: PercentageEditor,
    colour: Color,
    outer_select_box: Rect,
    inner_select_box: Rect
}
impl ColourPointEditor {
    fn new(colour_point: &ColourPoint, map_rect: Rect) -> ColourPointEditor {
        let width = screen_width()*PALETTEEDITOR_COLOUR_POINT_WIDTH;
        let select_width = screen_width()*PALETTEEDITOR_COLOUR_POINT_SELECT_WIDTH;
        let x = map_rect.x - width/2. + colour_point.percent*map_rect.w;

        let outer_select_box = Rect::new(
            x + (width-select_width)/2.,
            map_rect.top(),
            select_width,
            map_rect.h
        );

        ColourPointEditor { 
            percentage_editor: PercentageEditor::new(
                &map_rect, 
                Rect::new(x, map_rect.bottom(), width, 2. * width)
            ),
            colour: colour_point.colour,
            outer_select_box,
            inner_select_box: inflate_rect(&outer_select_box, -screen_width()*PALETTEEDITOR_COLOUR_POINT_SELECT_BORDER_WIDTH)
        }
    }

    /// draw and update the `ColourPointEditor`
    /// 
    /// # Returns
    /// None if the point was unchanged
    /// Some(new percentage) if the point was changed 
    fn update(&mut self, other_selected: bool) -> Option<f32> {
        self.draw();
        let output = self.percentage_editor.mouse_interact(other_selected);
        
        self.outer_select_box.x = self.percentage_editor.rect.center().x - self.outer_select_box.w/2.;
        self.inner_select_box.x = self.percentage_editor.rect.center().x - self.inner_select_box.w/2.;

        output
    }
    
    fn draw(&self) {
        let color = match self.percentage_editor.selected {
            true => WHITE,
            false => LAYERMANAGER_LAYER_TYPE_COLOUR
        };

        let rect = &self.percentage_editor.rect;
        draw_triangle(
            Vec2::new(rect.center().x, rect.y),
            Vec2::new(rect.x, rect.center().y),
            Vec2::new(rect.right(), rect.center().y),
            color
        );
        draw_rectangle(rect.x,rect.center().y,rect.w,rect.h/2.,color);
        draw_circle(rect.center().x,rect.y + rect.h * 0.75,rect.w * 0.4,self.colour);

        if !self.percentage_editor.selected { return }

        draw_rect(&self.outer_select_box, WHITE);
        draw_rect(&self.inner_select_box, self.colour);
    }
}

struct PaletteEditor {
    old_palette: Palette,
    layer_index: usize,
    font: Font,
    title_back: Texture2D,
    inner_title_rect: Rect,
    title_text_measure: TextDimensions,
    title_text_colour: Color,
    colour_map_rect: Rect,
    colour_map_texture: Texture2D,
    colour_point_editors: Vec<ColourPointEditor>,
    add_button: Button,
    delete_button: Button,
    red_slider: Slider,
    green_slider: Slider,
    blue_slider: Slider,
    alpha_slider: Slider,
    bar_rect: Rect,
    bar_grad: Texture2D,
    palette_rect: Rect,
    palette_texture: Texture2D,
    mapping_type: DropDown<MappingType>,
    length_slider: Slider,
    offset_slider: Slider,
    sumbit_button: Button,
    cancel_button: Button
}
impl PaletteEditor {
    async fn new(visualiser: &Visualiser) -> PaletteEditor {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();

        let title_rect = Rect::new(0., 0., 
            screen_width()*MENU_SCREEN_PROPORTION, 
            screen_height()*(2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION);
        let title_back = get_back_gradient(visualiser, 0, title_rect.w as u16, title_rect.h as u16);
            
        let start_x = screen_width()*PALETTEEDITOR_HOR_PADDING;
        let vert_padding = screen_height()*PALETTEEDITOR_VERT_PADDING;

        let button_size = screen_width()*PALETTEEDITOR_BUTTON_WIDTH;
        let button_border = screen_width()*PALETTEEDIOR_BUTTON_BORDER_WIDTH;
        let add_rect = Rect::new(
            start_x, 
            title_rect.h + 2.*vert_padding + screen_height()*PALETTEEDITOR_PALETTE_HEIGHT
                +screen_width()*2.*PALETTEEDITOR_COLOUR_POINT_WIDTH,
            button_size, button_size
        );
        let mut delete_rect = add_rect.clone();
        delete_rect.x = screen_width()*(MENU_SCREEN_PROPORTION-PALETTEEDITOR_HOR_PADDING)-button_size;
        let sumbit_rect = Rect::new(
            screen_width()*(MENU_SCREEN_PROPORTION/2. - PALETTEEDITOR_HOR_PADDING/2.) - button_size, 
            screen_height() - button_size - vert_padding,
            button_size, button_size
        );
        let mut cancel_rect = sumbit_rect.clone();
        cancel_rect.x = screen_width()*(MENU_SCREEN_PROPORTION/2. + PALETTEEDITOR_HOR_PADDING/2.);

        let bar_bottom = title_rect.h + 
            vert_padding*4. + 
            screen_height()*(PALETTEEDITOR_PALETTE_HEIGHT+4.*PALETTEEDITOR_TEXTBOX_HEIGHT+3.*PALETTEEDITOR_TEXTBOX_VERT_PADDING+
                PALETTEEDITOR_BAR_HEIGHT) +
            screen_width()*(2.*PALETTEEDITOR_COLOUR_POINT_WIDTH+PALETTEEDITOR_BUTTON_WIDTH);
        let bar_rect = Rect::new(
            0., 
            bar_bottom-screen_height()*PALETTEEDITOR_BAR_HEIGHT, 
            screen_width()*MENU_SCREEN_PROPORTION,
            screen_height()*PALETTEEDITOR_BAR_HEIGHT
        );
        let palette_rect = Rect::new(start_x, bar_bottom+vert_padding, 
            screen_width()*(MENU_SCREEN_PROPORTION-2.*PALETTEEDITOR_HOR_PADDING),
            screen_height()*PALETTEEDITOR_PALETTE_HEIGHT
        );

        let textbox_dims = PaletteEditor::get_textbox_dims(title_rect.h, vert_padding);

        PaletteEditor { 
            old_palette: Palette::default(),
            layer_index: 0,
            font,
            title_back,
            inner_title_rect: inflate_rect(&title_rect, -screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION),
            title_text_measure: measure_text(
                "PALETTE EDITOR", 
                Some(font), 
                (screen_width()*STATE_TEXT_FONT_PROPORTION) as u16, 
                1.0
            ),
            title_text_colour: get_brightest_colour(title_back),
            colour_map_rect: Rect::new(
                start_x, title_rect.h + vert_padding, 
                screen_width()*(MENU_SCREEN_PROPORTION-2.*PALETTEEDITOR_HOR_PADDING),
                screen_height()*PALETTEEDITOR_PALETTE_HEIGHT
            ),
            colour_map_texture: Texture2D::empty(),
            colour_point_editors: Vec::new(),
            add_button: Button::gradient_border_and_image(visualiser, &add_rect, 
                button_border, load_image("assets/plus.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            delete_button: Button::gradient_border_and_alternating_image(
                visualiser, &delete_rect, button_border, 
                load_image("assets/bin.png").await.unwrap(), DrawTextureParams::default(),
                load_image("assets/binOpen.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            red_slider: PaletteEditor::get_slider(0, visualiser, font, title_rect.h, vert_padding),
            green_slider: PaletteEditor::get_slider(1, visualiser, font, title_rect.h, vert_padding),
            blue_slider: PaletteEditor::get_slider(2, visualiser, font, title_rect.h, vert_padding),
            alpha_slider: PaletteEditor::get_slider(3, visualiser, font, title_rect.h, vert_padding),
            bar_rect, 
            bar_grad: get_back_gradient(visualiser, bar_rect.x as u16, bar_rect.w as u16, bar_rect.h as u16),
            palette_rect,
            palette_texture: Texture2D::empty(),
            mapping_type: DropDown::new(
                visualiser,
                GradientInputBox::new(
                    visualiser, 
                    textbox_dims.right()-screen_width()*PALETTEEDITOR_MAPPING_DROPDOWN_WIDTH, palette_rect.bottom() + vert_padding,
                    screen_width()*PALETTEEDITOR_MAPPING_DROPDOWN_WIDTH, textbox_dims.h,
                    screen_height() * DEFAULT_INPUT_BOX_BORDER_SIZE
                ),
                InputLabel::default_input_box_label(visualiser, font, "mapping type", false),
                InputLabel::default_input_box_content(font)
            ).await,
            length_slider: PaletteEditor::get_slider(4, visualiser, font, title_rect.h, vert_padding),
            offset_slider: PaletteEditor::get_slider(5, visualiser, font, title_rect.h, vert_padding),
            sumbit_button: Button::gradient_border_and_image(
                visualiser, &sumbit_rect, button_border, 
                load_image("assets/tick.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            cancel_button: Button::gradient_border_and_image(
                visualiser, &cancel_rect, button_border,
                load_image("assets/cross.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            )
        }
    }

    fn colour_slider_rect(slider_i: usize, title_height: f32, vert_padding: f32) -> Rect {
        let start_x = screen_width()*PALETTEEDITOR_COLOUR_SLIDER_START_X;

        // TODO: un spaghetti this
        let y = match slider_i <= 3 {
            true => title_height + 3.*vert_padding + screen_height()*(PALETTEEDITOR_PALETTE_HEIGHT) +
                    screen_width()*(2.*PALETTEEDITOR_COLOUR_POINT_WIDTH+PALETTEEDITOR_BUTTON_WIDTH) +
                    screen_height()*(PALETTEEDITOR_TEXTBOX_HEIGHT-PALETTEEDITOR_COLOUR_SLIDER_HEIGHT)/2. +
                    slider_i as f32 * screen_height()*(PALETTEEDITOR_COLOUR_SLIDER_HEIGHT + 
                        PALETTEEDITOR_TEXTBOX_HEIGHT-PALETTEEDITOR_COLOUR_SLIDER_HEIGHT +
                        PALETTEEDITOR_TEXTBOX_VERT_PADDING),
            false => title_height + 7.*vert_padding + screen_height()*(2.*PALETTEEDITOR_PALETTE_HEIGHT+
                        5.*PALETTEEDITOR_TEXTBOX_HEIGHT+3.*PALETTEEDITOR_TEXTBOX_VERT_PADDING) +
                    screen_width()*(2.*PALETTEEDITOR_COLOUR_POINT_WIDTH+PALETTEEDITOR_BUTTON_WIDTH) +
                    screen_height()*(PALETTEEDITOR_TEXTBOX_HEIGHT-PALETTEEDITOR_COLOUR_SLIDER_HEIGHT)/2. +
                    (slider_i - 4) as f32 * screen_height()*(PALETTEEDITOR_COLOUR_SLIDER_HEIGHT + 
                        PALETTEEDITOR_TEXTBOX_HEIGHT-PALETTEEDITOR_COLOUR_SLIDER_HEIGHT +
                        PALETTEEDITOR_TEXTBOX_VERT_PADDING),
        };

        Rect::new(
            start_x, y,
            screen_width()*(MENU_SCREEN_PROPORTION-2.*PALETTEEDITOR_HOR_PADDING-PALETTEEDITOR_TEXTBOX_WIDTH) - 
                start_x - screen_height()*PALETTEEDITOR_COLOUR_SLIDER_HEIGHT,
            screen_height()*PALETTEEDITOR_COLOUR_SLIDER_HEIGHT
        )
    }

    fn get_textbox_dims(title_height: f32, vert_padding: f32) -> Rect {
        let rect = PaletteEditor::colour_slider_rect(0, title_height, vert_padding);
        Rect::new(
            rect.right() + screen_width()*PALETTEEDITOR_HOR_PADDING + rect.h*0.8,
            0.,
            screen_width()*PALETTEEDITOR_TEXTBOX_WIDTH,
            screen_height()*PALETTEEDITOR_TEXTBOX_HEIGHT
        )
    }

    fn get_slider(slider_i: usize, visualiser: &Visualiser, font: Font, title_height: f32, vert_padding: f32) -> Slider {
        let font_size = screen_width()*PALETTEEDTIOR_FONT_PROPORTION;

        let rect = PaletteEditor::colour_slider_rect(slider_i, title_height, vert_padding);

        let textbox_start_x = rect.right() + screen_width()*PALETTEEDITOR_HOR_PADDING + rect.h*0.8;
        let textbox_width = screen_width()*PALETTEEDITOR_TEXTBOX_WIDTH;
        let textbox_height = screen_height()*PALETTEEDITOR_TEXTBOX_HEIGHT;

        Slider::new(
            Some(
                InputLabel::new(
                    match slider_i {
                        0 => "red",
                        1 => "green",
                        2 => "blue",
                        3 => "alpha",
                        4 => "length",
                        _ => "offset"
                    }, 
                    font, 
                    font_size, 
                    WHITE, false,
                    screen_width() * DEFAULT_INPUT_BOX_LABEL_PADDING, 
                    TextAlign::StartX(screen_width() * MENU_HOR_PADDING)
                )),
            0., 
            match slider_i <= 3 {
                true => 255.,
                false => 100.
            },
            None, 
            Some(TextBox::new(
                GradientInputBox::new(
                    visualiser, 
                    textbox_start_x, 
                    rect.y - screen_height()*(PALETTEEDITOR_TEXTBOX_HEIGHT-PALETTEEDITOR_COLOUR_SLIDER_HEIGHT)/2.,
                    textbox_width, textbox_height, 
                    screen_height() * DEFAULT_INPUT_BOX_BORDER_SIZE
                ),  
                None,
                InputLabel::default_input_box_content(font),
                ""
            )),
            rect.x, rect.y, rect.w, rect.h,
            match slider_i <= 3 {
                true => Box::new(GradientSliderBar::empty()),
                false => Box::new(SolidSliderBar::new(LAYERMANAGER_LAYER_TYPE_COLOUR))
            },
            rect.h*0.8,
            match slider_i {
                0 => Color { r: 0.5, g: 0., b: 0., a: 1.},
                1 => Color { r: 0., g: 0.5, b: 0., a: 1.},
                2 => Color { r: 0., g: 0., b: 0.5, a: 1.},
                3 => Color { r: 0.5, g: 0.5, b: 0.5, a: 1.},
                _ => LAYERMANAGER_LAYER_TYPE_COLOUR
            },
            match slider_i {
                0 => Color { r: 1., g: 0., b: 0., a: 1. },
                1 => Color { r: 0., g: 1., b: 0., a: 1. },
                2 => Color { r: 0., g: 0., b: 1., a: 1. },
                3 => Color { r: 1., g: 1., b: 1., a: 1. },
                _ => WHITE
            }
            
        )
    }

    fn load_colour_points(&mut self, palette: &Palette) {
        self.colour_point_editors = Vec::with_capacity(palette.colour_map.len());
        for point in palette.colour_map.iter() {
            self.colour_point_editors.push(ColourPointEditor::new(point, self.colour_map_rect));
        }
    }

    fn draw_title(&self) {
        draw_texture(self.title_back, 0., 0., WHITE);
        draw_rectangle(self.inner_title_rect.x, self.inner_title_rect.y, self.inner_title_rect.w, self.inner_title_rect.h, BLACK);
        draw_text_ex(
            "PALETTE EDITOR",
            self.inner_title_rect.center().x - self.title_text_measure.width/2.,
            self.inner_title_rect.center().y + self.title_text_measure.height/2.,
            TextParams { 
                font: self.font, 
                font_size: (screen_width()*STATE_TEXT_FONT_PROPORTION) as u16, 
                color: self.title_text_colour, 
                ..Default::default() 
            }
        );
    }

    fn update_add_button(&mut self, palette: &mut Palette) -> bool {
        if palette.get_add_point_percent().is_none() { return false }

        self.add_button.update();
        if !self.add_button.clicked { return false }

        palette.add_point();
        self.load_colour_points(palette);
        true
    }

    fn update_delete_button(&mut self, palette: &mut Palette, selected_point: Option<usize>) -> bool {
        let index = match selected_point {
            None => {return false},
            Some(index) => index
        };
        if !palette.can_delete_point() { return false }

        self.delete_button.update();
        if !self.delete_button.clicked { return false }

        palette.delete_point(index);
        self.load_colour_points(palette);
        true
    }

    fn update_colour_sliders(&mut self, palette: &mut Palette, selected_point: Option<usize>) -> bool {
        let index = match selected_point {
            None => {return false},
            Some(index) => index
        };
        let colour = palette.colour_map[index].colour;
        let mut update_colour = false;

        self.red_slider.slider_bar.make_gradient(
            self.red_slider.rect, 
            color_with_params(&colour, Some(0.), None, None, None), 
            color_with_params(&colour, Some(1.), None, None, None)
        );
        self.red_slider.percentage = colour.r;
        self.red_slider.update();
        if self.red_slider.percentage != colour.r { update_colour = true }

        self.green_slider.slider_bar.make_gradient(
            self.green_slider.rect, 
            color_with_params(&colour, None, Some(0.), None, None), 
            color_with_params(&colour, None, Some(1.), None, None), 
        );
        self.green_slider.percentage = colour.g;
        self.green_slider.update();
        if self.green_slider.percentage != colour.g { update_colour = true }

        self.blue_slider.slider_bar.make_gradient(
            self.blue_slider.rect, 
            color_with_params(&colour, None, None, Some(0.), None), 
            color_with_params(&colour, None, None, Some(1.), None), 
        );
        self.blue_slider.percentage = colour.b;
        self.blue_slider.update();
        if self.blue_slider.percentage != colour.b { update_colour = true }

        self.alpha_slider.slider_bar.make_gradient(
            self.alpha_slider.rect, 
            color_with_params(&colour, None, None, None, Some(0.)),
            color_with_params(&colour, None, None, None, Some(1.))
        );
        self.alpha_slider.percentage = colour.a;
        self.alpha_slider.update();
        if self.alpha_slider.percentage != colour.a { update_colour = true }

        if update_colour {
            palette.update_colour(
                index, 
                Some(self.red_slider.percentage), 
                Some(self.green_slider.percentage), 
                Some(self.blue_slider.percentage), 
                Some(self.alpha_slider.percentage)
            );
            self.colour_point_editors[index].colour.r = self.red_slider.percentage;
            self.colour_point_editors[index].colour.g = self.green_slider.percentage;
            self.colour_point_editors[index].colour.b = self.blue_slider.percentage;
            self.colour_point_editors[index].colour.a = self.alpha_slider.percentage;
            true
        } else {
            false
        }
    }
}
impl MenuType for PaletteEditor {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        let palette = &mut visualiser.layers.layers[self.layer_index].palette;
        
        self.draw_title();

        let mut changed_this_frame = false;

        // for some reason deleting only works before creating a new texture, not after using one,
        // so this texture has to be an attribute of self
        Texture2D::delete(&self.colour_map_texture);
        self.colour_map_texture = palette.get_full_gradient(self.colour_map_rect.w, self.colour_map_rect.h);
        draw_texture(
            self.colour_map_texture, 
            self.colour_map_rect.x, self.colour_map_rect.y, WHITE
        );
        let mut selected_point: Option<usize> = None;
        for (i, point_editor) in self.colour_point_editors.iter_mut().enumerate() {
            if let Some(new_percent) = point_editor.update(
                selected_point.is_some()
            ) {
                if visualiser.layers.layers[self.layer_index].palette.change_point_percent(new_percent, i) {
                    changed_this_frame = true;
                }
            }
            if point_editor.percentage_editor.selected { selected_point = Some(i) }
        }

        let palette = &mut visualiser.layers.layers[self.layer_index].palette;

        if self.update_colour_sliders(palette, selected_point) {
            changed_this_frame = true;
        }

        if self.update_add_button(palette) {
            changed_this_frame = true;
        }

        if self.update_delete_button(palette, selected_point) {
            changed_this_frame = true;
        }

        draw_texture(self.bar_grad, self.bar_rect.x, self.bar_rect.y, WHITE);
        Texture2D::delete(&self.palette_texture);
        self.palette_texture = palette.get_full_palette(self.palette_rect.w, self.palette_rect.h);
        draw_texture(
            self.palette_texture, 
            self.palette_rect.x, self.palette_rect.y, WHITE
        );

        if !self.mapping_type.open {
            self.length_slider.percentage = palette.get_palette_length();
            self.length_slider.update();
            if palette.set_palette_length(self.length_slider.percentage) {
                changed_this_frame = true;
            }

            self.offset_slider.percentage = palette.get_offset();
            self.offset_slider.update();
            if palette.set_offset(self.offset_slider.percentage) {
                changed_this_frame = true;
            }
        } else {
            self.length_slider.draw();
            self.offset_slider.draw();
        }
        

        if let Some(new) = self.mapping_type.update(&palette.mapping_type) {
            palette.mapping_type = new;
            changed_this_frame = true;
        }

        if changed_this_frame {
            visualiser.layers.layers[self.layer_index].palette.generate_palette(visualiser.max_iterations);
            visualiser.generate_image();
        }

        self.sumbit_button.update();
        if self.sumbit_button.clicked {
            return MenuSignal::RefreshGradients;
        }

        self.cancel_button.update();
        if self.cancel_button.clicked {
            visualiser.layers.layers[self.layer_index].palette = self.old_palette.clone();
            visualiser.layers.layers[self.layer_index].palette.generate_palette(visualiser.max_iterations);
            visualiser.generate_image();
            return MenuSignal::RefreshGradients;
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for slider in vec![&self.red_slider, &self.green_slider, &self.blue_slider, &self.alpha_slider,
                                    &self.length_slider, &self.offset_slider] {
            if let Some(s) = slider.percentage_text_box.as_ref().and_then(|tb| Some(tb.selected)) {
                if s == true { return true }
            }
        }
        false
    }

    fn open_layer_to_edit(&mut self, index: usize, visualiser: &Visualiser) {
        self.layer_index = index;
        self.old_palette = visualiser.layers.layers[index].palette.clone();

        self.load_colour_points(&visualiser.layers.layers[index].palette);
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.title_back);
        Texture2D::delete(&self.bar_grad);
        Texture2D::delete(&self.colour_map_texture);
        Texture2D::delete(&self.palette_texture);

        let title_rect = Rect::new(0., 0., 
            screen_width()*MENU_SCREEN_PROPORTION, 
            screen_height()*(2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION);
        self.title_back = get_back_gradient(visualiser, 0, title_rect.w as u16, title_rect.h as u16);
        self.bar_grad =  get_back_gradient(visualiser, self.bar_rect.x as u16, 
            self.bar_rect.w as u16, self.bar_rect.h as u16);
            
        self.layer_index = self.layer_index.clamp(0, visualiser.layers.layers.len()-1);

        let palette = &visualiser.layers.layers[self.layer_index].palette;
        self.colour_map_texture = palette.get_full_gradient(self.colour_map_rect.w, self.colour_map_rect.h);
        self.palette_texture = palette.get_full_palette(self.palette_rect.w, self.palette_rect.h);

        self.add_button.refresh_gradient(visualiser);
        self.delete_button.refresh_gradient(visualiser);
        self.red_slider.refresh_gradient(visualiser);
        self.green_slider.refresh_gradient(visualiser);
        self.blue_slider.refresh_gradient(visualiser);
        self.alpha_slider.refresh_gradient(visualiser);
        self.mapping_type.refresh_gradient(visualiser);
        self.length_slider.refresh_gradient(visualiser);
        self.offset_slider.refresh_gradient(visualiser);
        self.sumbit_button.refresh_gradient(visualiser);
        self.cancel_button.refresh_gradient(visualiser);
    }
}

#[derive(Clone, PartialEq)]
enum ScreenshotResolution {
    R1080p,
    R4k,
    R8k,
    Custom
}
impl ScreenshotResolution {
    fn to_screen_dimensions(&self, width: usize, height: usize) -> ScreenDimensions {
        ScreenDimensions::from_tuple(match self {
            ScreenshotResolution::R1080p => ScreenDimensions::tuple_1080p(),
            ScreenshotResolution::R4k => ScreenDimensions::tuple_4k(),
            ScreenshotResolution::R8k => ScreenDimensions::tuple_8k(),
            ScreenshotResolution::Custom => (width, height)
        })
    }
}
impl DropDownType<ScreenshotResolution> for ScreenshotResolution {
    fn get_variants() -> Vec<ScreenshotResolution> {
        vec![
            ScreenshotResolution::R1080p,
            ScreenshotResolution::R4k,
            ScreenshotResolution::R8k,
            ScreenshotResolution::Custom
        ]
    }

    fn get_string(&self) -> String {
        String::from(match self {
            ScreenshotResolution::R1080p => "1080p",
            ScreenshotResolution::R4k => "4k",
            ScreenshotResolution::R8k => "8k",
            ScreenshotResolution::Custom => "Custom"
        })
    }
}

struct ScreenshotMenu {
    name: TextBox,
    resolution: DropDown<ScreenshotResolution>,
    current_resolution: ScreenshotResolution,
    width: TextBox,
    height: TextBox,
    bar_rect: Rect,
    bar_grad: Texture2D,
    export: Button,
    cancel: Button,
    import: Button,
    progress_bar: ProgressBar,
    exporting: bool,
}
impl ScreenshotMenu {
    async fn new(visualiser: &Visualiser) -> ScreenshotMenu { 
        let font =  load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();

        let vert_padding = screen_height() * DEFAULT_INPUT_BOX_VERT_PADDING;
        let name_input_box = GradientInputBox::default_top(visualiser);
        let res_input_box = name_input_box.next_vert(visualiser, vert_padding, true);
        let width_input_box = res_input_box.next_vert(visualiser, vert_padding, true);
        let height_input_box = width_input_box.next_vert(visualiser, vert_padding, true);

        let bar_rect = Rect::new(
            0.,
            height_input_box.outer_rect().bottom() + screen_height() * SCREENSHOT_VERT_PADDING,
            screen_width() * MENU_SCREEN_PROPORTION,
            screen_height() * SCREENSHOT_BAR_HEIGHT
        );

        let button_size = screen_width()*SCREENSHOT_BUTTON_WIDTH;
        let button_border = screen_width()*SCREENSHOT_BUTTON_BORDER_WIDTH;
        let button_x_padding = screen_width()*TEXTBOX_RIGHT_PADDING;

        let export_rect = Rect::new(
            button_x_padding, bar_rect.y + screen_height()*SCREENSHOT_VERT_PADDING,
            button_size, button_size
        );
        let mut cancel_rect = export_rect.clone();
        cancel_rect.x = screen_width()*MENU_SCREEN_PROPORTION - button_size - button_x_padding;
        let mut import_rect = export_rect.clone();
        import_rect.y += screen_height()*SCREENSHOT_VERT_PADDING + button_size;

        ScreenshotMenu {
            name: TextBox::new(name_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "name", true), 
                InputLabel::default_input_box_content(font),
                "[date]_[time]"
            ),
            resolution: DropDown::new(visualiser, res_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "resolution", true), 
                InputLabel::default_input_box_content(font)).await,
            current_resolution: ScreenshotResolution::R4k,
            width: TextBox::new(width_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "width", true), 
                InputLabel::default_input_box_content(font), "600"),
            height: TextBox::new(height_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "height", true), 
                InputLabel::default_input_box_content(font), "600"),
            bar_rect,
            bar_grad: get_back_gradient(visualiser, 0, bar_rect.w as u16, bar_rect.h as u16),
            export: Button::gradient_border_and_image
            (visualiser, &export_rect, button_border, 
                load_image("assets/export.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            cancel: Button::gradient_border_and_image(
                visualiser, &cancel_rect, button_border, 
                load_image("assets/stop.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            import: Button::gradient_border_and_image(
                visualiser, &import_rect, button_border, 
                load_image("assets/import.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            progress_bar: ProgressBar::new(
                visualiser, 
                Rect::new(
                    screen_width()*(MENU_SCREEN_PROPORTION/2.-PROGRESS_BAR_WIDTH/2.),
                    screen_height()*(1. - PROGRESS_BAR_VERT_PADDING),
                    screen_width()*PROGRESS_BAR_WIDTH,
                    screen_height()*PROGRESS_BAR_HEIGHT
                ),
                Some(InputLabel::new(
                    "0%", 
                    font, 
                    screen_width()*PROGRESS_BAR_FONT_PROPORTION, 
                    WHITE, false,
                    0., TextAlign::Centre // these don't matter for progress bars (yet)
                ))
            ),
            exporting: false
        }
    }

    fn update_top_menu(&mut self) {
        if let Some(new) = self.name.update(self.name.data.clone()) {
            self.name.data = new;
        }
        
        if self.current_resolution == ScreenshotResolution::Custom {
            if !self.resolution.open {
                if let Some(Ok(new)) = self.width.update(self.width.data.clone()).and_then(|d| Some(d.parse::<usize>())) {
                    if new > 0 {self.width.data = new.to_string();}
                }
                if let Some(Ok(new)) = self.height.update(self.height.data.clone()).and_then(|d| Some(d.parse::<usize>())) {
                    if new > 0 {self.height.data = new.to_string();}
                }
            } else {
                self.width.draw();
                self.height.draw();
            }
        } 

        if let Some(new) = self.resolution.update(&self.current_resolution) {
            self.current_resolution = new;
        }
    }

    fn draw_top_menu(&mut self) {
        self.name.draw();
        self.resolution.draw(&self.current_resolution);
        
        if self.current_resolution == ScreenshotResolution::Custom {
            self.width.draw();
            self.height.draw();
        }
    }
}
impl MenuType for ScreenshotMenu {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        draw_texture(self.bar_grad, self.bar_rect.x, self.bar_rect.y, WHITE);

        if !self.exporting {
            self.update_top_menu();

            self.progress_bar.draw(0., false, false);

            self.export.update();
            if self.export.clicked {
                let dimensions = self.current_resolution.to_screen_dimensions(
                    self.width.data.parse().unwrap(), self.height.data.parse().unwrap()
                );

                visualiser.start_export(&self.name.data, dimensions);

                self.exporting = true;
            }

            self.import.update();
            if self.import.clicked {
                let mut images_dir = std::env::current_dir().unwrap();
                images_dir.push("images");
                if let Ok(Some(file_path)) = FileDialog::new()
                    .set_location(&images_dir)
                    .add_filter("Text Files", &["txt"])
                    .show_open_single_file() 
                {
                    visualiser.import_from_file(&file_path);
                    return MenuSignal::Import;
                }
            }
        } else {
            self.draw_top_menu();

            self.progress_bar.draw(
                visualiser.exporter.progress_tracker.lock().unwrap().clone() as f32 / 
                    visualiser.exporter.dims.numpixels() as f32,
                true, true
            );

            self.export.holding = true;
            self.export.draw();
            self.cancel.update();
            if self.cancel.clicked {
                visualiser.cancel_current_render();
                visualiser.exporter.cancel_export();
                self.exporting = false;
            }
            if !visualiser.exporter.exporting {
                self.exporting = false;
            }
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for textbox in vec![&self.name, &self.width, &self.height].iter() {
            if textbox.selected { return true}
        }
        false
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.bar_grad);
        self.bar_grad = get_back_gradient(visualiser, 0, self.bar_rect.w as u16, self.bar_rect.h as u16);

        self.name.refresh_gradient(visualiser);
        self.resolution.refresh_gradient(visualiser);
        self.width.refresh_gradient(visualiser);
        self.height.refresh_gradient(visualiser);
        self.export.refresh_gradient(visualiser);
        self.cancel.refresh_gradient(visualiser);
        self.import.refresh_gradient(visualiser);

        self.progress_bar.refresh_gradient(visualiser);
    }
}

struct VideoMenu {
    name: TextBox,
    resolution: DropDown<ScreenshotResolution>,
    current_resolution: ScreenshotResolution,
    width: TextBox,
    height: TextBox,
    time: TextBox,
    fps: TextBox,
    bar_rect: Rect,
    bar_grad: Texture2D,
    record: Button,
    export: Button,
    resume: Button,
    cancel: Button,
    import: Button,
    progress_bar: ProgressBar,
    exporting: bool,
}
impl VideoMenu {
    async fn new(visualiser: &Visualiser) -> VideoMenu {
        let font =  load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();

        let vert_padding = screen_height() * DEFAULT_INPUT_BOX_VERT_PADDING;
        let name_input_box = GradientInputBox::default_top(visualiser);
        let res_input_box = name_input_box.next_vert(visualiser, vert_padding, true);
        let width_input_box = res_input_box.next_vert(visualiser, vert_padding, true);
        let height_input_box = width_input_box.next_vert(visualiser, vert_padding, true);
        let time_input_box = height_input_box.next_vert(visualiser, vert_padding, true);
        let fps_input_box = time_input_box.next_vert(visualiser, vert_padding, true);

        let bar_rect = Rect::new(
            0.,
            fps_input_box.outer_rect().bottom() + screen_height() * SCREENSHOT_VERT_PADDING,
            screen_width() * MENU_SCREEN_PROPORTION,
            screen_height() * SCREENSHOT_BAR_HEIGHT
        );

        let button_size = screen_width()*SCREENSHOT_BUTTON_WIDTH;
        let button_border = screen_width()*SCREENSHOT_BUTTON_BORDER_WIDTH;
        let button_x_padding = screen_width()*TEXTBOX_RIGHT_PADDING;

        let edit_rect = Rect::new(
            button_x_padding, 
            bar_rect.y + screen_height()*SCREENSHOT_VERT_PADDING,
            button_size, button_size
        );
        let mut export_rect = edit_rect.clone();
        export_rect.x += edit_rect.w + screen_height()*SCREENSHOT_VERT_PADDING;
        let mut cancel_rect = edit_rect.clone();
        cancel_rect.x = screen_width()*MENU_SCREEN_PROPORTION - button_size - button_x_padding;
        let mut resume_rect = cancel_rect.clone();
        resume_rect.x -= button_size - button_x_padding;

        let mut import_rect = edit_rect.clone();
        import_rect.y += edit_rect.h + screen_height()*SCREENSHOT_VERT_PADDING;

        VideoMenu {
            name: TextBox::new(name_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "name", true), 
                InputLabel::default_input_box_content(font),
                "[date]_[time]"
            ),
            resolution: DropDown::new(visualiser, res_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "resolution", true), 
                InputLabel::default_input_box_content(font)).await,
            current_resolution: ScreenshotResolution::R4k,
            width: TextBox::new(width_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "width", true), 
                InputLabel::default_input_box_content(font), "600"),
            height: TextBox::new(height_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "height", true), 
                InputLabel::default_input_box_content(font), "600"),
            time: TextBox::new(time_input_box,
                InputLabel::default_input_box_label(visualiser, font, "seconds", true),
                InputLabel::default_input_box_content(font), "5"),
            fps: TextBox::new(fps_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "FPS", true),
                InputLabel::default_input_box_content(font), "60"),
            bar_rect,
            bar_grad: get_back_gradient(visualiser, 0, bar_rect.w as u16, bar_rect.h as u16),
            record: Button::gradient_border_and_image(
                visualiser, &edit_rect, button_border, 
                load_image("assets/record.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            export: Button::gradient_border_and_image(
                visualiser, &export_rect, button_border, 
                load_image("assets/export.png").await.unwrap(), DrawTextureParams::default(),
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            resume: Button::gradient_border_and_image(
                visualiser, &resume_rect, button_border, 
                load_image("assets/forward.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            cancel: Button::gradient_border_and_image(
                visualiser, &cancel_rect, button_border, 
                load_image("assets/stop.png").await.unwrap(), DrawTextureParams::default(),
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            import: Button::gradient_border_and_image(
                visualiser, &import_rect, button_border, 
                load_image("assets/import.png").await.unwrap(), DrawTextureParams::default(),
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            progress_bar: ProgressBar::new(
                visualiser, 
                Rect::new(
                    screen_width()*(MENU_SCREEN_PROPORTION/2.-PROGRESS_BAR_WIDTH/2.),
                    screen_height()*(1. - PROGRESS_BAR_VERT_PADDING),
                    screen_width()*PROGRESS_BAR_WIDTH,
                    screen_height()*PROGRESS_BAR_HEIGHT
                ),
                Some(InputLabel::new(
                    "0%", 
                    font, 
                    screen_width()*PROGRESS_BAR_FONT_PROPORTION, 
                    WHITE, false,
                    0., TextAlign::Centre // these don't matter for progress bars (yet)
                ))
            ),
            exporting: false
        }
    }

    fn update_top_menu(&mut self, visualiser: &mut Visualiser) {
        if let Some(new) = self.name.update(self.name.data.clone()) {
            self.name.data = new;
            visualiser.video_recorder.changed = true;
        }
        
        if self.current_resolution == ScreenshotResolution::Custom {
            if !self.resolution.open {
                if let Some(Ok(new)) = self.width.update(self.width.data.clone()).and_then(|d| Some(d.parse::<usize>())) {
                    if new > 0 {
                        self.width.data = new.to_string();
                        visualiser.video_recorder.changed = true;
                    }
                }
                if let Some(Ok(new)) = self.height.update(self.height.data.clone()).and_then(|d| Some(d.parse::<usize>())) {
                    if new > 0 {
                        self.height.data = new.to_string();
                        visualiser.video_recorder.changed = true;
                    }
                }
            } else {
                self.width.draw();
                self.height.draw();
            }
        } 

        if let Some(Ok(new)) = self.time.update(self.time.data.clone()).and_then(|d| Some(d.parse::<usize>())) {
            if new > 0 {
                self.time.data = new.to_string();
                visualiser.video_recorder.changed = true;
            }
        }

        if let Some(Ok(new)) = self.fps.update(self.fps.data.clone()).and_then(|d| Some(d.parse::<usize>())) {
            if new > 0 {
                self.fps.data = new.to_string();
                visualiser.video_recorder.changed = true;
            }
        }

        if let Some(new) = self.resolution.update(&self.current_resolution) {
            self.current_resolution = new;
            visualiser.video_recorder.changed = true;
        }
    }

    fn draw_top_menu(&mut self) {
        self.name.draw();
        self.resolution.draw(&self.current_resolution);
        
        if self.current_resolution == ScreenshotResolution::Custom {
            self.width.draw();
            self.height.draw();
        }

        self.time.draw();
        self.fps.draw();
    }
}
impl MenuType for VideoMenu {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        draw_texture(self.bar_grad, self.bar_rect.x, self.bar_rect.y, WHITE);

        if !self.exporting {
            self.update_top_menu(visualiser);

            if visualiser.video_recorder.changed {
                self.progress_bar.draw(0., false, false);
            } else {
                self.progress_bar.draw(visualiser.video_recorder.get_progress(), true, true);
            }

            self.record.update();
            if self.record.clicked {
                return MenuSignal::RecordVideo;
            }

            if !visualiser.video_recorder.changed {
                self.resume.update();
                if self.resume.clicked {
                    visualiser.video_recorder.resume_export();
                    self.exporting = true;
                }
            }

            if visualiser.video_recorder.can_export() {
                self.export.update();
            }
            if self.export.clicked && visualiser.video_recorder.can_export() {
                let dimensions = self.current_resolution.to_screen_dimensions(
                    self.width.data.parse().unwrap(), self.height.data.parse().unwrap()
                );

                visualiser.start_recording(
                    &self.name.data, dimensions, self.time.data.parse().unwrap(), self.fps.data.parse().unwrap()
                );

                self.exporting = true;
            }

            self.import.update();
            if self.import.clicked {
                let mut images_dir = std::env::current_dir().unwrap();
                images_dir.push("videos");
                if let Ok(Some(file_path)) = FileDialog::new()
                    .set_location(&images_dir)
                    .add_filter("Text Files", &["txt"])
                    .show_open_single_file() 
                {
                    visualiser.video_recorder.import_from_file(&file_path);
                    return MenuSignal::RecordVideo;
                }
            }
        } else {
            self.draw_top_menu();

            self.progress_bar.draw(visualiser.video_recorder.get_progress(), true, true);

            self.record.draw();
            self.export.holding = true;
            self.export.draw();
            self.cancel.update();
            if self.cancel.clicked {
                visualiser.video_recorder.cancel_export();
                visualiser.cancel_current_render();
                self.exporting = false;
            }
            if !visualiser.video_recorder.exporting {
                self.exporting = false;
            }
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for textbox in vec![&self.name, &self.width, &self.height, &self.time, &self.fps].iter() {
            if textbox.selected { return true}
        }
        false
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.bar_grad);
        self.bar_grad = get_back_gradient(visualiser, 0, self.bar_rect.w as u16, self.bar_rect.h as u16);

        self.name.refresh_gradient(visualiser);
        self.resolution.refresh_gradient(visualiser);
        self.width.refresh_gradient(visualiser);
        self.height.refresh_gradient(visualiser);
        self.time.refresh_gradient(visualiser);
        self.fps.refresh_gradient(visualiser);
        self.record.refresh_gradient(visualiser);
        self.export.refresh_gradient(visualiser);
        self.resume.refresh_gradient(visualiser);
        self.cancel.refresh_gradient(visualiser);
        self.import.refresh_gradient(visualiser);

        self.progress_bar.refresh_gradient(visualiser);
    }
}

struct VideoTimestampEditor {
    percentage_editor: PercentageEditor,
    font: Font,
    font_size: u16
}
impl VideoTimestampEditor {
    fn new(timestamp: &VideoTimestamp, timeline_rect: &Rect, font: Font) -> VideoTimestampEditor {
        let width = screen_width()*VIDEORECORDER_TIMESTAMP_WIDTH;
        let x = timeline_rect.x - width/2. + timestamp.percent*timeline_rect.w;
        
        VideoTimestampEditor {
            percentage_editor: PercentageEditor::new(
                timeline_rect,
                Rect::new(x, timeline_rect.bottom(), width,  2.*width)
            ),
            font,
            font_size: (screen_width()*VIDEORECORDER_TIMELINE_FONT_PROPORTION) as u16
        }
    }

    /// draw and update the `VideotimestampEditor`
    /// 
    /// # Returns
    /// None if the timestamp was unchanged
    /// Some(new percentage) is the timestamp was changed
    fn update(&mut self, other_selected: bool) -> Option<f32> {
        self.draw();
        self.percentage_editor.update(other_selected)
    }

    fn draw(&self) {
        let color = match self.percentage_editor.selected {
            true => WHITE,
            false => LAYERMANAGER_LAYER_TYPE_COLOUR
        };

        let rect = &self.percentage_editor.rect;
        draw_circle(rect.center().x, rect.center().y, rect.w/2., color);
        draw_rectangle(rect.x, rect.y, rect.w, rect.h/2., BLACK);
        draw_triangle(
            Vec2::new(rect.center().x, rect.y),
            Vec2::new(rect.x, rect.center().y),
            Vec2::new(rect.right(), rect.center().y),
            color
        );

        let percent = format!["{:.1}%", self.percentage_editor.get_percent()*100.];
        let measure = measure_text(&percent, Some(self.font), self.font_size, 1.0);
        draw_text_ex(
            &percent,
            self.percentage_editor.rect.center().x - measure.width/2.,
            self.percentage_editor.rect.bottom() + measure.height,
            TextParams { font: self.font, font_size: self.font_size, color, ..Default::default() }
        );
    }
}

struct VideoTimelineEditor {
    rect: Rect,
    click_rect: Rect,
    preview_percent: f32,
    font: Font,
    zero_measure: TextDimensions,
    fifty_measure: TextDimensions,
    hundred_measure: TextDimensions,
    timestamp_editors: Vec<VideoTimestampEditor>,
    add_button: Button,
    delete_button: Button,
    prev_selected_i: Option<usize>,
    /// if the user has changed the preview of the video in the frame
    changed_preview: bool,
}
impl VideoTimelineEditor {
    async fn new(visualiser: &Visualiser, timeline_rect: Rect, font: Font) -> VideoTimelineEditor {
        let start_x = screen_width()*PALETTEEDITOR_HOR_PADDING;
        let vert_padding = screen_height()*PALETTEEDITOR_VERT_PADDING;

        let font_size = (screen_width()*VIDEORECORDER_TIMELINE_FONT_PROPORTION) as u16;

        let button_size = screen_width()*PALETTEEDITOR_BUTTON_WIDTH;
        let button_border = screen_width()*PALETTEEDIOR_BUTTON_BORDER_WIDTH;
        let inner_button_size = button_size - 2.*button_border;
        let delete_button_start_x = screen_width()*(MENU_SCREEN_PROPORTION-PALETTEEDITOR_HOR_PADDING)-button_size;

        let add_rect = Rect::new(
            start_x,
            timeline_rect.bottom() + 2.*screen_width()*VIDEORECORDER_TIMESTAMP_WIDTH + 
                screen_height()*VIDEORECORDER_TIMELINE_VERT_PADDING + vert_padding,
            button_size, button_size
        );
        let mut delete_rect = add_rect.clone();
        delete_rect.x = delete_button_start_x;

        let mut click_rect = inflate_rect(&timeline_rect, timeline_rect.w/20.);
        click_rect.h += screen_height()*(VIDEORECORDER_TIMELINE_VERT_PADDING+VIDEORECORDER_TIMELINE_TEXT_PADDING) +
            screen_width()*(VIDEORECORDER_TIMELINE_FONT_PROPORTION+2.*VIDEORECORDER_TIMESTAMP_WIDTH);

        VideoTimelineEditor { 
            rect: timeline_rect.clone(), 
            click_rect,
            preview_percent: 0.0,
            font: font.clone(),
            zero_measure: measure_text("0%", Some(font), font_size, 1.),
            fifty_measure: measure_text("50%", Some(font), font_size, 1.),
            hundred_measure: measure_text("100%", Some(font), font_size, 1.),
            timestamp_editors: Vec::new(), 
            add_button: Button::gradient_border_and_image(
                visualiser, &add_rect, button_border, 
                load_image("assets/plus.png").await.unwrap(), DrawTextureParams::default(),
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ), 
            delete_button: Button::from_rect(
                &delete_rect, 
                vec![
                    Box::new(ButtonGradientElement::full_back(visualiser, None, &delete_rect, WHITE, 0)),
                    Box::new(ButtonColourElement::inner_from_border(&delete_rect, button_border, 1)),
                    Box::new(ButtonImageElement::new(
                        load_image("assets/bin.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        2
                    ))
                ], 
                vec![
                    Box::new(ButtonColourElement::inner_from_border(&delete_rect, button_border, 3)),
                    Box::new(ButtonImageElement::new(
                        load_image("assets/binOpen.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        4
                    )),
                    Box::new(ButtonColourElement::full_button(&delete_rect, HOVER_WHITE_OVERLAY, 5))
                ], 
                vec![Box::new(ButtonColourElement::full_button(&delete_rect, HOVER_BLACK_OVERLAY, 6))]
            ),
            prev_selected_i: None,
            changed_preview: false
        }
    }

    /// clears the timeline editor and creates a new one
    fn load(&mut self, recorder: &VideoRecorder) {
        self.timestamp_editors = Vec::with_capacity(recorder.timestamps.len());
        for timestamp in recorder.timestamps.iter() {
            self.timestamp_editors.push(VideoTimestampEditor::new(timestamp, &self.rect, self.font));
        } 
    }   

    fn add_timestamp(&mut self, visualiser: &mut Visualiser) {
        let mut new_timestamp = VideoTimestamp::new(&visualiser, 0.0);
        if visualiser.video_recorder.new_timestamp(&mut new_timestamp) {
            self.timestamp_editors.push(VideoTimestampEditor::new(&new_timestamp, &self.rect, self.font))
        }
    }

    fn delete_timestamp(&mut self, timestamp_i: usize, visualiser: &mut Visualiser) {
        if self.timestamp_editors.len() == 0 { return }
        visualiser.video_recorder.delete_timestamp(timestamp_i);
        self.timestamp_editors.remove(timestamp_i);
    }

    fn update_delete_button(&mut self, selected_i: Option<usize>, visualiser: &mut Visualiser) {
        let i = match selected_i {
            None => return,
            Some(index) => index
        };

        self.delete_button.update();   
        if !self.delete_button.clicked { return }

        self.delete_timestamp(i, visualiser);
    }

    fn render_current_percent(&mut self, visualiser: &mut Visualiser) {
        let timestamp = visualiser.video_recorder.get_timestamp_at_percent(self.preview_percent);
        let timestamp = match timestamp {
            None => return,
            Some(ts) => ts
        };

        visualiser.load_timestamp(&timestamp);
        visualiser.generate_image();
        self.changed_preview = true;
    }

    fn process_click(&mut self, visualiser: &mut Visualiser, selected: bool) {
        if !is_mouse_button_down(MouseButton::Left) || selected ||
           !self.click_rect.contains(mouse_position().into()) { return }

        let mx = mouse_position().0;
        let percent = (mx-self.rect.x) / self.rect.w;
        self.preview_percent = percent.clamp(0., 1.);

        self.render_current_percent(visualiser);
    }

    fn update(&mut self, visualiser: &mut Visualiser, preview_speed: f32) {
        self.draw();

        self.changed_preview = false;

        let mut selected_i: Option<usize> = None;
        for (i, timestampeditor) in self.timestamp_editors.iter_mut().enumerate() {
            if let Some(p) = timestampeditor.update(selected_i.is_some()) {
                visualiser.video_recorder.change_timestamp_percent(i, p);
            }
            if timestampeditor.percentage_editor.selected { selected_i = Some(i) }
        }

        if let Some(i) = selected_i {
            if self.prev_selected_i != selected_i && !visualiser.video_recorder.previewing {
                let this_timestamp = visualiser.video_recorder.timestamps[i].clone();
                visualiser.load_timestamp(&this_timestamp);
                visualiser.generate_image();
                self.changed_preview = true;
            }
        }

        if visualiser.video_recorder.previewing {
            self.preview_percent += preview_speed * get_frame_time();
            if self.preview_percent >= 1. {
                self.preview_percent -= 1.;
            }
            self.render_current_percent(visualiser);
        } else {
            self.process_click(visualiser, selected_i.is_some());
        }
        
        self.add_button.update();
        if self.add_button.clicked {
            self.add_timestamp(visualiser);
        }

        self.update_delete_button(selected_i, visualiser);

        self.prev_selected_i = selected_i;
    }

    fn draw(&self) {
        let line_width = screen_height() * VIDEORECORDER_TIMELINE_LINE_HEIGHT;
        let small_bar_height = screen_height() * VIDEORECORDER_TIMELINE_SMALL_BAR_HEIGHT;

        // main line
        draw_rectangle(self.rect.x, self.rect.center().y - line_width/2., self.rect.w, line_width, WHITE);

        // 0 and 100% bars
        draw_rectangle(self.rect.x, self.rect.y, line_width, self.rect.h, WHITE);
        draw_rectangle(self.rect.right()-line_width, self.rect.y, line_width, self.rect.h, WHITE);

        // 25, 50, and 75 bars
        for i in 0..3 {
            draw_rectangle(
                self.rect.x + self.rect.w/4. + i as f32 * self.rect.w/4. - line_width/2.,
                self.rect.center().y - small_bar_height/2.,
                line_width, small_bar_height,
                WHITE
            );
        }

        // percentages
        let font_size = (screen_width() * VIDEORECORDER_TIMELINE_FONT_PROPORTION) as u16;
        draw_text_ex(
            "0%", 
            self.rect.x - self.zero_measure.width/2. + line_width/2.,
            self.rect.y- screen_width()*VIDEORECORDER_TIMELINE_TEXT_PADDING,
            TextParams { font: self.font, font_size, color: WHITE, ..Default::default() }
        );
        draw_text_ex(
            "50%", 
            self.rect.center().x - self.fifty_measure.width/2.,
            self.rect.y- screen_width()*VIDEORECORDER_TIMELINE_TEXT_PADDING,
            TextParams { font: self.font, font_size, color: WHITE, ..Default::default() }
        );
        draw_text_ex(
            "100%", 
            self.rect.right() - self.hundred_measure.width/2. - line_width/2.,
            self.rect.y - screen_width()*VIDEORECORDER_TIMELINE_TEXT_PADDING,
            TextParams { font: self.font, font_size, color: WHITE, ..Default::default() }
        );

        // preview bar
        draw_rectangle(
            self.rect.x + self.rect.w * self.preview_percent - line_width/2., 
            self.rect.y,
            line_width, 
            self.rect.h, 
            VIDEORECORDER_TIMELINE_PREVIEW_BAR_COLOUR
        );
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        self.add_button.refresh_gradient(visualiser);
        self.delete_button.refresh_gradient(visualiser);
    }
}

struct VideoRecorderMenu {
    old_recorder: VideoRecorder,
    font: Font,
    title_back: Texture2D,
    inner_title_rect: Rect,
    title_text_measure: TextDimensions,
    title_text_colour: Color,
    timeline_editor: VideoTimelineEditor,
    bar_rect: Rect,
    bar_grad: Texture2D,
    play: Button,
    preview_seconds: TextBox,
    /// percentage increase per second
    preview_speed: f32,
    sumbit_button: Button,
    cancel_button: Button
}
impl VideoRecorderMenu {
    async fn new(visualiser: &Visualiser) -> VideoRecorderMenu {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();

        let title_rect = Rect::new(0., 0., 
            screen_width()*MENU_SCREEN_PROPORTION, 
            screen_height()*(2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION);
        let title_back = get_back_gradient(visualiser, 0, title_rect.w as u16, title_rect.h as u16);
            
        let start_x = screen_width()*PALETTEEDITOR_HOR_PADDING;
        let vert_padding = screen_height()*PALETTEEDITOR_VERT_PADDING;

        let timeline_rect = Rect::new(
            start_x, title_rect.h + vert_padding + screen_height()*VIDEORECORDER_TIMELINE_VERT_PADDING, 
            screen_width()*(MENU_SCREEN_PROPORTION-2.*PALETTEEDITOR_HOR_PADDING),
            screen_height()*VIDEORECORDER_TIMELINE_HEIGHT
        );

        let button_size = screen_width()*PALETTEEDITOR_BUTTON_WIDTH;
        let button_border = screen_width()*PALETTEEDIOR_BUTTON_BORDER_WIDTH;

        let bar_bottom = title_rect.h + 
            vert_padding*3. + 
            screen_height()*(VIDEORECORDER_TIMELINE_HEIGHT+2.*VIDEORECORDER_TIMELINE_VERT_PADDING+PALETTEEDITOR_BAR_HEIGHT) +
            screen_width()*(2.*VIDEORECORDER_TIMESTAMP_WIDTH+PALETTEEDITOR_BUTTON_WIDTH);
        let bar_rect = Rect::new(
            0., 
            bar_bottom-screen_height()*PALETTEEDITOR_BAR_HEIGHT, 
            screen_width()*MENU_SCREEN_PROPORTION,
            screen_height()*PALETTEEDITOR_BAR_HEIGHT
        );

        let play_rect = Rect::new(
            start_x, bar_bottom + vert_padding,
            button_size, button_size
        );
        let preview_input_box = GradientInputBox::new(
            visualiser, 
            screen_width()*VIDEORECORDER_TEXTBOX_START_X, 
            play_rect.bottom() + vert_padding, 
            screen_width()*(MENU_SCREEN_PROPORTION-VIDEORECORDER_TEXTBOX_START_X-PALETTEEDITOR_HOR_PADDING),
            screen_height()*DEFAULT_INPUT_BOX_HEIGHT,
            screen_height()*DEFAULT_INPUT_BOX_BORDER_SIZE
        );

        let sumbit_rect = Rect::new(
            screen_width()*(MENU_SCREEN_PROPORTION/2. - PALETTEEDITOR_HOR_PADDING/2.) - button_size,
            screen_height() - button_size - vert_padding,
            button_size, button_size
        );
        let mut cancel_rect = sumbit_rect.clone();
        cancel_rect.x = screen_width()*(MENU_SCREEN_PROPORTION/2. + PALETTEEDITOR_HOR_PADDING/2.);

        VideoRecorderMenu { 
            old_recorder: visualiser.video_recorder.clone(),
            font,
            title_back,
            inner_title_rect: inflate_rect(&title_rect, -screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION),
            title_text_measure: measure_text(
                "VIDEO RECORDER",
                Some(font),
                (screen_width()*STATE_TEXT_FONT_PROPORTION) as u16,
                1.0
            ),
            title_text_colour: get_brightest_colour(title_back),
            timeline_editor: VideoTimelineEditor::new(visualiser, timeline_rect, font).await,
            bar_rect, 
            bar_grad: get_back_gradient(visualiser, bar_rect.x as u16, bar_rect.w as u16, bar_rect.h as u16),
            play: Button::gradient_border_and_image(visualiser, &play_rect, button_border, 
                load_image("assets/next.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            preview_seconds: TextBox::new(
                preview_input_box, 
                InputLabel::default_input_box_label(visualiser, font, "preview seconds", true),
                InputLabel::default_input_box_content(font),
                "5"
            ),
            preview_speed: 0.0,
            sumbit_button: Button::gradient_border_and_image(visualiser, &sumbit_rect, button_border, 
                load_image("assets/tick.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            cancel_button: Button::gradient_border_and_image(visualiser, &cancel_rect, button_border, 
                load_image("assets/cross.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
        }
    }

    fn draw_title(&self) {
        draw_texture(self.title_back, 0., 0., WHITE);
        draw_rect(&self.inner_title_rect, BLACK);
        draw_text_ex(
            "VIDEO RECORDER",
            self.inner_title_rect.center().x - self.title_text_measure.width/2.,
            self.inner_title_rect.center().y + self.title_text_measure.height/2.,
            TextParams { 
                font: self.font, 
                font_size: (screen_width()*STATE_TEXT_FONT_PROPORTION) as u16, 
                color: self.title_text_colour, 
                ..Default::default() 
            }
        );
    }
}
impl MenuType for VideoRecorderMenu {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal {
        self.draw_title();

        self.timeline_editor.update(visualiser, self.preview_speed);
        if self.timeline_editor.changed_preview { self.refresh_gradients(visualiser) }

        draw_texture(self.bar_grad, self.bar_rect.x, self.bar_rect.y, WHITE);

        self.play.update();
        if self.play.clicked {
            visualiser.video_recorder.previewing = !visualiser.video_recorder.previewing;
            if visualiser.video_recorder.previewing {
                self.preview_speed = 1. / self.preview_seconds.data.parse::<f32>().unwrap();
            }
        }

        if let Some(Ok(new)) = self.preview_seconds.update(self.preview_seconds.data.clone())
                                      .and_then(|s| Some(s.parse::<usize>())) {
            if new > 0 { self.preview_seconds.data = new.to_string() }
        }

        self.sumbit_button.update();
        if self.sumbit_button.clicked {
            visualiser.video_recorder.previewing = false;
            // return MenuSignal::RefreshGradients;
            return MenuSignal::Import;
        }

        self.cancel_button.update();
        if self.cancel_button.clicked {
            visualiser.video_recorder = self.old_recorder.clone();
            self.timeline_editor.load(&visualiser.video_recorder);
            visualiser.video_recorder.previewing = false;
            // return MenuSignal::RefreshGradients;
            return MenuSignal::Import;
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        self.preview_seconds.selected
    }

    fn open_layer_to_edit(&mut self, _index: usize, visualiser: &Visualiser) {
        self.old_recorder = visualiser.video_recorder.clone();
        self.timeline_editor.load(&visualiser.video_recorder);
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        Texture2D::delete(&self.title_back);
        Texture2D::delete(&self.bar_grad);

        let vert_padding = screen_width()*PALETTEEDITOR_VERT_PADDING;
        let title_rect = Rect::new(0., 0., 
            screen_width()*MENU_SCREEN_PROPORTION, 
            screen_height()*(2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION);
        self.title_back = get_back_gradient(visualiser, 0, title_rect.w as u16, title_rect.h as u16);

        let bar_bottom = title_rect.h + 
            vert_padding*3. + 
            screen_height()*(VIDEORECORDER_TIMELINE_HEIGHT+2.*VIDEORECORDER_TIMELINE_VERT_PADDING+PALETTEEDITOR_BAR_HEIGHT) +
            screen_width()*(2.*VIDEORECORDER_TIMESTAMP_WIDTH+PALETTEEDITOR_BUTTON_WIDTH);
        let bar_rect = Rect::new(
            0., 
            bar_bottom-screen_height()*PALETTEEDITOR_BAR_HEIGHT, 
            screen_width()*MENU_SCREEN_PROPORTION,
            screen_height()*PALETTEEDITOR_BAR_HEIGHT
        );
        self.bar_grad = get_back_gradient(visualiser, bar_rect.x as u16, bar_rect.w as u16, bar_rect.h as u16);

        self.timeline_editor.refresh_gradients(visualiser);

        self.play.refresh_gradient(visualiser);
        self.preview_seconds.refresh_gradient(visualiser);
        self.sumbit_button.refresh_gradient(visualiser);
        self.cancel_button.refresh_gradient(visualiser);
    }
}

pub struct LeaveMenu {
    leave_button: Button,
    close_button: Button,
    input_box: GradientInputBox,
    mandelbrot: Button,
    julia: Button,
    exit: Button,
    open: bool
}
impl LeaveMenu {
    async fn new(visualiser: &Visualiser) -> LeaveMenu {
        let leave_rect = Rect::new(
            screen_width() - screen_height()*NAVBAR_HEIGHT_PROPORTION,
            0., 
            screen_height()*NAVBAR_HEIGHT_PROPORTION,
            screen_height()*NAVBAR_HEIGHT_PROPORTION
        );

        let border_size = screen_width()*LEAVEMENU_BORDER_SIZE;

        let outer_rect = Rect::new(
            0.5*screen_width()*(1.-LEAVEMENU_WIDTH),
            0.5*screen_height()*(1.-LEAVEMENU_HEIGHT),
            screen_width()*LEAVEMENU_WIDTH,
            screen_height()*LEAVEMENU_HEIGHT
        );
        let input_box = GradientInputBox::from_outer_rect(visualiser, outer_rect, border_size);

        let mandelbrot_rect = Rect::new(
            outer_rect.center().x - 0.5*screen_width()*LEAVEMENU_OPTION_WIDTH,
            outer_rect.y + screen_height()*LEAVEMENU_VERT_PADDING,
            screen_width()*LEAVEMENU_OPTION_WIDTH,
            screen_height()*LEAVEMENU_OPTION_HEIGHT
        );
        let mut julia_rect = mandelbrot_rect.clone();
        julia_rect.y += screen_height()*(LEAVEMENU_OPTION_HEIGHT+LEAVEMENU_VERT_PADDING);

        let exit_rect = Rect::new(
            outer_rect.center().x - 0.5*screen_width()*LEAVEMENU_EXIT_SIZE,
            outer_rect.bottom() - screen_height()*LEAVEMENU_VERT_PADDING - screen_width()*LEAVEMENU_EXIT_SIZE,
            screen_width()*LEAVEMENU_EXIT_SIZE, screen_width()*LEAVEMENU_EXIT_SIZE
        );

        LeaveMenu {
            leave_button: Button::gradient_border_and_image(
                visualiser, &leave_rect, screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION, 
                load_image("assets/door.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            close_button: Button::gradient_border_and_image(
                visualiser, &leave_rect, screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION, 
                load_image("assets/cross.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            input_box,
            mandelbrot: LeaveMenu::get_option_button(visualiser, &mandelbrot_rect, "mandelbrot").await,
            julia: LeaveMenu::get_option_button(visualiser, &julia_rect, "julia").await,
            exit: Button::gradient_border_and_image(
                visualiser, &exit_rect, border_size, 
                load_image("assets/door.png").await.unwrap(), DrawTextureParams::default(), 
                HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
            ),
            open: false
        }
    }

    async fn get_option_button(visualiser: &Visualiser, rect: &Rect, text: &str) -> Button {
        let border_size = screen_width()*LEAVEMENU_BORDER_SIZE;

        Button::gradient_border_and_text(
            visualiser, &rect, border_size, 
            text, load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap(), 
            screen_width()*LEAVEMENU_OPTION_FONT_PROPORTION, 0., TextAlign::Centre, 
            HOVER_WHITE_OVERLAY, HOVER_BLACK_OVERLAY
        )
    }

    fn refresh_gradients(&mut self, visualiser: &Visualiser) {
        self.leave_button.refresh_gradient(visualiser);
        self.input_box.refresh_gradient(visualiser);
        self.mandelbrot.refresh_gradient(visualiser);
        self.julia.refresh_gradient(visualiser);
        self.exit.refresh_gradient(visualiser);
    }

    pub fn update(&mut self, running: &mut bool, visualiser: &mut Visualiser) {
        if !self.open { 
            self.leave_button.update();
            if self.leave_button.clicked {
                self.open = true;
            } else {
                return;
            }
        }

        draw_rect(
            &Rect::new(0., 0., screen_width(), screen_height()),
            LEAVEMENU_BACK_OVERLAY
        );

        self.close_button.update();
        if self.close_button.clicked {
            self.open = false;
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) && !self.input_box.outer_rect().contains(mouse_position().into()) {
            self.open = false;
            return;
        }

        self.input_box.draw(None);

        if visualiser.fractal.is_mandelbrot() {
            self.mandelbrot.hovering = true;
            self.mandelbrot.holding = true;
            self.mandelbrot.draw();
        } else {
            self.mandelbrot.update();
            if self.mandelbrot.clicked {
                visualiser.center = ComplexType::Double(Complex::new(-0.5, 0.));
                visualiser.set_fractal(Fractal::Mandelbrot);
                self.open = false;
            }
        }
    
        if visualiser.fractal.is_julia() {
            self.julia.hovering = true;
            self.julia.holding = true;
            self.julia.draw()
        } else {
            self.julia.update();
            if self.julia.clicked {
                visualiser.center = ComplexType::Double(Complex::new(0., 0.));
                visualiser.set_fractal(Fractal::Julia(JuliaSeed::new(0., 0.)));
                self.open = false;
            }
        }

        self.exit.update();
        if self.exit.clicked {
            *running = false;
        }
    }
}