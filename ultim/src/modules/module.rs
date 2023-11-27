use downcast_rs::{impl_downcast, Downcast};

pub trait Module: Downcast {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}
impl_downcast!(Module);
