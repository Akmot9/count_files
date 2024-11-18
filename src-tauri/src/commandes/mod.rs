use serde::Serialize;
use walkdir::WalkDir;
use log::{error, info};
use std::path::Path;
use tauri::{command, ipc::Channel};
use crate::errors::SurimiError;

#[command(async)]
pub fn get_file_count(on_event: Channel<FileCountEvent>) {
    let root_path = if cfg!(target_os = "windows") {
        Path::new("C:\\")
    } else {
        Path::new("/")
    };

    let mut file_count = 0;
    let mut last_reported_count = 0;

    for entry in WalkDir::new(root_path)
        .follow_links(false)
        .into_iter()
    {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    file_count += 1;

                    // Échantillonnage : envoyer une mise à jour tous les 100 fichiers
                    if file_count - last_reported_count >= 25_000 {
                        last_reported_count = file_count;
                        on_event
                            .send(FileCountEvent::Progress {
                                count: file_count,
                                current_path: entry.path().to_str().unwrap_or("unknown"),
                            })
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to send progress event: {}", e);
                            });
                    }
                }
            }
            Err(e) => {
                let error_message = format!("Error accessing path: {}", e);
                on_event
                    .send(FileCountEvent::Error {
                        message: error_message,
                    })
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to send error event: {}", e);
                    });
            }
        }
    }

    // Envoyer un événement de fin
    on_event
        .send(FileCountEvent::Finished {
            total_count: file_count,
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to send finished event: {}", e);
        });
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum FileCountEvent<'a> {
    #[serde(rename_all = "camelCase")]
    Progress {
        count: usize,
        current_path: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    Finished {
        total_count: usize,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        message: String,
    },
}
