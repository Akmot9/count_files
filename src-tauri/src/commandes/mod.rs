use serde::Serialize;
use walkdir::WalkDir;
use log::error;
use std::path::Path;
use tauri::{command, ipc::Channel};
use crate::errors::SurimiError;

#[command(async)]
pub fn get_file_count(on_event: Channel<FileCountEvent>) -> Result<(), SurimiError> {
    let root_path = if cfg!(target_os = "windows") {
        Path::new("C:\\")
    } else {
        Path::new("/")
    };

    let mut file_count = 0;
    let mut error_count = 0;

    for entry in WalkDir::new(root_path)
        .follow_links(false) // Évite les cycles de liens symboliques
        .into_iter()
    {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    file_count += 1;

                    // Envoyer l'événement de progression
                    on_event
                        .send(FileCountEvent::Progress {
                            count: file_count,
                            current_path: entry.path().to_str().unwrap_or("unknown"),
                            error: None,
                            error_count: Some(error_count),
                        })
                        .map_err(|e| SurimiError::ChannelSendError(e.to_string()))?;
                }
            }
            Err(e) => {
                error_count += 1;
                let path = e.path().map(|p| p.display().to_string()).unwrap_or_else(|| "unknown".to_string());
                let error_message = format!("Cannot access path: {}: {}", path, e);

                // Log l'erreur
                error!("{}", error_message);

                // Envoyer l'événement avec l'erreur
                on_event
                    .send(FileCountEvent::Progress {
                        count: file_count,
                        current_path: "unknown",
                        error: Some(error_message),
                        error_count: Some(error_count),
                    })
                    .map_err(|e| SurimiError::ChannelSendError(e.to_string()))?;
            }
        }
    }

    // Envoyer l'événement de fin
    on_event
        .send(FileCountEvent::Finished {
            total_count: file_count,
        })
        .map_err(|e| SurimiError::ChannelSendError(e.to_string()))?;

    Ok(())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum FileCountEvent<'a> {
    #[serde(rename_all = "camelCase")]
    Progress {
        count: usize,
        current_path: &'a str,
        error: Option<String>,
        error_count: Option<usize>,
    },
    #[serde(rename_all = "camelCase")]
    Finished {
        total_count: usize,
    },
}
