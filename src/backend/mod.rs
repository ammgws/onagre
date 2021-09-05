use pop_launcher::{Request, Response};
use pop_launcher::Indice;

pub mod launcher;

#[derive(Debug, Clone)]
pub enum  PopRequest {
    Activate(Indice),
    ActivateContext {
        id: Indice,
        context: Indice,
    },
    Complete(Indice),
    Context(Indice),
    Exit,
    Interrupt,
    Quit(Indice),
    Search(String)
}

impl From<Request> for PopRequest {
    fn from(request: Request) -> Self {
        match request {
            Request::Activate(id) => PopRequest::Activate(id),
            Request::ActivateContext { id, context} => PopRequest::ActivateContext {
                id,
                context
            },
            Request::Complete(id) => PopRequest::Complete(id),
            Request::Context(id) => PopRequest::Context(id),
            Request::Exit => PopRequest::Exit,
            Request::Interrupt => PopRequest::Interrupt,
            Request::Quit(id) => PopRequest::Exit,
            Request::Search(search_request) => PopRequest::Search(search_request),
        }
    }
}


#[derive(Debug, Clone)]
pub enum  PopResponse {
    Close,
    Context {
        id: Indice,
        context: Vec<PopGpuPreference>,
    }
    DesktopEntry
    Update
    Fill
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PopGpuPreference {
    Default,
    NonDefault,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PopContextOption {
    pub id: Indice,
    pub name: String,
}

impl From<Response> for PopResponse {
    fn from(response: Response) -> Self {
        match response {
            Response::Close =>
            Response::Context { id, options } => {}
            Response::DesktopEntry { path, gpu_preference } => {}
            Response::Update(update) => {}
            Response::Fill(fill) => {}
        }
    }
}