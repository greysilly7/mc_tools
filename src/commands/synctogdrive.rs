use std::{
    fs::{self, File},
    process::{self, Command},
};

use flate2::{Compression, write::GzEncoder};

use crate::utils::{current_time, log_error, log_info, usage};

pub fn synctogdrive(args: &[String]) {
    let mut root_dir = String::new();
    let mut remote_path = String::from("gdrive:"); // Default remote path

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--root-dir" => {
                if i + 1 < args.len() {
                    root_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --root-dir");
                    usage(&args[0]);
                }
            }
            "--remote-path" => {
                if i + 1 < args.len() {
                    remote_path = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --remote-path");
                    usage(&args[0]);
                }
            }
            _ => {
                log_error(&format!("Unknown option: {}", args[i]));
                usage(&args[0]);
            }
        }
    }

    if root_dir.is_empty() {
        log_error("Missing required option --root-dir for syncing.");
        usage(&args[0]);
    }

    log_info(&format!(
        "Iterating over '{}' to compress dated backups and sync to '{}'",
        root_dir, remote_path
    ));

    // Iterate over each server directory in the root directory
    for server_entry in fs::read_dir(&root_dir).unwrap() {
        let server_entry = server_entry.unwrap();
        let server_path = server_entry.path();

        if server_path.is_dir() {
            // Iterate over each dated backup directory
            for dated_entry in fs::read_dir(&server_path).unwrap() {
                let dated_entry = dated_entry.unwrap();
                let dated_backup_path = dated_entry.path();

                if dated_backup_path.is_dir() {
                    // Create a compressed tar.gz file for the dated backup
                    let timestamp = current_time().replace(":", "_").replace(".", "_");
                    let compressed_backup = format!(
                        "{}/{}_{}.tar.gz",
                        server_path.display(),
                        dated_entry.file_name().to_string_lossy(),
                        timestamp
                    );
                    let tar_gz = File::create(&compressed_backup).unwrap_or_else(|_| {
                        log_error(&format!(
                            "Failed to create backup file: {}",
                            compressed_backup
                        ));
                        process::exit(1)
                    });

                    let mut encoder = GzEncoder::new(tar_gz, Compression::default());
                    let mut archive = tar::Builder::new(&mut encoder);

                    // Add files to the archive
                    for entry in fs::read_dir(&dated_backup_path).unwrap() {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.is_file() {
                            log_info(&format!("Adding {:?} to compressed backup", path));
                            archive.append_path(&path).unwrap_or_else(|_| {
                                log_error(&format!(
                                    "Failed to add {:?} to compressed backup",
                                    path
                                ));
                            });
                        }
                    }

                    // Finish the archive
                    if let Err(e) = archive.finish() {
                        log_error(&format!("Failed to finish compressed backup: {}", e));
                        process::exit(1);
                    }

                    log_info("Compressed backup created successfully.");

                    // Execute the rclone sync command
                    let output = Command::new("rclone")
                        .args(&["copy", &compressed_backup, &remote_path])
                        .output()
                        .expect("Failed to execute rclone command");

                    if output.status.success() {
                        log_info("Sync to Google Drive completed successfully.");
                    } else {
                        let error_message = String::from_utf8_lossy(&output.stderr);
                        log_error(&format!(
                            "Failed to sync to Google Drive: {}",
                            error_message
                        ));
                    }

                    // Remove the compressed backup file after syncing
                    if let Err(e) = fs::remove_file(&compressed_backup) {
                        log_error(&format!(
                            "Failed to remove compressed backup file: {}: {}",
                            compressed_backup, e
                        ));
                    } else {
                        log_info(&format!(
                            "Removed compressed backup file: {}",
                            compressed_backup
                        ));
                    }
                }
            }
        }
    }
}
