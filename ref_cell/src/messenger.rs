use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

// #[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: usize,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        let percentage = count * 100 / self.max;

        if percentage >= 100 {
            self.logger.error(&format!("Error: you are over your quota!"));
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
        
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        let percentage = count * 100 / self.max;
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage
        ))
    }
  
}