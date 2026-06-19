use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use dnsmasqweb::storage::backup;

#[tokio::test]
async fn delete_backup_removes_matching_backup_file() {
    let dir = temp_backup_dir("delete-backup");
    fs::create_dir_all(&dir).expect("create temp backup dir");
    let id = "20260619-154500000000000";
    let path = dir.join(format!("dnsmasq.conf.{id}"));
    fs::write(&path, "address=/app.example.internal/10.10.0.1\n").expect("write backup");

    backup::delete_backup(&dir, id)
        .await
        .expect("delete backup");

    assert!(!path.exists());
    let _ = fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn delete_backup_rejects_path_traversal_ids() {
    let dir = temp_backup_dir("delete-backup-traversal");
    fs::create_dir_all(&dir).expect("create temp backup dir");

    let error = backup::delete_backup(&dir, "../dnsmasq.conf")
        .await
        .expect_err("reject traversal id");

    assert!(error.to_string().contains("invalid backup id"));
    let _ = fs::remove_dir_all(&dir);
}

fn temp_backup_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("dnsmasqweb-{name}-{}-{nanos}", std::process::id()))
}
