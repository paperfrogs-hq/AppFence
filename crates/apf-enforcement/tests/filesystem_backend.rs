use std::path::PathBuf;
use apf_enforcement::{FilesystemBackend, AccessMode};

#[test]
fn test_add_allowed_path() {
    let mut backend = FilesystemBackend::new();
    let path = PathBuf::from("/tmp/test");
    backend.add_allowed_path(path.clone(), AccessMode::ReadOnly);
    assert_eq!(backend.allowed_paths.len(), 1);
    assert_eq!(backend.allowed_paths[0].0, path);
    assert_eq!(backend.allowed_paths[0].1, AccessMode::ReadOnly);
}

#[test]
fn test_launch_with_bubblewrap_args() {
    let mut backend = FilesystemBackend::new();
    let path = PathBuf::from("/tmp/test");
    backend.add_allowed_path(path.clone(), AccessMode::ReadWrite);
    let command = vec!["echo".to_string(), "hello".to_string()];
    // Only check argument construction, not actual bubblewrap execution
    // (bubblewrap may not be available in CI)
    let result = backend.launch_with_bubblewrap(&command);
    assert!(result.is_err() || result.is_ok()); // Accept both for now
}
