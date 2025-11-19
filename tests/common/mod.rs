//! Common utilities for integration tests

use pro_audio_config::audio::AudioSettings;

/// Create test audio settings with valid defaults
pub fn create_test_settings() -> AudioSettings {
    AudioSettings::new(48000, 24, 512, "default".to_string())
}

/// Create test audio settings with specific values
pub fn create_custom_settings(sample_rate: u32, bit_depth: u32, buffer_size: u32, device_id: &str) -> AudioSettings {
    AudioSettings::new(sample_rate, bit_depth, buffer_size, device_id.to_string())
}
