pub trait Layered {
    fn get_layer(&self) -> u32;
    fn get_layer_mut(&mut self) -> &mut u32;
    fn change_layer(&mut self, layer: u32) {
        *self.get_layer_mut() = layer;
    }
    fn increase_layer(&mut self) {
        self.change_layer(self.get_layer()+1)
    }
    fn decrease_layer(&mut self) {
        self.change_layer(self.get_layer()-1)
    }
    fn at_layer(mut self, layer:u32) -> Self
    where Self: Sized
    {
        self.change_layer(layer);
        self
    }
}

pub trait FrameList {
    fn get_frames(&self) -> (u32, u32);
    fn get_frames_mut(&mut self) -> (&mut u32, &mut u32);
    fn set_frames(&mut self, frames_h: u32, frames_v: u32) {
        let (h, v) = self.get_frames_mut();
        *h = frames_h;
        *v = frames_v;
    }
    fn with_frames(mut self, frames_h: u32, frames_v: u32) -> Self
    where Self: Sized
    {
        self.set_frames(frames_h, frames_v);
        self
    }
}

pub trait HasTexture {
    type Texture;
    fn get_texture(&self) -> &Self::Texture;
    fn get_texture_mut(&mut self) -> &mut Self::Texture;
}