use std::ops::{AddAssign, RemAssign, SubAssign};

/// Что-то, что периодически(или единожды) обновляется в зависимости от прошедшего времени.
pub trait Updatable {
    type Host;
    /// Возвращает время, прошедшее после последнего обновления
    fn get_time_elapsed(&self) -> u32;
    /// Возвращает изменяемую ссылку на время, прошедшее после последнего обновления
    fn get_time_elapsed_mut(&mut self) -> &mut u32;
    fn get_delay(&self) -> u32;
    fn updated_internal(self, mult: u32, host: &mut Self::Host) -> Self where Self: Sized;
    fn updated(mut self, dt: u32, host: &mut Self::Host) -> Self
    where Self: Sized
    {
        self.get_time_elapsed_mut().add_assign(dt);
        let time_elapsed = self.get_time_elapsed();
        let delay = self.get_delay();
        if time_elapsed >= delay {
            let mult = time_elapsed // in case if multiple updates happened
                .checked_div(delay)
                .unwrap_or(0);
            if delay > 0 {
                self.get_time_elapsed_mut().rem_assign(delay);
            }
            return self.updated_internal(mult, host);

        }
        self
    }
}

// pub trait UpdatableGroup {
//     type Item: Updatable;
//     fn get_group(&self) -> &Group<Self::Item>;
//     fn get_group_mut(&mut self) -> &mut Group<Self::Item>;
//     fn updated(mut self, dt: u32, host: Self::Item::Host)
//         where Self:Sized
//     {
//         self.get_group_mut().call(|item| {
//             item.updated(dt, host)
//         })
//     }
// }