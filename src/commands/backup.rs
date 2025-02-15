use std::{fs, path::Path, process::{self, Command}};

use crate::{usage, utils::{current_time, log_error, log_info}};

pub fn backup(args: &[String]) {
    let mut world_dir = String::new();
    let mut backup_dir = String::new();
    let mut server_session = String::new();

    let mut i: usize = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--world-dir" => {
                if i + 1 < args.len() {
                    world_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --world-dir");
                    usage(&args[0]);
                }
            }
            "--backup-dir" => {
                if i + 1 < args.len() {
                    backup_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --backup-dir");
                    usage(&args[0]);
                }
            }
            "--server-session" => {
                if i + 1 < args.len() {
                    server_session = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --server-session");
                    usage(&args[0]);
                }
            }
            _ => {
                log_error(&format!("Unknown option: {}", args[i]));
                usage(&args[0]);
            }
        }
    }

    Command::new("tmux")
        .args(["send-keys", "-t", &server_session, "save-all", "Enter"])
        .output()
        .expect("Failed to execute tmux command");
    Command::new("tmux")
        .args(["send-keys", "-t", &server_session, "save-all", "Enter"])
        .output()
        .expect("Failed to execute tmux command");

    // Pause for 10 seconds to allow the server to save
    std::thread::sleep(std::time::Duration::from_secs(10));

    if world_dir.is_empty() || backup_dir.is_empty() {
        log_error("Missing required options for backup (--world-dir and --backup-dir).");
        usage(&args[0]);
    }

    // Create the backup directory if it does not exist.
    fs::create_dir_all(&backup_dir).unwrap_or_else(|_| {
        log_error(&format!(
            "Failed to create backup directory: {}",
            backup_dir
        ));
        process::exit(1);
    });

    // Create a new backup folder named by timestamp.
    let timestamp = current_time().replace(":", "_").replace(".", "_");
    let new_backup = format!("{}/{}", backup_dir, timestamp);
    log_info(&format!("Creating new backup: {}", new_backup));

    // Find the latest backup (if any) to use for hard-linking unchanged files.
    let latest = fs::read_dir(&backup_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .max_by_key(|entry| entry.path())
        .map(|entry| entry.path());

    // Create the new backup directory
    fs::create_dir(&new_backup).unwrap_or_else(|_| {
        log_error(&format!(
            "Failed to create new backup directory: {}",
            new_backup
        ));
        process::exit(1);
    });

    if let Some(latest) = latest {
        log_info(&format!(
            "Using previous backup ({:?}) for incremental backup with hard links.",
            latest
        ));
        // Use hard links for unchanged files
        hardlink_incremental_backup(&latest, &world_dir, &new_backup);
    } else {
        log_info("No previous backup found. Creating a full backup.");
        // Copy files for the first backup
        copy_files(&world_dir, &new_backup);
    }

    log_info("Backup completed successfully.");
}

fn hardlink_incremental_backup(latest: &Path, world_dir: &str, new_backup: &str) {
    // Iterate over the world directory and create hard links
    for entry in fs::read_dir(world_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let source_path = entry.path();
        let target_path = Path::new(new_backup).join(&file_name);

        // Check if the file exists in the latest backup
        let latest_file_path = latest.join(&file_name);
        if latest_file_path.exists() {
            // Create a hard link to the latest file
            if let Err(e) = fs::hard_link(&latest_file_path, &target_path) {
                log_error(&format!(
                    "Failed to create hard link for {:?}: {}",
                    latest_file_path, e
                ));
            } else {
                log_info(&format!("Created hard link for {:?}", target_path));
            }
        } else {
            // If the file does not exist in the latest backup, copy it
            copy_file(&source_path, &target_path);
        }
    }
}

fn copy_files(source: &str, target: &str) {
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let source_path = entry.path();
        let target_path = Path::new(target).join(&file_name);
        copy_file(&source_path, &target_path);
    }
}

fn copy_file(source: &Path, target: &Path) {
    if let Err(e) = fs::copy(source, target) {
        log_error(&format!(
            "Failed to copy {:?} to {:?}: {}",
            source, target, e
        ));
    } else {
        log_info(&format!("Copied {:?} to {:?}", source, target));
    }
}
