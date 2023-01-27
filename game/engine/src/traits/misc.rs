pub trait Named {
    /// Получает неизменяемую ссылку на имя, вернее &Option<String>
    fn get_name(&self) -> &Option<String>;
    /// Получает изменяемую ссылку на имя, вернее &mut Option<String>, т.к имени может не быть
    fn get_name_mut(&mut self) -> &mut Option<String>;
    /// Возвращает объект с установленным новым именем
    fn named(mut self, name: &str) -> Self
    where Self:Sized
    {
        self.get_name_mut().replace(name.to_string());
        self
    }
}