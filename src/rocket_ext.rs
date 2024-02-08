use rocket::http::Header;

#[derive(Responder)]
#[response(status = 200, content_type = "html")]
pub(crate) enum HtmxResponder {
    Ok(String),
    Retarget {
        content: String,
        retarget: Header<'static>,
        reswap: Header<'static>,
    },
}
