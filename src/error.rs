use std::fmt::Display;

/// Represents an error caused by invalid user interaction or input.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct UserError(String);

impl UserError {
    #[cfg_attr(track_caller, track_caller)]
    pub fn bail<T, D>(msg: D) -> Result<T, UserError>
    where D: Display + Send + Sync + 'static {
        Err(Self(msg.to_string()))
    }
}

pub trait WrapUserError<T, E> {
    #[cfg_attr(track_caller, track_caller)]
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Send + Sync + 'static;

    #[cfg_attr(track_caller, track_caller)]
    fn mark_as_user_err(self) -> Result<T, UserError>;
}

pub trait OptionUserError<T> {
    #[cfg_attr(track_caller, track_caller)]
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Send + Sync + 'static;
}

impl<T, E> WrapUserError<T, E> for Result<T, E>
where E: Display + Send + Sync + 'static
{
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(UserError(msg.to_string())),
        }
    }

    fn mark_as_user_err(self) -> Result<T, UserError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(UserError(e.to_string())),
        }
    }
}

impl<T> OptionUserError<T> for Option<T> {
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(UserError(msg.to_string())),
        }
    }
}
