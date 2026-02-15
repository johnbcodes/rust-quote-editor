use rocket::{
    Request,
    response::{Debug, Responder, Result},
};

// Make our own error that wraps `anyhow::Error`.
pub(crate) struct AppError(anyhow::Error);

impl<'r> Responder<'r, 'r> for AppError {
    fn respond_to(self, request: &Request<'_>) -> Result<'r> {
        Debug(self.0).respond_to(request)
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
