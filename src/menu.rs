// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use super::{
    ScreenDimensions, Visualiser, interpolate_colour, 
    layers::*,
    orbit_trap::*,
    palettes::*
};

/// the proportion of the screen width taken over by the menu
const MENU_SCREEN_PROPORTION: f32 = 0.25;

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

/// proportion of the screen width for the size of the menu text
const TEXTBOX_LABEL_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen width for the padding before and after the text box label
const TEXTBOX_LABEL_WIDTH_PADDING_PROPORTION: f32 = MENU_SCREEN_PROPORTION/100.;
/// proportion of the screen height for the height of the text boxes
const TEXTBOX_HEIGHT_PROPORTION: f32 = 1./20.;
/// proportion of the screen height for the y value of the the first text box
const TEXTBOX_START_Y_PROPORTION: f32 = 0.2;
/// proportion of the screen height for the padding between text boxes
const TEXTBOX_HEIGHT_PADDING_PROPORTION: f32 = TEXTBOX_HEIGHT_PROPORTION/4.;
/// proportion of the screen height for the border of text boxes
const TEXTBOX_BORDER_PROPORTION: f32 = TEXTBOX_HEIGHT_PROPORTION/20.;
/// proportion of the screen width for the padding on the right of text boxes
const TEXTBOX_RIGHT_PADDING: f32 = MENU_SCREEN_PROPORTION/50.;
/// proportion of the screen width for the padding between the text box and content inside
const TEXTBOX_CONTENT_PADDING: f32 = MENU_SCREEN_PROPORTION/100.;
/// time between blinks of the text box cursor
const TEXTBOX_CURSOR_BLINK_TIME: f32 = 0.5;

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
/// proportion of the screen height for the size of the exit button
const LAYERMANAGER_DELETE_BUTTON_SIZE: f32 = LAYERMANAGER_HEIGHT/6.;

/// proportion of the screen height for the height of the layer carousel
const LAYEREDITOR_CAROUSEL_HEIGHT: f32 = 1./15.;
/// proportion of the screen width for the size of the layer carousel
const LAYEREDITOR_CAROUSEL_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/15.;
/// proportion of the screen width for the size of the layer carousel
const LAYEREDITOR_INPUT_FONT_PROPORTION: f32 = MENU_SCREEN_PROPORTION/20.;
/// proportion of the screen width for the start of the text boxes in the layer editor menu
const LAYEREDITOR_TEXTBOX_START_X: f32 = MENU_SCREEN_PROPORTION*0.3;
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
/// proportion of the scree nheight for the bar in the palette editor
const PALETTEEDITOR_BAR_HEIGHT: f32 = NAVBAR_BORDER_WIDTH_PROPORTION;
/// proportion of the screen width for the width of the mapping type dropdown
const PALETTEEDITOR_MAPPING_DROPDOWN_WIDTH: f32 = PALETTEEDITOR_TEXTBOX_WIDTH*2.2;

