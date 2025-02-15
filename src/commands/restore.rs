use crate::utils::{log_error, log_info, usage};
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io::{self};
use tar::Archive;

pub fn restore(args: &[String]) {
    let mut backup_source = String::new();
    let mut restore_dir = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--backup-source" => {
                if i + 1 < args.len() {
                    backup_source = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --backup-source");
                    usage(&args[0]);
                }
            }
            "--restore-dir" => {
                if i + 1 < args.len() {
                    restore_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --restore-dir");
                    usage(&args[0]);
                }
            }
            _ => {
                log_error(&format!("Unknown option: {}", args[i]));
                usage(&args[0]);
            }
        }
    }

    if backup_source.is_empty() || restore_dir.is_empty() {
        log_error("Missing required options for restore (--backup-source and --restore-dir).");
        usage(&args[0]);
    }

    log_info(&format!(
        "Restoring from '{}' to '{}'",
        backup_source, restore_dir
    ));

    // Check if the backup source is a compressed file
    if backup_source.ends_with(".tar.gz") {
        // Restore from a compressed tar.gz file
        if let Err(e) = restore_from_tar_gz(&backup_source, &restore_dir) {
            log_error(&format!(
                "Failed to restore from '{}': {}",
                backup_source, e
            ));
            return;
        }
    } else {
        // Restore from an uncompressed directory or file
        if let Err(e) = fs::copy(&backup_source, &restore_dir) {
            log_error(&format!(
                "Failed to copy from '{}' to '{}': {}",
                backup_source, restore_dir, e
            ));
            return;
        }
    }

    log_info("Restore completed successfully.");
}

fn restore_from_tar_gz(backup_source: &str, restore_dir: &str) -> io::Result<()> {
    // Create the restore directory if it does not exist
    fs::create_dir_all(restore_dir)?;

    // Open the compressed tar.gz file
    let tar_gz = File::open(backup_source)?;
    let decoder = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decoder);

    // Extract the contents of the archive to the restore directory
    archive.unpack(restore_dir)?;

    Ok(())
}
