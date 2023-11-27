use ultim::Request;

pub trait Panel {
    fn name(&self) -> &'static str;
    fn category(&self) -> &'static str;

    fn human_name(&self, request: &Request) -> String;
}
