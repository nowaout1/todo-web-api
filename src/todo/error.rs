use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Error {
    #[error("invalid argument")]
    InvalidArgument,

    #[error("internal server error")]
    Internal,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::InvalidArgument => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Self {
        use tonic::Code::*;

        match status.code() {
            InvalidArgument => Self::InvalidArgument,
            Internal => {
                tracing::error!("Todo error: {status:?}");
                Self::Internal
            }

            Ok => todo!(),
            AlreadyExists | FailedPrecondition | OutOfRange | Unimplemented | Unavailable => {
                todo!()
            }
            NotFound | Aborted | Cancelled | Unknown | DeadlineExceeded | DataLoss => todo!(),
            Unauthenticated | PermissionDenied | ResourceExhausted => todo!(),
        }
    }
}
