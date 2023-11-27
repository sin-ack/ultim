pub trait View {
    fn dispatch(&self, request: &Request) -> Result<impl Responder, Box<dyn Error>> {
        match request.method() {
            &Method::GET => self.get(request),
            &Method::POST => self.post(request),
            &Method::PUT => self.put(request, PutPartial::Yes),
            &Method::PATCH => self.put(request, PutPartial::No),
            &Method::DELETE => self.delete(request),
            _ => Err(Box::new(HttpResponse::MethodNotAllowed())),
        }
    }

    fn get(&self, request: &Request) -> Result<impl Responder, Box<dyn Error>> {
        Err(Box::new(HttpResponse::MethodNotAllowed()))
    }

    fn post(&self, request: &Request) -> Result<impl Responder, Box<dyn Error>> {
        Err(Box::new(HttpResponse::MethodNotAllowed()))
    }

    fn put(
        &self,
        request: &Request,
        partial: PutPartial,
    ) -> Result<impl Responder, Box<dyn Error>> {
        Err(Box::new(HttpResponse::MethodNotAllowed()))
    }

    fn delete(&self, request: &Request) -> Result<impl Responder, Box<dyn Error>> {
        Err(Box::new(HttpResponse::MethodNotAllowed()))
    }
}

pub struct ListRoute {
    pub path: String,
    pub method: String,
    pub handler: fn(&Request) -> Result<HttpResponse, Box<dyn Error>>,
}

impl Route for ListRoute {}
