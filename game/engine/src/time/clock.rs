use std::time::{Duration, Instant};

/// Часы, которые засекают время, проходящее между вызовами метода get_time
/// Запомнинают время своего создания, либо время последнего вызова get_time
/// А потом возвращают прошедшее с этого момента время, после чего
/// Запоминают момент когда это прошедшее время считали.
pub struct Clock {
    instant: Instant,
}

impl Clock {
    /// Создаёт новый экземпляр Clock и засекает время с его создания
    pub fn new() -> Clock {
        Clock {instant: Instant::now()}
    }
    /// Возвращает прошедшее время с последнего вызова этой функции (либо с момента создания Clock)
    pub fn get_time(&mut self) -> Duration {
        let dt = self.instant.elapsed();
        self.instant = Instant::now();
        return dt;
    }
}