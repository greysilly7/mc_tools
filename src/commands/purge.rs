use crate::utils::{log_error, log_info, usage};

pub fn purge(args: &[String]) {
    let mut backup_dir = String::new();
    let mut retention_days = 7;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--backup-dir" => {
                if i + 1 < args.len() {
                    backup_dir = args[i + 1].clone();
                    i += 2;
                } else {
                    log_error("Missing value for --backup-dir");
                    usage(&args[0]);
                }
            }
            "--retention-days" => {
                if i + 1 < args.len() {
                    retention_days = args[i + 1].parse().unwrap_or(7);
                    i += 2;
                } else {
                    log_error("Missing value for --retention-days");
                    usage(&args[0]);
                }
            }
            _ => {
                log_error(&format!("Unknown option: {}", args[i]));
                usage(&args[0]);
            }
        }
    }

    if backup_dir.is_empty() {
        log_error("Missing required option --backup-dir for purge.");
        usage(&args[0]);
    }

    log_info(&format!(
        "Purging backup directories in '{}' older than {} days.",
        backup_dir, retention_days
    ));
    // Implement purge logic here
}
