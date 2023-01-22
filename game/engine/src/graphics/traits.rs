pub trait Layered {
    fn get_layer_mut(&mut self) -> &mut u32;
    fn get_layer(&self) -> u32;
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