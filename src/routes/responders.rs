
use rocket::{Response, Request};
use rocket::http::Status;
use rocket::response::Responder;

#[derive(Responder)]
#[responce(status = 500, content_type = "json")]
pub struct MyResponder {
    inner: OtherResponder,
    header: SomeHeader,
    more: YetAnotherHeader,
    #[response(ignore)]
    unrelated: MyType,
}

impl Responder<'static> fro Strin {

    pub fn respone_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(self))
            .ok()
    }
}