/// gives a texture which is a snippet of the gradient for the menu at the given place
fn get_back_gradient(visualiser: &Visualiser, start_x: u16, width: u16, height: u16) -> Texture2D {
    let mut image = Image::gen_image_color(width, height, BLACK);

    for i in 0..width {
        let percent = ((start_x + i) as f32) / (screen_width()*MENU_SCREEN_PROPORTION);
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
/// being stretched by a scale factor of scale at each border
fn inflate_rect(rect: &Rect, scale: f32) -> Rect {
    Rect::new(
        rect.x - scale,
        rect.y - scale,
        rect.w + 2. * scale,
        rect.h + 2. * scale
    )
}

enum MenuSignal {
    None,
    OpenEditor(usize),
    OpenPalette(usize),
    RefreshGradients
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
    UpdateGradient
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

    fn map_state_indexes(&self) -> usize {
        match self {
            MenuState::General => 0,
            MenuState::Layers => 1,
            MenuState::LayerEditor => 2,
            MenuState::Screenshot => 3,
            MenuState::Video => 4,
            MenuState::PaletteEditor => 5,
            MenuState::Closed => 5,
            MenuState::UpdateGradient => 6
        }
    }

    fn get_string(&self) -> String {
        String::from(match self {
            MenuState::General => "GENERAL",
            MenuState::Layers => "LAYERS",
            MenuState::LayerEditor => "LAYER EDITOR",
            MenuState::Screenshot => "SCREENSHOT",
            MenuState::Video => "VIDEO",
            MenuState::PaletteEditor => "PALETTE EDITOR",
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

    async fn create_menu(&self, visualiser: &mut Visualiser, index: usize, font: Font) -> Option<Box<dyn MenuType>> {
        match index {
            0 => Some(Box::new(GeneralMenu::new(visualiser, font))),
            1 => Some(Box::new(LayersMenu::new(visualiser, font).await)),
            2 => Some(Box::new(LayerEditorMenu::new(visualiser).await)),
            5 => Some(Box::new(PaletteEditor::new(&visualiser).await)),
            // placeholder
            _ => None
        }
    }

    async fn process_signal(
        &mut self, 
        menus: &mut [Option<Box<dyn MenuType>>; 6], 
        visualiser: &mut Visualiser, 
        signal: MenuSignal, 
        font: Font
    ) {
        match signal {
            MenuSignal::None => {},
            MenuSignal::OpenEditor(index) => {
                if menus[2].is_none() {
                    menus[2] = self.create_menu(visualiser, 2, font).await
                }
                match &mut menus[2] {
                    None => panic!("layer editor menu failed to be created"),
                    Some(m) => m.as_mut().open_layer_to_edit(index, &visualiser)
                }
                *self = MenuState::LayerEditor;
            },
            MenuSignal::OpenPalette(index) => {
                if menus[5].is_none() {
                    menus[5] = self.create_menu(visualiser, 5, font).await
                }
                match &mut menus[5] {
                    None => panic!("palette editor menu failed to be created"),
                    Some(m) => m.as_mut().open_layer_to_edit(index, &visualiser)
                }
                *self = MenuState::PaletteEditor;
            },
            MenuSignal::RefreshGradients => {
                for menu in menus {
                    *menu = None;
                }
                *self = MenuState::UpdateGradient;
            }
        }
    }

    async fn update_state_menu(&mut self, menus: &mut [Option<Box<dyn MenuType>>; 6], visualiser: &mut Visualiser, index: usize, font: Font) {
        match &mut menus[index] {
            None => {
                menus[index] = self.create_menu(visualiser, index, font).await;
            },
            Some(m) => {
                let signal = m.as_mut().update(visualiser);
                self.process_signal(menus, visualiser, signal, font).await;
            }
        }
    }

    /// updates the menu for the current state
    async fn update_state(&mut self, menus: &mut [Option<Box<dyn MenuType>>; 6], visualiser: &mut Visualiser, font: Font) {
        self.update_state_menu(menus, visualiser, self.map_state_indexes(), font).await;
    }

    fn get_editing_menu(&self, menus: &mut [Option<Box<dyn MenuType>>; 6], index: usize) -> bool {
        match &mut menus[index] {
            None => {false},
            Some(m) => m.as_mut().get_editing()
        }
    }

    fn get_editing(&self, menus: &mut [Option<Box<dyn MenuType>>; 6]) -> bool {
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
    text_font: Font,
    open_button: Button,
    close_button: Button,
    navbar: Navbar,
    menus: [Option<Box<dyn MenuType>>; 6],
    updated_gradient: bool
}
impl Menu {
    pub async fn new() -> Menu {
        Menu { 
            state: MenuState::Closed,
            visualiser_menu_size: ScreenDimensions::new(
                ((1.0-MENU_SCREEN_PROPORTION)*screen_width()) as usize,
                screen_height() as usize
            ),
            gradient: Texture2D::empty(),
            text_colour: BLACK,
            state_font: load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap(),
            text_font: load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap(),
            open_button: Button::new(
                (20., 20.),
                (0., 0.),
                vec![
                    Box::new(ButtonColourElement::new(PINK, (20., 20.), (0., 0.), 0)),
                    Box::new(ButtonColourElement::new(BLACK, (15., 15.), (2.5, 2.5), 3)),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/triangle.png").await.unwrap(), 1.,
                        DrawTextureParams { dest_size: Some(Vec2::new(15., 15.)), flip_x: true, ..Default::default() },
                        (2.5, 2.5), 4
                    ))
                ],
                vec![Box::new(ButtonColourElement::new(WHITE, (20., 20.), (0., 0.), 1))],
                vec![Box::new(ButtonColourElement::new(BLACK, (20., 20.), (0., 0.), 2))]
            ),
            close_button: Button::new(
                (20., 20.),
                (MENU_SCREEN_PROPORTION*screen_width(), 0.),
                vec![
                    Box::new(ButtonColourElement::new(PINK, (20., 20.), (0., 0.), 0)),
                    Box::new(ButtonColourElement::new(BLACK, (15., 15.), (2.5, 2.5), 3)),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/triangle.png").await.unwrap(), 1.,
                        DrawTextureParams { dest_size: Some(Vec2::new(15., 15.)), flip_x: false, ..Default::default() },
                        (2.5, 2.5), 4
                    ))
                ],
                vec![Box::new(ButtonColourElement::new(WHITE, (20., 20.), (0., 0.), 1))],
                vec![Box::new(ButtonColourElement::new(BLACK, (20., 20.), (0., 0.), 2))]
            ),
            navbar: Navbar::new().await,
            menus: [None, None, None, None, None, None],
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
        self.gradient = get_back_gradient(
            &visualiser, 
            0, 
            (MENU_SCREEN_PROPORTION*screen_width()) as u16, 
            (screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) +
                    screen_width()*STATE_TEXT_FONT_PROPORTION) as u16
        );
        self.navbar.back = self.gradient;
        self.text_colour = get_brightest_colour(self.gradient);
        self.updated_gradient = true;
    }

    /// updates the menu every frame
    pub async fn update(&mut self, visualiser: &mut Visualiser) {
        if self.state == MenuState::Closed {
            self.menu_state_closed(visualiser);
            return;
        }
        
        if !self.updated_gradient {
            self.update_gradient(visualiser);
        }

        self.state.update_state(&mut self.menus, visualiser, self.text_font).await;
        if self.state == MenuState::UpdateGradient {
            self.update_gradient(visualiser);
            self.state = MenuState::General;
            return;
        }

        self.state = self.navbar.update(self.state, self.state_font, self.text_colour);

        if self.state == MenuState::PaletteEditor { return }

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
    params: Option<DrawTextureParams>,
    alpha_colour: Color,
    /// offset from the topleft of the button
    offset: (f32, f32),
    draw_order: usize
}
impl ButtonImageElement {
    fn from_texture(image: Texture2D, alpha: f32, offset: (f32, f32), draw_order: usize) -> ButtonImageElement {
        ButtonImageElement { image, alpha_colour: Color::new(1., 1., 1., alpha), params: None, offset, draw_order }
    }

    fn from_image(image: Image, alpha: f32, params: DrawTextureParams, offset: (f32, f32), draw_order: usize) -> ButtonImageElement {
        ButtonImageElement { 
            image: Texture2D::from_image(&image), 
            alpha_colour: Color::new(1., 1., 1., alpha),
            params: Some(params), 
            offset, 
            draw_order
        }
    }
}
impl ButtonElement for ButtonImageElement {
    fn draw(&self, button_rect: &Rect) {
        match &self.params {
            Some(p) => draw_texture_ex(
                self.image, 
                button_rect.x+self.offset.0, 
                button_rect.y+self.offset.1, 
                self.alpha_colour, 
                p.clone()
            ),
            None => draw_texture(
                self.image, 
                button_rect.x+self.offset.0, 
                button_rect.y+self.offset.1, 
                self.alpha_colour
            )
        }
    }

    fn get_draw_order(&self) -> usize {
        self.draw_order
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
        Button { 
            rect: Rect::new(topleft.0, topleft.1, size.0, size.1),
            back_elements, hover_elements, hold_elements,
            clicked: false, hovering: false, holding: false
        }
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
        if menu_state == MenuState::PaletteEditor {
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

#[derive(Clone)]
struct DataInfo {
    content: String,
    content_dims: TextDimensions,
    content_params: TextParams,
    letters: usize,
    letter_width: f32
}
impl DataInfo {
    fn new(data: &str, content_params: TextParams) -> DataInfo {
        let content = data.to_owned();
        let content_dims = measure_text(&content, Some(content_params.font), 
            content_params.font_size, 1.0);
        let letters = content.chars().count();
        let letter_width = content_dims.width / letters as f32;
        DataInfo { content, content_params, content_dims, letters, letter_width }
    }

    fn remeasure(&mut self) {
        self.content_dims = measure_text(&self.content, Some(self.content_params.font), 
            self.content_params.font_size, 1.0);
        self.letters = self.content.chars().count();
        self.letter_width = self.content_dims.width / self.letters as f32;
    }
}

#[derive(Clone)]
/// a label for an input field
struct InputLabel {
    text: String,
    label_dims: TextDimensions,
    label_params: TextParams
}
impl InputLabel {
    fn new(text: &str, font: Font, font_size: f32, color: Color) -> InputLabel {
        let params = TextParams { font, font_size: font_size as u16,  color, ..Default::default()};
        let measure = measure_text(
            text, 
            Some(font),
            params.font_size,
            params.font_scale
        );

        InputLabel { 
            text: String::from(text), 
            label_dims: measure,
            label_params: params 
        }
    }
}

#[derive(Clone)]
struct TextBox {
    label: Option<InputLabel>,
    data: String,
    data_info: DataInfo,
    border_back: Texture2D,
    outer_rect: Rect,
    inner_rect: Rect,
    content_params: TextParams,
    selected: bool,
    selected_shade: Color,
    start_pos: usize,
    cursor_pos: usize,
    cursor_visible: bool,
    cursor_blink_timer: f32
}
impl TextBox {
    fn new(
        label: Option<InputLabel>,
        default_data: String,
        width: f32, 
        height: f32,
        start_x: u16, start_y: f32, 
        gradient: Texture2D,
        content_params: TextParams
    ) -> TextBox {
        let outer_rect = Rect::new(start_x as f32, start_y, width, height);
        let border_width = screen_height() * TEXTBOX_BORDER_PROPORTION;
        TextBox {
            label, 
            data: default_data.to_owned(),
            data_info: DataInfo::new(&default_data, content_params),
            border_back: gradient,
            outer_rect,
            inner_rect: inflate_rect(&outer_rect, -border_width),
            content_params,
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
            self.data = data.to_owned();
            self.data_info = DataInfo::new(&data, self.content_params);
            self.start_pos = 0;
        }

        self.check_clicked();
        let output = self.keyboard_entry();
        self.update_cursor();

        self.draw();

        output
    }

    fn draw(&self) {
        if let Some(label) = self.label.clone() {
            draw_text_ex(
                &label.text, 
                screen_width()*TEXTBOX_LABEL_WIDTH_PADDING_PROPORTION, 
                self.outer_rect.y + self.outer_rect.h/2. + label.label_dims.height/2., 
                label.label_params,
            );
        }
        draw_texture(self.border_back, self.outer_rect.x, self.outer_rect.y, WHITE);
        if self.selected {
            draw_rectangle(self.outer_rect.x, self.outer_rect.y, self.outer_rect.w, self.outer_rect.h, self.selected_shade);
        }
        draw_rectangle(self.inner_rect.x, self.inner_rect.y, self.inner_rect.w, self.inner_rect.h, BLACK);

        draw_text_ex(
            &self.data_info.content[self.start_pos..self.start_pos+self.get_to_use()],
            self.inner_rect.x + screen_width()*TEXTBOX_CONTENT_PADDING,
            self.inner_rect.y + self.inner_rect.h/2. + self.data_info.content_dims.height/2.,
            self.content_params
        );
        if self.selected && self.cursor_visible {
            draw_rectangle(
                self.get_cursor_x(),
                self.inner_rect.y + self.inner_rect.h / 10.,
                2.0,
                self.inner_rect.h - self.inner_rect.h / 5.,
                WHITE
            );
        }
    }

    fn get_to_use(&self) -> usize {
        let to_end = self.data_info.letters - self.start_pos;
        for i in (0..=to_end).rev() {
            let measure = measure_text(
                &self.data_info.content[self.start_pos..self.start_pos+i], 
                Some(self.data_info.content_params.font), 
                self.data_info.content_params.font_size, 
                1.0
            );
            if measure.width < self.inner_rect.w - 2.*screen_width()*TEXTBOX_CONTENT_PADDING {
                return i;
            }
        }
        0
    }

    fn get_cursor_x(&self) -> f32 {
        self.inner_rect.x + measure_text(
            &self.data_info.content[self.start_pos..self.cursor_pos],
            Some(self.data_info.content_params.font), 
            self.data_info.content_params.font_size, 
            1.0
        ).width + self.data_info.letter_width / 3.
    }

    fn check_clicked(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) { return }
        if !self.outer_rect.contains(Vec2::from(mouse_position())) { 
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
            if mouse_position().0 - self.inner_rect.x < measure.width {
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

    fn translate(&mut self, translate: (f32, f32)) {
        translate_rect(&mut self.outer_rect, translate);
        translate_rect(&mut self.inner_rect, translate);
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
        // draw label
        if let Some(label) = self.label.clone() {
            draw_text_ex(
                &label.text, 
                self.inflated_rect.x - label.label_dims.width * 1.1 - self.rect.h/2., 
                self.rect.center().y + label.label_dims.height/2.,
                label.label_params,
            );
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
        if let Some(percentage_params) = self.percentage_label_params.clone() {
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
                percentage_params
            );
        }
        // draw text box
        if let Some(textbox) = self.percentage_text_box.clone() {
            textbox.draw();
        }
    }

    fn translate(&mut self, translate: (f32, f32)) {
        translate_rect(&mut self.rect, translate);
        translate_rect(&mut self.inflated_rect, translate);
    }
}

pub trait DropDownType<T> {
    fn get_variants() -> Vec<T>;
    fn get_string(&self) -> String;
}

#[derive(Clone)]
struct DropDown<T: DropDownType<T> + std::cmp::PartialEq + Clone> {
    variants: Vec<T>,
    variant_font: Font,
    variant_text_params: TextParams,
    /// rect to contain the currently selected variant
    closed_rect: Rect,
    closed_back: Texture2D,
    /// rect to contain the extra variants
    open_rect: Rect,
    open_back: Texture2D,
    border_size: f32,
    label: Option<InputLabel>,
    arrow_image: Texture2D,
    open: bool,
    hovering: bool,
    hover_index: usize
}
impl<T: DropDownType<T> + std::cmp::PartialEq + Clone> DropDown<T> {
    async fn new(
        visualiser: &Visualiser, 
        variant_font_size: u16,
        closed_topleft: (f32, f32), 
        closed_size: (f32, f32), 
        border_size: f32,
        label: Option<InputLabel>
    ) -> DropDown<T> {
        let closed_rect = Rect::new(closed_topleft.0, closed_topleft.1, closed_size.0, closed_size.1);

        let variants = T::get_variants();

        let variant_extension = closed_size.1 - border_size;
        let extra_height = (variants.len()-1) as f32 * variant_extension;
        let open_rect = Rect::new(
            closed_topleft.0,
            closed_topleft.1 + if closed_rect.bottom() + extra_height <= screen_height() 
                { variant_extension } else { -extra_height },
            closed_size.0,
            extra_height + border_size
        );

        let variant_font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        let variant_text_params = TextParams { font: variant_font, 
            font_size: variant_font_size, color: WHITE, ..Default::default()};

        DropDown { variants, closed_rect, open_rect, border_size, variant_font, variant_text_params, label,
            closed_back: get_back_gradient(visualiser, closed_rect.x as u16, 
                            closed_rect.w as u16, closed_rect.h as u16),
            open_back: get_back_gradient(visualiser, open_rect.x as u16, 
                            open_rect.w as u16, open_rect.h as u16),
            arrow_image: Texture2D::from_image(&load_image("assets/down.png").await.unwrap()),
            open: false, hovering: false, hover_index: 0
        }
    }

    /// updates + draws the dropdown 
    /// 
    /// # Returns
    /// None if the value wasn't changed
    /// Some(T) if changed
    fn update(&mut self, current_variant: &T) -> Option<T> {
        self.draw(current_variant);
        if self.open {
            self.interact_open(current_variant)
        } else {
            self.interact_closed();
            None
        }
    }

    fn interact_closed(&mut self) {
        if !self.closed_rect.contains(Vec2::from(mouse_position())) {
            self.hovering = false;
            return;
        }

        self.hovering = true;
        if is_mouse_button_pressed(MouseButton::Left) {
            self.open = true;
        }
    }

    fn interact_open(&mut self, current_variant: &T) -> Option<T> {
        let closed_contain = self.closed_rect.contains(Vec2::from(mouse_position()));
        let open_contain = self.open_rect.contains(Vec2::from(mouse_position()));
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
            self.hover_index = (((mouse_position().1 - self.open_rect.y).abs() / self.open_rect.h) *
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
        // draw closed
        draw_texture(self.closed_back, self.closed_rect.x, self.closed_rect.y, WHITE);
        let closed_inner_height = self.closed_rect.h - 2.*self.border_size;
        draw_rectangle(
            self.closed_rect.x + self.border_size, 
            self.closed_rect.y + self.border_size, 
            self.closed_rect.w - 2.*self.border_size, 
            closed_inner_height, 
            BLACK
        );
        let current_string = current_variant.get_string();
        let measure = measure_text(
            &current_string, 
            Some(self.variant_font), 
            self.variant_text_params.font_size, self.variant_text_params.font_scale
        );
        let text_x = self.closed_rect.x + self.border_size + self.closed_rect.w/100.;
        draw_text_ex(
            &current_string, 
            text_x, 
            self.closed_rect.y + self.border_size + self.closed_rect.h/2. + measure.height*0.25, 
            self.variant_text_params
        );
        if !self.open { 
            draw_texture_ex(
                self.arrow_image, 
                self.closed_rect.right() - self.border_size - closed_inner_height, 
                self.closed_rect.y + self.border_size, 
                WHITE, 
                DrawTextureParams { 
                    dest_size: Some(Vec2::new(closed_inner_height, closed_inner_height)), 
                    ..Default::default() 
                }
            );
        }
        if self.hovering && (!self.open || self.hover_index == 0) {
            draw_rectangle(self.closed_rect.x, self.closed_rect.y, 
                self.closed_rect.w, self.closed_rect.h, HOVER_WHITE_OVERLAY);
        }

        if let Some(label) = self.label.clone() {
            draw_text_ex(
                &label.text, 
                self.closed_rect.x - label.label_dims.width - screen_width()*TEXTBOX_LABEL_WIDTH_PADDING_PROPORTION, 
                self.closed_rect.y + self.closed_rect.h/2. + label.label_dims.height/2., 
                label.label_params,
            );
        }

        if !self.open { return }

        // draw open
        draw_texture_ex(
            self.arrow_image, 
            self.closed_rect.right() - self.border_size - closed_inner_height, 
            self.closed_rect.y + self.border_size, 
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(Vec2::new(closed_inner_height, closed_inner_height)), 
                flip_y: true,
                ..Default::default() 
            }
        );

        draw_texture(self.open_back, self.open_rect.x, self.open_rect.y, WHITE);
        let non_current: Vec<&T> = self.variants.iter().filter(|x| **x != *current_variant).collect();
        for i in 0..non_current.len() {
            let container = Rect::new(
                self.open_rect.x + self.border_size, 
                self.open_rect.y + self.border_size + i as f32 * (self.closed_rect.h - self.border_size), 
                self.open_rect.w - 2.*self.border_size, 
                self.closed_rect.h - 2.*self.border_size, 
            );
            draw_rectangle(container.x, container.y, container.w, container.h, BLACK);
            
            let variant_string = non_current[i].get_string();
            let measure = measure_text(
                &variant_string, 
                Some(self.variant_font), 
                self.variant_text_params.font_size, self.variant_text_params.font_scale
            );
            draw_text_ex(
                &variant_string, 
                text_x,
                container.y + container.h/2. + measure.height/2., 
                self.variant_text_params
            );
            if self.hovering && i+1 == self.hover_index {
                draw_rectangle(
                    self.open_rect.x, 
                    self.open_rect.y + i as f32 * (self.closed_rect.h - self.border_size), 
                    self.open_rect.w, 
                    self.closed_rect.h, 
                    HOVER_WHITE_OVERLAY
                );
            }
        }
    }

    fn translate(&mut self, translate: (f32, f32)) {
        translate_rect(&mut self.closed_rect, translate);

        let variant_extension = self.closed_rect.h - self.border_size;
        let extra_height = (self.variants.len()-1) as f32 * variant_extension;
        self.open_rect = Rect::new(
            self.closed_rect.x,
            self.closed_rect.y + if self.closed_rect.bottom() + extra_height <= screen_height() 
                { variant_extension } else { -extra_height },
            self.closed_rect.w,
            extra_height + self.border_size
        );
    }
}

trait CarouselType {
    fn get_string(&self) -> String;
}

#[derive(Clone)]
struct Carousel {
    variant_font: Font,
    variant_text_params: TextParams,
    /// rect to contain the currently selected variant
    rect: Rect,
    back: Texture2D,
    border_size: f32,
    left_arrow_rect: Rect,
    right_arrow_rect: Rect,
    right_arrow_image: Texture2D,
    hovering: bool
}
impl Carousel {
    async fn new(
        visualiser: &Visualiser,
        font_size: u16,
        topleft: (f32, f32),
        size: (f32, f32),
        border_size: f32
    ) -> Carousel {
        let outer_rect = Rect::new(topleft.0, topleft.1, size.0, size.1);
        let inner_rect = inflate_rect(&outer_rect, -border_size);

        let variant_font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        let variant_text_params = TextParams { font: variant_font, 
            font_size: font_size, color: WHITE, ..Default::default()};

        Carousel {
            variant_font, variant_text_params, 
            rect: outer_rect,
            back: get_back_gradient(visualiser, outer_rect.x as u16, outer_rect.w as u16, outer_rect.h as u16),
            border_size, 
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

        self.draw(&variants[index], allowed_left, allowed_right);

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

    fn draw<T: CarouselType>(&self, current_variant: &T, allowed_left: bool, allowed_right: bool) {
        draw_texture(self.back, self.rect.x, self.rect.y, WHITE);
        draw_rectangle(
            self.rect.x + self.border_size,
            self.rect.y + self.border_size,
            self.rect.w - 2.*self.border_size,
            self.rect.h - 2.*self.border_size,
            BLACK
        );

        let current_string = current_variant.get_string();
        let measure = measure_text(
            &current_string, 
            Some(self.variant_font), 
            self.variant_text_params.font_size, self.variant_text_params.font_scale
        );
        draw_text_ex(
            &current_string, 
            self.rect.center().x - measure.width/2., 
            self.rect.center().y + measure.height/2.,
            self.variant_text_params
        );

        if self.hovering {
            draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, HOVER_WHITE_OVERLAY);
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
}

trait MenuType {
    fn update(&mut self, visualiser: &mut Visualiser) -> MenuSignal;
    fn get_editing(&mut self) -> bool;
    fn open_layer_to_edit(&mut self, _index: usize, _visualiser: &Visualiser) {}
}

/// generates the text boxes for the general menu
struct GeneralMenuTextBoxGenerator {
    labels: [&'static str;5],
    label_dims: [TextDimensions; 5],
    label_params: TextParams,
    default_data: [f64; 5],
    width: f32,
    height: f32,
    start_x: u16,
    start_y: f32,
    y_change: f32,
    gradient: Texture2D,
    content_params: TextParams
}
impl GeneralMenuTextBoxGenerator {
    fn get_text_box(&self, i: usize) -> TextBox {
        TextBox::new(
            Some(InputLabel { 
                text: self.labels[i].to_owned(), 
                label_dims: self.label_dims[i], 
                label_params: self.label_params
            }),
            self.default_data[i].to_string(),
            self.width,
            self.height,
            self.start_x,
            self.start_y + i as f32*self.y_change,
            self.gradient,
            self.content_params
        )
    }
}

struct GeneralMenu {
    center_re: TextBox,
    center_im: TextBox,
    magnification: TextBox,
    max_iterations: TextBox,
    bailout: TextBox
}
impl GeneralMenu {
    fn new(visualiser: &Visualiser, text_font: Font) -> GeneralMenu {
        let labels = [
            "center (re)", "center (im)", "magnification", 
            "max iterations", "bailout"
        ];
        let mut label_dims: [TextDimensions; 5] = [TextDimensions{width: 0., height: 0., offset_y: 0.}; 5];
        let font_size = (screen_width() * TEXTBOX_LABEL_FONT_PROPORTION) as u16;

        let mut max_label_width = 0.0;
        for (i, label) in labels.iter().enumerate() {
            let dims = measure_text(label, Some(text_font), font_size, 1.0);
            label_dims[i] = dims;
            max_label_width = f32::max(max_label_width, dims.width);
        }

        let start_x = (screen_width() * TEXTBOX_LABEL_WIDTH_PADDING_PROPORTION * 2. + max_label_width) as u16;
        let start_y = screen_height() * TEXTBOX_START_Y_PROPORTION;
        let y_change = screen_height() * (TEXTBOX_HEIGHT_PROPORTION + TEXTBOX_HEIGHT_PADDING_PROPORTION);
        let width = screen_width()*MENU_SCREEN_PROPORTION 
                         - start_x as f32
                         - screen_width() * TEXTBOX_RIGHT_PADDING;
        let height = screen_height() * TEXTBOX_HEIGHT_PROPORTION;
        
        let gradient = get_back_gradient(visualiser, start_x, width as u16, height as u16);
        let label_params = TextParams {font: text_font, font_size, color: get_brightest_colour(gradient), ..Default::default()};
        let content_params = TextParams {font: text_font, font_size, color: WHITE, ..Default::default()};

        let generator = GeneralMenuTextBoxGenerator {
            labels, label_dims, label_params, default_data: [-0.5, 0., 1.0, 500., 200.], 
            start_x, width, height, start_y, y_change, gradient, content_params
        };


        GeneralMenu {
            center_re: generator.get_text_box(0),
            center_im: generator.get_text_box(1),
            magnification: generator.get_text_box(2),
            max_iterations: generator.get_text_box(3),
            bailout: generator.get_text_box(4)
        }
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
            (0.005/visualiser.pixel_step).to_string()
        } else if i == 3 {
            visualiser.max_iterations.to_string()
        } else {
            20.0.to_string() // placeholder until bailout becomes dynamic
        }
    }

    fn update_data(visualiser: &mut Visualiser, i: usize, new: String) {
        if i == 0 {
            visualiser.center.update_real_from_string(new);
        } else if i == 1 {
            visualiser.center.update_im_from_string(new);
        } else if i == 2 {
            if let Ok(new) = new.parse::<f64>() {
                visualiser.set_pixel_step(0.005/new);
            }
        } else if i == 3 {
            if let Ok(new) = new.parse::<f32>() {
                visualiser.max_iterations = new;
            }
        } else {
            // placeholder until bailout becomes dynamic
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

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for text_box in self.all_text_boxes().iter() {
            if text_box.selected { return true }
        }
        false
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
            label_params: strength_label_params 
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
        let edit_button_x = inner_rect.w*LAYERMANAGER_HALF_END_PROPORION + 
            screen_width()*LAYERMANAGER_INNER_LEFT_PADDING;
        let edit_button_border = screen_height()*LAYERMANAGER_EDIT_BUTTON_BORDER_HEIGHT;

        let layer_range_dropdown_y = palette_size + 2.*screen_height()*LAYERMANAGER_INNER_TOP_PADDING;
        // let layer_range_dropdown_y = inner_rect.y + screen_height()*LAYERMANAGER_INNER_TOP_PADDING;

        let drag_x = edit_button_x + palette_size + screen_width()*LAYERMANAGER_LEFT_PADDING;

        let delete_button_size = screen_height()*LAYERMANAGER_DELETE_BUTTON_SIZE;
        let delete_button_x_offset = inner_rect.w-delete_button_size-screen_height()*LAYERMANAGER_INNER_LEFT_PADDING;

        LayerManager { 
            border_back: get_back_gradient(
                visualiser, 
                outer_rect.x as u16, 
                outer_rect.w as u16, 
                outer_rect.h as u16
            ), 
            outer_rect, inner_rect,
            palette_button: Button::new(
                (palette_size, palette_size),
                (screen_width()*LAYERMANAGER_INNER_LEFT_PADDING, screen_height()*LAYERMANAGER_INNER_TOP_PADDING),
                vec![Box::new(ButtonImageElement::from_texture(
                    visualiser.layers.layers[layer_num].palette.get_full_gradient(palette_size, palette_size),
                    1., (0., 0.), 0
                ))],
                vec![
                    Box::new(ButtonColourElement::new(Color::new(0., 0., 0., 0.5), (palette_size, palette_size), (0., 0.), 1)),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/wrench.png").await.unwrap(), 0.7, 
                        DrawTextureParams { dest_size: Some(Vec2::new(palette_size, palette_size)), ..Default::default() },
                        (0., 0.), 2
                    ))
                ], 
                vec![]
            ),
            name: TextBox::new(
                None, 
                layer.name.clone(),
                name_textbox_width,
                name_textbox_height,
                name_textbox_start_x as u16,
                screen_height()*LAYERMANAGER_INNER_TOP_PADDING,
                get_back_gradient(
                    visualiser,
                    name_textbox_start_x as u16,
                    name_textbox_width as u16,
                    name_textbox_height as u16
                ),
                name_text_params
            ), 
            layer_type_text_params,
            edit_button: Button::new(
                (palette_size, palette_size),
                (edit_button_x, screen_height()*LAYERMANAGER_INNER_TOP_PADDING),
                vec![
                    Box::new(ButtonImageElement::from_texture(
                        get_back_gradient(visualiser, edit_button_x as u16, palette_size as u16, palette_size as u16),
                        1., (0., 0.), 0
                    )),
                    Box::new(ButtonColourElement::new(BLACK, 
                        (palette_size-2.*edit_button_border, palette_size-2.*edit_button_border), (edit_button_border, edit_button_border),
                        1
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/wrench.png").await.unwrap(), 0.7, 
                        DrawTextureParams { dest_size: Some(Vec2::new(palette_size, palette_size)), ..Default::default() },
                        (0., 0.), 2
                    ))
                ],
                vec![Box::new(ButtonColourElement::new( HOVER_WHITE_OVERLAY, (palette_size, palette_size), (0., 0.), 3 ))], 
                vec![]
            ),
            strength_slider: generate_strength_slider(strength_slider_text_params, inner_rect, layer.strength),
            layer_range_dropdown: DropDown::new(
                visualiser,
                (screen_width()*LAYERMANAGER_LAYER_RANGE_FONT_PROPORTION) as u16,
                (edit_button_x, layer_range_dropdown_y),
                // (edit_button_x+palette_size+screen_width()*LAYERMANAGER_INNER_LEFT_PADDING, layer_range_dropdown_y),
                (palette_size, inner_rect.bottom() - layer_range_dropdown_y - screen_height()*LAYERMANAGER_INNER_TOP_PADDING),
                // (palette_size, palette_size*0.4),
                edit_button_border,
                None
            ).await,
            delete_button: Button::new(
                (delete_button_size, delete_button_size),
                (delete_button_x_offset, screen_height()*LAYERMANAGER_INNER_TOP_PADDING),
                vec![
                    Box::new(ButtonImageElement::from_texture(
                        get_back_gradient(visualiser, (inner_rect.x+delete_button_x_offset) as u16, delete_button_size as u16, delete_button_size as u16),
                        1., (0., 0.), 0
                    )),
                    Box::new(ButtonColourElement::new(
                        BLACK, (delete_button_size-2.*edit_button_border, delete_button_size-2.*edit_button_border),
                        (edit_button_border, edit_button_border), 1
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/cross.png").await.unwrap(), 1.,
                        DrawTextureParams { dest_size: Some((delete_button_size-2.*edit_button_border, delete_button_size-2.*edit_button_border).into()),
                            ..Default::default() },
                        (edit_button_border, edit_button_border), 2
                    ))
                ],
                vec![Box::new(ButtonColourElement::new(
                        HOVER_WHITE_OVERLAY, (delete_button_size, delete_button_size), (0., 0.), 3
                ))],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_RED_OVERLAY, (delete_button_size, delete_button_size), (0., 0.), 4
                ))]
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
        let mut copy = copy.clone();
        copy.undo_translation();

        let mut palette_button = copy.palette_button.clone();
        let palette_size = screen_height()*LAYERMANAGER_PALETTE_HEIGHT_PROPORTION;
        palette_button.back_elements[0] = Box::new(ButtonImageElement::from_texture(
            visualiser.layers.layers[visualiser.layers.layers.len()-1].palette.get_full_gradient(palette_size, palette_size),
            1., (0., 0.), 0
        ));

        let mut strength_slider = copy.strength_slider.clone();
        strength_slider.percentage = 0.;

        let outer_rect = Rect::new(
            screen_width()*LAYERMANAGER_LEFT_PADDING,
            screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - (visualiser.layers.layers.len()-1) as f32 * 
                (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING)) + scroll,
            screen_width()*(MENU_SCREEN_PROPORTION-LAYERMANAGER_LEFT_PADDING-LAYERMANAGER_RIGHT_PADDING),
            screen_height()*LAYERMANAGER_HEIGHT
        );

        LayerManager { 
            border_back: copy.border_back, 
            outer_rect,
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
        }
    }

    fn translate(&mut self, new_outer_rect_pos: (f32, f32)) {
        if self.translated {
            if self.outer_rect.x == new_outer_rect_pos.0 && self.outer_rect.y == new_outer_rect_pos.1 {
                return;
            }
            self.undo_translation();
        }

        self.outer_rect.x = new_outer_rect_pos.0;
        self.outer_rect.y = new_outer_rect_pos.1;
        self.perform_translation();

        self.translated = true;
    }

    fn translate_items(&mut self, translate: (f32, f32)) {
        self.palette_button.translate(translate);
        self.name.translate(translate);
        self.edit_button.translate(translate);
        self.strength_slider.translate(translate);
        self.layer_range_dropdown.translate(translate);
        self.delete_button.translate(translate);
        translate_rect(&mut self.drag_rect, translate);
    }

    fn perform_translation(&mut self) {
        let translate = (self.outer_rect.x, self.outer_rect.y);
        translate_rect(&mut self.inner_rect, translate);

        let translate = (self.inner_rect.x, self.inner_rect.y);
        self.translate_items(translate);
    }

    fn undo_translation(&mut self) {
        let translate = (-self.outer_rect.x, -self.outer_rect.y);
        let old_inner = self.inner_rect.clone();
        translate_rect(&mut self.inner_rect, translate);

        let translate = (-old_inner.x, -old_inner.y);
        self.translate_items(translate);
    }

    fn update(&mut self, layer: &mut Layer, update_edit_button: bool) -> bool {
        if !self.translated {
            self.translate((self.outer_rect.x, self.outer_rect.y));
        }

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
    async fn new(visualiser: &Visualiser, font: Font) -> LayersMenu {
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
                Box::new(ButtonImageElement::from_texture(
                    get_back_gradient(
                        visualiser, 
                        add_rect.x as u16, 
                        add_rect.w as u16, 
                        add_rect.h as u16
                    ),
                    1., 
                    (0., 0.),
                    0
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
                Box::new(ButtonImageElement::from_image(
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
    
    fn add_layer(&mut self, visualiser: &mut Visualiser) {
        let mut new_layer =  Layer::new(LayerType::Colour, LayerRange::Both, 0., Palette::default());
        new_layer.palette.generate_palette(visualiser.max_iterations);
        visualiser.layers.add_layer(&new_layer);

        self.layer_managers.push(LayerManager::new_copy(&visualiser, &self.layer_managers[0], self.scroll));

        self.update_add_button_pos(visualiser.layers.layers.len(), -1);
    }

    fn delete_layer(&mut self, visualiser: &mut Visualiser, i: usize) {
        visualiser.layers.delete_layer(i);
        self.layer_managers.remove(i);

        for (i, layer_manager) in self.layer_managers.iter_mut().enumerate() {
            layer_manager.translate((
                screen_width()*LAYERMANAGER_LEFT_PADDING,
                screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - i as f32 * 
                    (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING)) + self.scroll
            ));
        }

        self.update_add_button_pos(visualiser.layers.layers.len(), 1);
        self.update_scroll(true);
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
        for (i, layer_manager) in self.layer_managers.iter_mut().enumerate() {
            layer_manager.translate((
                screen_width()*LAYERMANAGER_LEFT_PADDING,
                screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - i as f32 * 
                    (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING)) + self.scroll
            ));
        }
    }

    fn get_add_topleft(layers_num: usize) -> (f32, f32) {
        (
            screen_width()*LAYERMANAGER_LEFT_PADDING,
            screen_height()*(1.0-LAYERMANAGER_BOTTOM_PADDING-LAYERMANAGER_HEIGHT) - layers_num as f32 * 
                    (screen_height()*(LAYERMANAGER_HEIGHT+LAYERMANAGER_TOP_PADDING))
        )
    }

    /// draw + update scroll
    fn update_scroll(&mut self, just_deleted: bool) {
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
            manager.translate((manager.outer_rect.x, manager.outer_rect.y - self.scroll));
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
            manager.translate((manager.outer_rect.x, manager.outer_rect.y + self.scroll));
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
            if manager.edit_button.rect.overlaps(&manager.layer_range_dropdown.open_rect) {
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
                &mut visualiser.layers.layers[i], 
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
            self.update_scroll(false);
        }

        if changed {
            Layers::place_constraints(&mut visualiser.layers.layers);
            visualiser.generate_image();
        }

        MenuSignal::None
    }

    fn get_editing(&mut self) -> bool {
        for manager in self.layer_managers.iter() {
            if manager.name.selected { return true }
        }
        false
    }
}

struct SpecificMenuEditorInputGenerator {
    // y position of the bottom of the title
    top_y: f32,
    font: Font,
    font_size: u16,
    inputbox_start_x: f32,
    inputbox_width: f32,
    inputbox_height: f32,
    border_size: f32,
    inputbox_vert_padding: f32,
    inputbox_label_colour: Color
}
impl SpecificMenuEditorInputGenerator {
    async fn new(visualiser: &Visualiser) -> SpecificMenuEditorInputGenerator {
        let font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();

        let title_height = measure_text(
            "Orbit Trap", 
            Some(font), 
            (screen_width()*LAYEREDITOR_SPECIFIC_MENU_TITLE_FONT_PROPORTION) as u16, 
            1.0
        ).height;

        let inputbox_start_x = screen_width()*LAYEREDITOR_TEXTBOX_START_X;
        let inputbox_width = screen_width()*(MENU_SCREEN_PROPORTION-TEXTBOX_RIGHT_PADDING) - inputbox_start_x;

        SpecificMenuEditorInputGenerator { 
            top_y: screen_height()*(NAVBAR_HEIGHT_PROPORTION+LAYEREDITOR_CAROUSEL_HEIGHT+2.*STATE_TEXT_PADDING_PROPORTION+
                TEXTBOX_HEIGHT_PROPORTION+2.*LAYEREDITOR_INPUT_BOX_VERT_PADDING+LAYEREDTIOR_SPECIFIC_MENU_BAR_HEIGHT) + screen_width()*STATE_TEXT_FONT_PROPORTION+title_height, 
            font, 
            font_size: (screen_width()*LAYEREDITOR_INPUT_FONT_PROPORTION) as u16, 
            inputbox_start_x, 
            inputbox_width, 
            inputbox_height: screen_height()*TEXTBOX_HEIGHT_PROPORTION,
            border_size: screen_height()*TEXTBOX_BORDER_PROPORTION,
            inputbox_vert_padding: screen_height()*LAYEREDITOR_INPUT_BOX_VERT_PADDING,
            inputbox_label_colour: get_brightest_colour(get_back_gradient(
                visualiser, 
                inputbox_start_x as u16, 
                inputbox_width as u16, 
                1
            ))
        }
    }

    fn get_inputbox_dims(&self, num: usize) -> Rect {
        Rect::new(
            self.inputbox_start_x,
            self.top_y + self.inputbox_vert_padding + num as f32 * (self.inputbox_height+self.inputbox_vert_padding),
            self.inputbox_width,
            self.inputbox_height
        )
    }

    fn get_text_params(&self) -> TextParams {
        TextParams { 
            font: self.font, 
            font_size: self.font_size, 
            color: self.inputbox_label_colour,
            ..Default::default()
        }
    }

    fn measure_text(&self, text: &str) -> TextDimensions {
        measure_text(
            text,
            Some(self.font), 
            self.font_size, 
            1.0
        )
    }

    async fn make_dropdown<T: DropDownType<T> + PartialEq + Clone>(&self, visualiser: &Visualiser, num: usize, label: &str)-> DropDown<T> {
        let rect = self.get_inputbox_dims(num);

        DropDown::new(
            visualiser,
            self.font_size,
            (rect.x, rect.y),
            (rect.w, rect.h),
            self.border_size,
            Some(InputLabel { 
                text: String::from(label), 
                label_dims: self.measure_text(label), 
                label_params: self.get_text_params()
            })
        ).await
    }

    fn make_textbox(&self, visualiser: &Visualiser, num: usize, label: &str) -> TextBox {
        let rect = self.get_inputbox_dims(num);

        TextBox::new(
            Some(InputLabel { 
                text: String::from(label),
                label_dims: self.measure_text(label), 
                label_params: self.get_text_params()
            }),
            String::from("0"),
            rect.w, rect.h, rect.x as u16, rect.y,
            get_back_gradient(visualiser, rect.x as u16, rect.w as u16, rect.h as u16),
            TextParams { font: self.font, font_size: self.font_size, color: WHITE, ..Default::default() }
        )
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
        let generator = SpecificMenuEditorInputGenerator::new(visualiser).await;

        let top_bar = get_back_gradient(visualiser, 0, 
            (screen_width()*MENU_SCREEN_PROPORTION) as u16, 
            (screen_height()*LAYEREDTIOR_SPECIFIC_MENU_BAR_HEIGHT) as u16
        );

        OrbitTrapEditor {
            top_bar,
            top_bar_y: screen_height()*(NAVBAR_HEIGHT_PROPORTION+LAYEREDITOR_CAROUSEL_HEIGHT+2.*STATE_TEXT_PADDING_PROPORTION+
                TEXTBOX_HEIGHT_PROPORTION+2.*LAYEREDITOR_INPUT_BOX_VERT_PADDING) + screen_width()*STATE_TEXT_FONT_PROPORTION,
            title_params: TextParams { 
                font: generator.font, 
                font_size: (screen_width()*LAYEREDITOR_SPECIFIC_MENU_TITLE_FONT_PROPORTION) as u16, 
                color: get_brightest_colour(top_bar),
                ..Default::default()
            },
            trap_type: generator.make_dropdown(visualiser, 0, "type").await,
            analysis: generator.make_dropdown(visualiser, 1, "analysis").await,
            center_re: generator.make_textbox(visualiser, 2, "center (re)"),
            center_im: generator.make_textbox(visualiser, 3, "center (im)"),
            radius: generator.make_textbox(visualiser, 4, "radius"),
            arm_length: generator.make_textbox(visualiser, 4, "arm length"),
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
        if let Some(new_im) = self.center_im.update(orbit_trap.get_center_im().to_string()) {
            if let Ok(new) = new_im.parse::<f64>() {
                orbit_trap.set_center_im(new);
                changed = true;
            }
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
        let carousel_font_size = (screen_width()*LAYEREDITOR_CAROUSEL_FONT_PROPORTION) as u16;
        let font_size = (screen_width()*LAYEREDITOR_INPUT_FONT_PROPORTION) as u16;
        
        let carousel_start_y = screen_height()*(NAVBAR_HEIGHT_PROPORTION+2.*STATE_TEXT_PADDING_PROPORTION) + screen_width()*STATE_TEXT_FONT_PROPORTION;
        let carousel_height = screen_height()*LAYEREDITOR_CAROUSEL_HEIGHT;

        let textbox_start_x = screen_width()*LAYEREDITOR_TEXTBOX_START_X;
        let type_textbox_start_y = carousel_start_y + carousel_height + screen_height()*LAYEREDITOR_INPUT_BOX_VERT_PADDING;

        let layer_type_label_font = load_ttf_font("assets/Montserrat-SemiBold.ttf").await.unwrap();
        let layer_type_label_measure = measure_text(
            "type", 
            Some(layer_type_label_font), 
            font_size, 
            1.0
        );

        LayerEditorMenu { 
            layer_carousel: Carousel::new(
                visualiser,
                carousel_font_size,
                (0., carousel_start_y),
                (screen_width()*MENU_SCREEN_PROPORTION, carousel_height),
                screen_width()*NAVBAR_BORDER_WIDTH_PROPORTION
            ).await,
            layer_type: DropDown::new(
                visualiser, 
                font_size as u16,
                (textbox_start_x, type_textbox_start_y),
                (screen_width()*(MENU_SCREEN_PROPORTION-TEXTBOX_RIGHT_PADDING) - textbox_start_x, screen_height()*TEXTBOX_HEIGHT_PROPORTION),
                screen_height()*TEXTBOX_BORDER_PROPORTION,
                Some(InputLabel { 
                    text: String::from("type"),
                    label_dims: layer_type_label_measure,
                    label_params: TextParams { font: layer_type_label_font, font_size, 
                        color: get_brightest_colour(get_back_gradient(
                            visualiser, 
                            textbox_start_x as u16,  
                            (screen_width()*(MENU_SCREEN_PROPORTION-TEXTBOX_RIGHT_PADDING) - textbox_start_x) as u16, 
                            layer_type_label_measure.height as u16
                        )), 
                        ..Default::default() 
                    }
                })
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
}

fn color_with_params(colour: &Color, r: Option<f32>, g: Option<f32>, b: Option<f32>, a: Option<f32>) -> Color {
    Color {
        r: r.unwrap_or(colour.r),
        g: g.unwrap_or(colour.g),
        b: b.unwrap_or(colour.b),
        a: a.unwrap_or(colour.a)
    }
}

struct ColourPointEditor {
    map_rect: Rect,
    rect: Rect,
    colour: Color,
    selected: bool,
    deselect_y: f32,
    selected_x_offset: f32,
    outer_select_box: Rect,
    inner_select_box: Rect
}
impl ColourPointEditor {
    fn new(colour_point: &ColourPoint, map_rect: Rect) -> ColourPointEditor {
        let width = screen_width()*PALETTEEDITOR_COLOUR_POINT_WIDTH;
        let select_width = screen_width()*PALETTEEDITOR_COLOUR_POINT_SELECT_WIDTH;
        let x = map_rect.x - width/2. + (colour_point.percent/100.)*map_rect.w;

        let outer_select_box = Rect::new(
            x + (width-select_width)/2.,
            map_rect.top(),
            select_width,
            map_rect.h
        );

        ColourPointEditor { 
            map_rect,
            rect: Rect::new(
                x,
                map_rect.bottom(),
                width,
                2. * width
            ),
            colour: colour_point.colour,
            selected: false,
            deselect_y: map_rect.bottom() + 2.* width + screen_height()*PALETTEEDITOR_VERT_PADDING,
            selected_x_offset: 0.0,
            outer_select_box,
            inner_select_box: inflate_rect(&outer_select_box, -screen_width()*PALETTEEDITOR_COLOUR_POINT_SELECT_BORDER_WIDTH)
        }
    }

    fn translate_to(&mut self, new_x: f32) -> Option<f32> {
        let old_x = self.rect.x;

        self.rect.x = new_x;
        self.rect.x = self.rect.x.clamp(self.map_rect.x - self.rect.w/2., self.map_rect.right() - self.rect.w/2.);

        let delta = self.rect.x - old_x;
        self.outer_select_box.x += delta;
        self.inner_select_box.x += delta;

        match delta == 0.0 {
            true => None,
            false => Some( ((self.rect.center().x - self.map_rect.x) / self.map_rect.w)*100. )
        }
    }

    /// draw and update the `ColourPointEditor`
    /// 
    /// # Returns
    /// None if the point was unchanged
    /// Some(new percentage) if the point was changed 
    fn update(&mut self, other_selected: bool) -> Option<f32> {
        self.draw();
        self.mouse_interact(other_selected)
    }
    
    fn draw(&self) {
        let color = match self.selected {
            true => WHITE,
            false => LAYERMANAGER_LAYER_TYPE_COLOUR
        };

        draw_triangle(
            Vec2::new(self.rect.x + self.rect.w/2., self.rect.y),
            Vec2::new(self.rect.x, self.rect.y + self.rect.h/2.),
            Vec2::new(self.rect.x + self.rect.w, self.rect.y + self.rect.h/2.),
            color
        );
        draw_rectangle(
            self.rect.x,
            self.rect.y + self.rect.h/2.,
            self.rect.w,
            self.rect.h/2.,
            color
        );
        draw_circle(
            self.rect.x + self.rect.w/2.,
            self.rect.y + self.rect.h * 0.75,
            self.rect.w * 0.4,
            self.colour
        );

        if !self.selected { return }

        draw_rectangle(
            self.outer_select_box.x, 
            self.outer_select_box.y, 
            self.outer_select_box.w, 
            self.outer_select_box.h, 
            WHITE
        );
        draw_rectangle(
            self.inner_select_box.x,
            self.inner_select_box.y,
            self.inner_select_box.w,
            self.inner_select_box.h,
            self.colour
        );
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
        let inner_button_size = button_size - 2.*button_border;
        let delete_button_start_x = screen_width()*(MENU_SCREEN_PROPORTION-PALETTEEDITOR_HOR_PADDING)-button_size;

        let font_size = screen_width()*PALETTEEDTIOR_FONT_PROPORTION;

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
            colour_point_editors: Vec::new(),
            add_button: Button::new(
                (button_size, button_size),
                (start_x, title_rect.h + 2.*vert_padding + screen_height()*PALETTEEDITOR_PALETTE_HEIGHT
                        +screen_width()*2.*PALETTEEDITOR_COLOUR_POINT_WIDTH),
                vec![
                    Box::new(ButtonImageElement::from_texture(
                        get_back_gradient(visualiser, start_x as u16, button_size as u16, button_size as u16),
                        1.0, (0., 0.), 0
                    )),
                    Box::new(ButtonColourElement::new(
                        BLACK, (inner_button_size, inner_button_size), (button_border, button_border), 1
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/plus.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        2
                    ))
                ],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_WHITE_OVERLAY, (button_size, button_size), (0., 0.), 3
                ))],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_BLACK_OVERLAY, (button_size, button_size), (0., 0.), 4
                ))]
            ),
            delete_button: Button::new(
                (button_size, button_size),
                (delete_button_start_x, 
                        title_rect.h + 2.*vert_padding + screen_height()*PALETTEEDITOR_PALETTE_HEIGHT
                            +screen_width()*2.*PALETTEEDITOR_COLOUR_POINT_WIDTH),
                vec![
                    Box::new(ButtonImageElement::from_texture(
                        get_back_gradient(visualiser, delete_button_start_x as u16, button_size as u16, button_size as u16),
                        1.0, (0., 0.), 0
                    )),
                    Box::new(ButtonColourElement::new(
                        BLACK, (inner_button_size, inner_button_size), (button_border, button_border), 1
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/bin.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        2
                    ))
                ],
                vec![
                    Box::new(ButtonColourElement::new(
                        BLACK, (inner_button_size, inner_button_size), (button_border, button_border), 3
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/binOpen.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        4
                    )),
                    Box::new(ButtonColourElement::new(
                        HOVER_WHITE_OVERLAY, (button_size, button_size), (0., 0.), 5
                ))
                ],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_BLACK_OVERLAY, (button_size, button_size), (0., 0.), 6
                ))]
            ),
            red_slider: PaletteEditor::get_slider(0, visualiser, font, title_rect.h, vert_padding),
            green_slider: PaletteEditor::get_slider(1, visualiser, font, title_rect.h, vert_padding),
            blue_slider: PaletteEditor::get_slider(2, visualiser, font, title_rect.h, vert_padding),
            alpha_slider: PaletteEditor::get_slider(3, visualiser, font, title_rect.h, vert_padding),
            bar_rect, 
            bar_grad: get_back_gradient(visualiser, bar_rect.x as u16, bar_rect.w as u16, bar_rect.h as u16),
            palette_rect,
            mapping_type: DropDown::new(
                visualiser,
                font_size as u16,
                (textbox_dims.right()-screen_width()*PALETTEEDITOR_MAPPING_DROPDOWN_WIDTH, palette_rect.bottom() + vert_padding),
                (screen_width()*PALETTEEDITOR_MAPPING_DROPDOWN_WIDTH, textbox_dims.h),
                screen_height() * TEXTBOX_BORDER_PROPORTION,
                Some(InputLabel::new("mapping type", font, font_size, WHITE))
            ).await,
            length_slider: PaletteEditor::get_slider(4, visualiser, font, title_rect.h, vert_padding),
            offset_slider: PaletteEditor::get_slider(5, visualiser, font, title_rect.h, vert_padding),
            sumbit_button: Button::new(
                (button_size, button_size),
                (screen_width()*(MENU_SCREEN_PROPORTION/2. - PALETTEEDITOR_HOR_PADDING/2.) - button_size, 
                         screen_height() - button_size - vert_padding),
                vec![
                    Box::new(ButtonImageElement::from_texture(
                        get_back_gradient(visualiser, 
                            (screen_width()*(MENU_SCREEN_PROPORTION/2. - PALETTEEDITOR_HOR_PADDING/2.) - button_size) as u16, 
                            button_size as u16, button_size as u16),
                        1.0, (0., 0.), 0
                    )),
                    Box::new(ButtonColourElement::new(
                        BLACK, (inner_button_size, inner_button_size), (button_border, button_border), 1
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/tick.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        2
                    ))
                ],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_WHITE_OVERLAY, (button_size, button_size), (0., 0.), 3
                ))],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_BLACK_OVERLAY, (button_size, button_size), (0., 0.), 4
                ))]
            ),
            cancel_button: Button::new(
                (button_size, button_size),
                (screen_width()*(MENU_SCREEN_PROPORTION/2. + PALETTEEDITOR_HOR_PADDING/2.), 
                         screen_height() - button_size - vert_padding),
                vec![
                    Box::new(ButtonImageElement::from_texture(
                        get_back_gradient(visualiser, 
                            (screen_width()*(MENU_SCREEN_PROPORTION/2. + PALETTEEDITOR_HOR_PADDING/2.)) as u16, 
                            button_size as u16, button_size as u16),
                        1.0, (0., 0.), 0
                    )),
                    Box::new(ButtonColourElement::new(
                        BLACK, (inner_button_size, inner_button_size), (button_border, button_border), 1
                    )),
                    Box::new(ButtonImageElement::from_image(
                        load_image("assets/cross.png").await.unwrap(),
                        1.0,
                        DrawTextureParams {dest_size: Some((inner_button_size, inner_button_size).into()), ..Default::default()},
                        (button_border, button_border),
                        2
                    ))
                ],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_WHITE_OVERLAY, (button_size, button_size), (0., 0.), 3
                ))],
                vec![Box::new(ButtonColourElement::new(
                    HOVER_BLACK_OVERLAY, (button_size, button_size), (0., 0.), 4
                ))]
            ),
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
                    WHITE
                )),
            0., 
            match slider_i <= 3 {
                true => 255.,
                false => 100.
            },
            None, 
            Some(
                TextBox::new(
                    None, 
                    String::from(""),
                    textbox_width,
                    textbox_height,
                    textbox_start_x as u16,
                    rect.y - screen_height()*(PALETTEEDITOR_TEXTBOX_HEIGHT-PALETTEEDITOR_COLOUR_SLIDER_HEIGHT)/2.,
                    get_back_gradient(visualiser, textbox_start_x as u16, textbox_width as u16, textbox_height as u16),
                    TextParams { 
                        font, 
                        font_size: font_size as u16,
                        color: WHITE ,
                        ..Default::default()
                    }
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

        self.delete_button.hovering = false;
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

        draw_texture(
            palette.get_full_gradient(self.colour_map_rect.w, self.colour_map_rect.h), 
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
            if point_editor.selected { selected_point = Some(i) }
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
        draw_texture(
            palette.get_full_palette(self.palette_rect.w, self.palette_rect.h, visualiser.max_iterations), 
            self.palette_rect.x, self.palette_rect.y, WHITE
        );

        if !self.mapping_type.open {
            self.length_slider.percentage = palette.get_palette_length()/100.;
            self.length_slider.update();
            if palette.set_palette_length(self.length_slider.percentage * self.length_slider.conversion) {
                changed_this_frame = true;
            }

            self.offset_slider.percentage = palette.get_offset()/100.;
            self.offset_slider.update();
            if palette.set_offset(self.offset_slider.percentage * self.length_slider.conversion) {
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
}