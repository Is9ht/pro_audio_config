//! Main integration tests for Pro Audio Config

mod common;
mod audio_integration;
mod ui_integration;

// Re-export for easy access
pub use audio_integration::*;
pub use ui_integration::*;
pub use common::*;

#[test]
fn test_library_integration() {
    // Basic smoke test to ensure all modules work together
    use pro_audio_config::audio::AudioSettings;
    use pro_audio_config::config::apply_audio_settings_with_auth_blocking;
    
    let settings = AudioSettings::new(48000, 24, 512, "default".to_string());
    assert!(settings.validate().is_ok());
    
    // Test that the function signature works (without actually running privileged commands)
    let result = std::panic::catch_unwind(|| {
        let _ = apply_audio_settings_with_auth_blocking(settings);
    });
    assert!(result.is_ok());
}
