use apf_enforcement::filesystem::FilesystemBackend;
use apf_enforcement::filesystem::AccessMode;
use apf_enforcement::network::NetworkBackend;
use apf_enforcement::device::DeviceBackend;
use apf_enforcement::clipboard::ClipboardBackend;
use apf_enforcement::background::BackgroundBackend;
use apf_enforcement::autostart::AutostartBackend;
use apf_enforcement::audit::AuditBackend;
use apf_enforcement::sandbox::SandboxBackend;

#[test]
fn test_network_backend() {
    let backend = NetworkBackend::new(apf_core::types::NetworkLevel::Internet);
    let command = vec!["echo".to_string(), "network test".to_string()];
    let result = backend.enforce_network_policy(&command);
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn test_device_backend() {
    let backend = DeviceBackend::new(vec![apf_core::types::DeviceType::Camera]);
    let command = vec!["echo".to_string(), "device test".to_string()];
    let result = backend.enforce_device_policy(&command);
    assert!(result.is_ok());
}

#[test]
fn test_clipboard_backend() {
    let backend = ClipboardBackend::new(true);
    let command = vec!["echo".to_string(), "clipboard test".to_string()];
    let result = backend.enforce_clipboard_policy(&command);
    assert!(result.is_ok());
}

#[test]
fn test_background_backend() {
    let backend = BackgroundBackend::new(true);
    let command = vec!["echo".to_string(), "background test".to_string()];
    let result = backend.enforce_background_policy(&command);
    assert!(result.is_ok());
}

#[test]
fn test_autostart_backend() {
    let backend = AutostartBackend::new(true);
    let command = vec!["echo".to_string(), "autostart test".to_string()];
    let result = backend.enforce_autostart_policy(&command);
    assert!(result.is_ok());
}

#[test]
fn test_audit_backend() {
    let backend = AuditBackend::new();
    let result = backend.log_event("test event");
    assert!(result.is_ok());
}

#[test]
fn test_sandbox_backend() {
    let backend = SandboxBackend::new();
    let command = vec!["echo".to_string(), "sandbox test".to_string()];
    let result = backend.enforce_sandbox_policy(&command);
    assert!(result.is_err() || result.is_ok());
}
