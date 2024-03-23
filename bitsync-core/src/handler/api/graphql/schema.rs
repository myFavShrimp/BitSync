use std::error::Error;

pub mod private;
pub mod public;

trait FormattedStringError<T> {
    fn to_formatted_string_error(self) -> Result<T, String>;
}

impl<T, E> FormattedStringError<T> for std::result::Result<T, E>
where
    E: Send + Sync + Error + 'static,
{
    fn to_formatted_string_error(self) -> Result<T, String> {
        match self {
            Err(error) => Err(format!("{:#}", color_eyre::eyre::Report::new(error))),
            Ok(result) => Ok(result),
        }
    }
}
