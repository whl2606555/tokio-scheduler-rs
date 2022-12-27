use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum SchedulerErrorKind{
    AddJobErr(String),
    DeleteJobErr(String),
    HasJobErr(String)
}

#[derive(Debug)]
pub struct SchedulerError{
    error_kind:SchedulerErrorKind
}

impl SchedulerError{
    pub fn new(error_kind:SchedulerErrorKind)->Self{
        Self{
            error_kind
        }
    }
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"SchedulerError:\nKind:{:#?}",self.error_kind)
    }
}

impl std::error::Error for SchedulerError{

}

pub struct TaskError{

}