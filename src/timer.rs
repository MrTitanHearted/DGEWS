use std::time::*;

/// Timer sturct to retrieve current time
/// 
/// # Example
/// 
/// ```ignore
/// let mut timer = Timer::new();
/// println!("Time: {}", time.time());
/// 
/// std::time::sleep(Duration::from_secs(10));
/// time.update();
/// println!("Time: {}", time.time());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timer {
    pub(crate) instant: Instant,
    pub(crate) current_frame: f32,
}

impl Timer {
    /// Creates a new instance of the Time struct
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let timer = Timer::new();
    /// ```
    pub fn new() -> Self {
        return Self {
            instant: Instant::now(),
            current_frame: 0.0f32,
        };
    }

    /// Updates the Timer struct's time
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let mut timer = Timer::new();
    /// 
    /// std::time::sleep(Duration::from_secs(5));
    /// 
    /// timer.update();
    /// ```
    pub fn update(&mut self) {
        self.current_frame = self.time();
    }
    
    /// Retrieves the delta time. (dt() function updates the this instance as well)
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let mut timer = Timer::new();
    /// std::time::sleep(Duration::from_secs(10));
    /// timer.update();
    /// std::time::sleep(Duration::from_secs(10));
    /// println!("dt: {}", time.dt());
    /// ```
    pub fn dt(&mut self) -> f32 {
        let current_frame = self.time();
        let dt = current_frame - self.current_frame;
        self.update();
        return dt;
    }

    /// Retrieves the last time when an instance was updated
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let mut timer = Timer::new();
    /// assert_eq!(timer.current_frame(), 0.0);
    /// 
    /// std::time::sleep(Duration::from_secs(10));
    /// assert_eq!(timer.current_frame(), 0.0);
    /// 
    /// timer.update();
    /// assert_neq!(timer.current_frame(), 0.0);
    /// ```
    pub fn current_frame(&self) -> f32 {
        return self.current_frame;
    }

    /// Retrieves the exact current time
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let mut timer = Timer::new();
    /// 
    /// for _ in .. {
    ///     println!("Current time: {}", timer.time());
    /// }
    /// ```
    pub fn time(&self) -> f32 {
        return self.instant.elapsed().as_secs_f32();
    }
}
