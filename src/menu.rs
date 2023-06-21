// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use super::{ScreenDimensions, Visualiser, escape_time, interpolate_colour};

/// the proportion of the screen width taken over by the menu
const MENU_SCREEN_PROPORTION: f32 = 0.25;
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
/// proportion of the screen height for the border of tex boxes
const TEXTBOX_BORDER_PROPORTION: f32 = TEXTBOX_HEIGHT_PROPORTION/20.;
/// proportion of the screen width for the padding on the right of text boxes
const TEXTBOX_RIGHT_PADDING: f32 = MENU_SCREEN_PROPORTION/50.;
/// proportion of the screen width for the padding between the text box and content inside
const TEXTBOX_CONTENT_PADDING: f32 = MENU_SCREEN_PROPORTION/100.;

/// gives a texture which is a snippet of the gradient for the menu at the given placef
fn get_back_gradient(visualiser: &Visualiser, start_x: u16, width: u16, height: u16) -> Texture2D {
    let mut image = Image::gen_image_color(width, height, BLACK);
    
    let pallete_width = visualiser.layers.layers[0].pallete.len();

    for i in 0..width {
        let screen_x = start_x + i;
        let menu_fraction = screen_x as f32 / (MENU_SCREEN_PROPORTION*screen_width());
        let mut colour = Color::new(0., 0., 0., 1.);
        for layer in visualiser.layers.layers.iter() {
            if layer.layer_type.shading_layer() { continue }
            if !layer.layer_range.layer_applies(false) { continue } // has to be out the set
            let pallete_fraction = ((menu_fraction * layer.get_pallete_length()) / 1000.)*pallete_width as f32;
            let layer_colour = escape_time(pallete_fraction as f64, &layer.pallete);
            colour = interpolate_colour(colour, layer_colour, 0.9 * layer.strength);
        }
        for j in 0..height {
            image.set_pixel(i as u32, j as u32, colour);
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
    let mut colours = Vec::with_capacity(gradient.width() as usize);
    for i in 0..gradient.width() as u32 {
        colours.push(gradient.get_texture_data().get_pixel(i, 0));
    }

    let mut brightest_colour: Color = BLACK;
    let mut largest_luminance = 0.0;
    for colour in colours {
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

#[derive(Clone, Copy)]
enum MenuState {
    Closed,
    General,
    Layers,
    LayerEditor,
    Screenshot,
    Video
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

    fn get_string(&self) -> String {
        String::from(match self {
            MenuState::General => "GENERAL",
            MenuState::Layers => "LAYERS",
            MenuState::LayerEditor => "LAYER EDITOR",
            MenuState::Screenshot => "SCREENSHOT",
            MenuState::Video => "VIDEO",
            _ => ""
        })
    }

    /// draw the state to the screen 
    fn draw_state(&self, font: Font, colour: Color) {
        let text = &self.get_string();
        let font_size = (screen_width() * STATE_TEXT_FONT_PROPORTION) as u16;
        let dims = measure_text(text, Some(font), font_size, 1.0);
        draw_text_ex(
            text,
            screen_width()*MENU_SCREEN_PROPORTION*0.5 - dims.width*0.5,
            screen_height()*NAVBAR_HEIGHT_PROPORTION + dims.height + screen_height()*STATE_TEXT_PADDING_PROPORTION,
            TextParams {font, font_size, color: colour, ..Default::default()}
        )
    }

    fn create_menu(&self, visualiser: &mut Visualiser, index: usize, font: Font) -> Option<Box<dyn MenuType>> {
        match index {
            0 => Some(Box::new(GeneralMenu::new(visualiser, font))),
            // placeholder
            _ => Some(Box::new(GeneralMenu::new(visualiser, font)))
        }
    }

    fn update_state_menu(&self, menus: &mut [Option<Box<dyn MenuType>>; 5], visualiser: &mut Visualiser, index: usize, font: Font) {
        match &mut menus[index] {
            None => {
                menus[index] = self.create_menu(visualiser, index, font)
            },
            Some(m) => {
                m.as_mut().update(visualiser)
            }
        }
    }

    /// updates the menu for the current state
    fn update_state(&self, menus: &mut [Option<Box<dyn MenuType>>; 5], visualiser: &mut Visualiser, font: Font) {
        match self {
            MenuState::General => {
                self.update_state_menu(menus, visualiser, 0, font)
            },
            _ => {}
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
    menus: [Option<Box<dyn MenuType>>; 5],
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
                (10., 10.),
                load_image("assets/triangle.png").await.unwrap(),
                true,
                10.,
                PINK,
                2.5,
                BLACK,
                GRAY,
                BLACK
            ),
            close_button: Button::new(
                (20., 20.), 
                (MENU_SCREEN_PROPORTION*screen_width() + 10., 10.),
                load_image("assets/triangle.png").await.unwrap(),
                false,
                10.,
                PINK,
                2.5,
                BLACK,
                GRAY,
                BLACK
            ),
            navbar: Navbar::new().await,
            menus: [None, None, None, None, None]
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

    /// updates the menu every frame
    pub fn update(&mut self, visualiser: &mut Visualiser) {
        match self.state {
            MenuState::Closed => {
                self.menu_state_closed(visualiser);
                return;
            },
            _ => {}
        }

        // main menu code here
        if self.gradient.width() == 0. { 
            // this will eventually change so the gradient changes when a layer is changed
            self.gradient = get_back_gradient(
                &visualiser, 
                0, 
                (MENU_SCREEN_PROPORTION*screen_width()) as u16, 
                (NAVBAR_HEIGHT_PROPORTION*screen_height()) as u16
            );
            self.navbar.back = self.gradient;
            self.text_colour = get_brightest_colour(self.gradient);
        }

        self.state.draw_state(self.state_font, self.text_colour);
        self.state.update_state(&mut self.menus, visualiser, self.text_font);

        self.state = self.navbar.update(self.state);

        self.close_button.update();
        if self.close_button.clicked {
            self.close_menu(visualiser);
        }
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

/// a button that can be clicked to be interacted with
pub struct Button {
    /// the image in the middle of the button
    image: Texture2D,
    image_params: DrawTextureParams,
    image_size: (f32, f32),
    border_colour: Color,
    /// the width of the border all over
    border_size: f32,
    unhover_colour: Color,
    hover_colour: Color,
    hold_colour: Color,
    colour: Color,
    rect: Rect,
    /// whether the button has been clicked this frame
    clicked: bool
}
impl Button {
    fn new(
        size: (f32, f32),
        center_pos: (f32, f32),
        image: Image,
        image_flip_x: bool,
        image_padding: f32,
        border_colour: Color,
        border_size: f32,
        unhover_colour: Color,
        hover_colour: Color,
        hold_colour: Color
    ) -> Button {
        let image_size = (
            f32::min(size.0 - image_padding, image.width as f32),
            f32::min(size.1 - image_padding, image.height as f32)
        );
        Button { border_colour, border_size, unhover_colour, hover_colour, hold_colour, image_size,
            colour: unhover_colour,
            image: Texture2D::from_image(&image),
            image_params: DrawTextureParams {
                dest_size: Some(vec2(
                    image_size.0,
                    image_size.1
                )),
                flip_x: image_flip_x,
                ..Default::default()
            },
            rect: Rect::new(
                center_pos.0 - size.0 * 0.5,
                center_pos.1 - size.1 * 0.5,
                size.0,
                size.1
            ),
            clicked: false
        }
    }

    /// call while the button is active to carry out its tasks
    fn update(&mut self) {
        self.draw();
        self.mouse_interact()
    }

    /// draw the button to the screen
    fn draw(&self) {
        // draw border
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            self.border_colour
        );
        // draw background
        draw_rectangle(
            self.rect.x + self.border_size,
            self.rect.y + self.border_size,
            self.rect.w - self.border_size * 2.,
            self.rect.h - self.border_size * 2.,
            self.colour
        );
        // draw image
        draw_texture_ex(
            self.image, 
            self.rect.x + self.rect.w/2. - self.image_size.0/2., 
            self.rect.y + self.rect.h/2. - self.image_size.1/2., 
            WHITE,
            self.image_params.clone(), 
        );
    }

    /// controls mouse interaction with the buttons
    fn mouse_interact(&mut self) {
        self.clicked = false;

        if !self.rect.contains(Vec2::from(mouse_position())) {
            self.colour = self.unhover_colour;
            return;
        }

        self.colour = self.hover_colour;
        if is_mouse_button_pressed(MouseButton::Left) {
            self.clicked = true;
        }
        if is_mouse_button_down(MouseButton::Left) {
            self.colour = self.hold_colour;
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

    /// called once per frame to draw and change the navbar
    /// 
    /// returns the menu state the menu should be in
    fn update(&mut self, menu_state: MenuState) -> MenuState {
        draw_texture(self.back, 0., 0., WHITE);
        for button in self.buttons.iter_mut() {
            button.update()
        }

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

trait MenuType {
    fn update(&mut self, visualiser: &mut Visualiser);
}

/// generates the text boxes for the general menu
struct GeneralMenuTextBoxGenerator {
    labels: [&'static str;5],
    label_dims: [TextDimensions; 5],
    label_params: TextParams,
    default_data: [f64; 5],
    width: u16,
    start_x: u16,
    start_y: f32,
    y_change: f32,
    gradient: Texture2D,
    content_params: TextParams
}
impl GeneralMenuTextBoxGenerator {
    fn get_text_box(&self, i: usize) -> TextBox {
        TextBox::new(
            self.labels[i],
            self.label_dims[i],
            self.label_params,
            self.default_data[i],
            self.width,
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
        let width = (screen_width()*MENU_SCREEN_PROPORTION) as u16 
                         - start_x 
                         - (screen_width() * TEXTBOX_RIGHT_PADDING) as u16;
        
        let gradient = get_back_gradient(visualiser, start_x, width, (screen_height() * TEXTBOX_HEIGHT_PROPORTION) as u16);
        let label_params = TextParams {font: text_font, font_size, color: get_brightest_colour(gradient), ..Default::default()};
        let content_params = TextParams {font: text_font, font_size, color: WHITE, ..Default::default()};

        let generator = GeneralMenuTextBoxGenerator {
            labels, label_dims, label_params, default_data: [-0.5, 0., 1.0, 500., 200.], 
            start_x, width, start_y, y_change, gradient, content_params
        };

        GeneralMenu {
            center_re: generator.get_text_box(0),
            center_im: generator.get_text_box(1),
            magnification: generator.get_text_box(2),
            max_iterations: generator.get_text_box(3),
            bailout: generator.get_text_box(4),
        }
    }

    fn all_text_boxes(&mut self) -> [&mut TextBox; 5] {
        [
            &mut self.center_re, &mut self.center_im, &mut self.magnification, &mut self.max_iterations, &mut self.bailout
        ]
    }

    fn get_data(visualiser: &Visualiser, i: usize) -> f64 {
        if i == 0 {
            visualiser.center.real_f64()
        } else if i == 1 {
            visualiser.center.im_f64()
        } else if i == 2 {
            0.005/visualiser.pixel_step
        } else if i == 3 {
            visualiser.max_iterations as f64
        } else {
            20.0 // placeholder until bailout becomes dynamic
        }
    }
}
impl MenuType for GeneralMenu {
    fn update(&mut self, visualiser: &mut Visualiser) {
        for (i, text_box) in self.all_text_boxes().iter_mut().enumerate() {
            text_box.update(GeneralMenu::get_data(visualiser, i))
        }
    }
}

struct TextBox {
    label: String,
    label_dims: TextDimensions,
    label_params: TextParams,
    data: f64,
    border_back: Texture2D,
    outer_rect: Rect,
    inner_rect: Rect,
    content_params: TextParams
}
impl TextBox {
    fn new(
        label: &str,
        label_dims: TextDimensions, 
        label_params: TextParams,
        default_data: f64,
        width: u16, 
        start_x: u16, start_y: f32, 
        gradient: Texture2D,
        content_params: TextParams
    ) -> TextBox {
        let outer_rect = Rect::new(start_x as f32, start_y, width as f32, screen_height() * TEXTBOX_HEIGHT_PROPORTION);
        let border_width = screen_height() * TEXTBOX_BORDER_PROPORTION;
        TextBox {
            label: String::from(label), 
            data: default_data,
            label_dims, 
            label_params,
            border_back: gradient,
            outer_rect,
            inner_rect: Rect::new(outer_rect.x + border_width, outer_rect.y + border_width, 
                                   outer_rect.w-2.*border_width, outer_rect.h-2.*border_width),
            content_params
        }
    }

    /// draw and update the text box
    fn update(&mut self, data: f64) {
        draw_text_ex(
            &self.label, 
            screen_width()*TEXTBOX_LABEL_WIDTH_PADDING_PROPORTION, 
            self.outer_rect.y + self.outer_rect.h/2. + self.label_dims.height/2., 
            self.label_params,
        );
        draw_texture(self.border_back, self.outer_rect.x, self.outer_rect.y, WHITE);
        draw_rectangle(self.inner_rect.x, self.inner_rect.y, self.inner_rect.w, self.inner_rect.h, BLACK);

        self.data = data;

        let content = self.data.to_string();
        let content_dims = measure_text(&content, Some(self.content_params.font), self.content_params.font_size, 1.0);
        let letters = content.chars().count();
        let letter_width = content_dims.width / letters as f32;
        let to_use = letters.min(((self.inner_rect.w - 2.*screen_width()*TEXTBOX_CONTENT_PADDING)/letter_width) as usize);

        draw_text_ex(
            &content[0..to_use],
            self.inner_rect.x + screen_width()*TEXTBOX_CONTENT_PADDING,
            self.inner_rect.y + self.inner_rect.h/2. + content_dims.height/2.,
            self.content_params
        );
    }
}