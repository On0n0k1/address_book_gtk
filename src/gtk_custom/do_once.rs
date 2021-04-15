// Use in windows. Return false only once. Then it always return true.
pub struct DoOnce{
    done: bool,
}

impl DoOnce{
    pub fn new() -> Self {
        DoOnce{
            done: false,
        }
    }

    pub fn is_done(&mut self) -> bool {
        match self.done{
            true => true,
            false => {
                self.done = true;
                false
            }
        }
    }
}