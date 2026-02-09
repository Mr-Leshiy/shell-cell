use std::fmt::{Debug, Display};

/// Represents an error caused by invalid user interaction or input.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct UserError(color_eyre::eyre::Error);

impl UserError {
    pub fn inner(self) -> color_eyre::eyre::Error {
        self.0
    }
}

impl UserError {
    #[cfg_attr(track_caller, track_caller)]
    pub fn bail<T, D>(msg: D) -> Result<T, UserError>
    where D: Display + Debug + Send + Sync + 'static {
        Err(Self(color_eyre::eyre::Error::msg(msg)))
    }

    #[cfg_attr(track_caller, track_caller)]
    pub fn wrap<D>(msg: D) -> UserError
    where D: Display + Debug + Send + Sync + 'static {
        Self(color_eyre::eyre::Error::msg(msg))
    }
}

pub trait WrapUserError<T, E> {
    /// Wraps the provided `msg` into the `UserError` type, drop the existing error
    #[cfg_attr(track_caller, track_caller)]
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Debug + Send + Sync + 'static;

    /// Wraps the provided `msg` into the `UserError` type, keeping the existing error as
    /// well.
    #[cfg_attr(track_caller, track_caller)]
    fn wrap_user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Debug + Send + Sync + 'static;

    /// Wraps the current error into the `UserError` type.
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
        D: Display + Debug + Send + Sync + 'static;
}

impl<T, E> WrapUserError<T, E> for Result<T, E>
where E: Display + Debug + Send + Sync + 'static
{
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Debug + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(UserError(color_eyre::eyre::Error::msg(msg))),
        }
    }

    fn wrap_user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Debug + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(UserError(color_eyre::eyre::Error::msg(e).wrap_err(msg))),
        }
    }

    fn mark_as_user_err(self) -> Result<T, UserError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(UserError(color_eyre::eyre::Error::msg(e))),
        }
    }
}

impl<T> OptionUserError<T> for Option<T> {
    fn user_err<D>(
        self,
        msg: D,
    ) -> Result<T, UserError>
    where
        D: Display + Debug + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(UserError(color_eyre::eyre::Error::msg(msg))),
        }
    }
}
