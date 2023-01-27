pub trait Named {
    fn get_name(&self) -> &Option<String>;
    fn get_name_mut(&mut self) -> &mut Option<String>;
    fn named(mut self, name: &str) -> Self
    where Self:Sized
    {
        self.get_name_mut().replace(name.to_string());
        self
    }
}