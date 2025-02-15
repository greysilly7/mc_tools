use std::{
    process,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn log_info(message: &str) {
    println!("[INFO] {} - {}", current_time(), message);
}

pub fn log_error(message: &str) {
    eprintln!("[ERROR] {} - {}", current_time(), message);
}

pub fn current_time() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    let nanos = now.subsec_nanos();
    format!(
        "{:02}:{:02}:{:02}.{:09}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60,
        nanos
    )
}

pub fn usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  Backup:");
    eprintln!(
        "    {} backup --server-dir /path/to/world --backup-dir /path/to/backups [--server-session tmux_session]",
        program_name
    );
    eprintln!("  Restore:");
    eprintln!(
        "    {} restore --backup-source /path/to/backups/backup_timestamp --restore-dir /path/to/world",
        program_name
    );
    eprintln!("  Purge:");
    eprintln!(
        "    {} purge --backup-dir /path/to/backups [--retention-days N]",
        program_name
    );
    process::exit(1);
}
