use glium::texture::SrgbTexture2d;
use crate::component::group::ComponentsGroup;
use crate::component::traits::Composite;
use crate::Rect;
use crate::rect::Rectangular;
use crate::graphics::traits::{FrameList, HasTexture, Layered};
use crate::misc_traits::named::Named;

/// Спрайт - это текстура и квадрат, в котором эта текстура рисуется.
/// У спрайта есть слой, на котором он рисуется.
/// У спрайта есть имя
#[derive(Debug)]
pub struct Sprite {
    rect: Rect,
    texture: SrgbTexture2d,
    name: Option<String>,
    frames_h: u32,
    frames_v: u32,
    _cur_frame: u32,
    layer: u32,
    components: ComponentsGroup,
    _hidden: bool,
}

impl Sprite {
    pub fn new (rect: Rect, texture: SrgbTexture2d, name: Option<String>, frames_h: u32,
                frames_v: u32, _cur_frame: u32,
                layer: u32, components: ComponentsGroup, _hidden: bool) -> Self {
        Sprite { rect, texture, name, frames_h, frames_v, _cur_frame, layer,  components, _hidden}
    }
    pub fn updated(self) -> Sprite {
        return self
    }
}

impl HasTexture for Sprite
{
    type Texture = SrgbTexture2d;
    fn get_texture(&self) -> &Self::Texture {
        return &self.texture
    }
    fn get_texture_mut(&mut self) -> &mut Self::Texture {
        return &mut self.texture
    }
}

impl Rectangular for Sprite {
    fn get_rect(&self) -> &Rect {
        &self.rect
    }
    fn get_rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}

impl Layered for Sprite {
    fn get_layer(&self) -> u32 {
        self.layer
    }
    fn get_layer_mut(&mut self) -> &mut u32 {
        &mut self.layer
    }
}

impl FrameList for Sprite {
    fn get_frames(&self) -> (u32, u32) {
        (self.frames_h, self.frames_v)
    }
    fn get_frames_mut(&mut self) -> (&mut u32, &mut u32) {
        (&mut self.frames_h, &mut self.frames_v)
    }
}

impl Named for Sprite {
    fn get_name(&self) -> &Option<String> {
        &self.name
    }
    fn get_name_mut(&mut self) -> &mut Option<String> {
        &mut self.name
    }
}

impl Composite for Sprite {
    fn get_components(&self) -> &ComponentsGroup {
        &self.components
    }
    fn get_components_mut(&mut self) -> &mut ComponentsGroup {
        &mut self.components
    }
}
