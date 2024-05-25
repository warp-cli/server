use thiserror::Error;
use url::Url;

/// Application configurations.
pub struct Config {}

impl Config {
    pub fn load() -> Result<Self, LoadError> {
        // Get configurations provider.
        let mut args = std::env::args_os().skip(1);

        while let Some(name) = args.next() {
            if name != "--config" {
                continue;
            }

            // Parse the value.
            let value = args.next().ok_or(LoadError::NoProvider)?;
            let url = value
                .to_str()
                .ok_or(LoadError::InvalidProvider)?
                .parse::<Url>()
                .map_err(|_| LoadError::InvalidProvider)?;

            // Select configurations provider.
            return match url.scheme() {
                "args" => Self::from_args(Some(url)),
                v => Err(LoadError::UnknownProvider(v.to_owned())),
            };
        }

        Self::from_args(None)
    }

    fn from_args(_: Option<Url>) -> Result<Self, LoadError> {
        Ok(Self {})
    }
}

/// Represents an error when [`Config::load()`] fails.
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("no value is provided for --config")]
    NoProvider,

    #[error("the value for --config is not a valid URL")]
    InvalidProvider,

    #[error("unknown configurations provider '{0}'")]
    UnknownProvider(String),
}
