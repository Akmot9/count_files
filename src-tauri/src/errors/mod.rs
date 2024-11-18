
use walkdir::Error as WalkdirError;

// Create a custom Error that we can return in Results
#[derive(Debug, thiserror::Error)]
pub enum SurimiError {
    #[error("Walkdir error: {0}")]
    Walkdir(#[from] WalkdirError),
    #[error("Channel send error: {0}")]
    ChannelSendError(String),
}

impl serde::Serialize for SurimiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
