use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Returned when `local_ip` is unable to find the system's local IP address
    /// in the collection of network interfaces
    LocalIpAddressNotFound,
    /// Returned when an error occurs in the strategy level.
    /// The error message may include any internal strategy error if available
    StrategyError(String),
    /// Returned when the current platform is not yet supported
    PlatformNotSupported(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LocalIpAddressNotFound => write!(
                f,
                "The Local IP Address wasn't available in the network interfaces list/table"
            ),
            Error::StrategyError(msg) => write!(
                f,
                "An error occurred executing the underlying strategy error.\n{}",
                msg
            ),
            Error::PlatformNotSupported(platform) => {
                write!(f, "The current platform: `{}`, is not supported", platform)
            }
        }
    }
}

impl std::error::Error for Error {}
