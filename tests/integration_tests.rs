//! Main integration tests for the audio config tool

use pro_audio_config::audio::{AudioSettings, detect_audio_device, detect_current_audio_settings};

#[test]
fn test_library_compiles_and_works() {
    // Basic smoke test to ensure the library can be used
    let settings = AudioSettings::new(48000, 24, 512, "default".to_string());
    assert_eq!(settings.sample_rate, 48000);
    assert_eq!(settings.bit_depth, 24);
    assert_eq!(settings.buffer_size, 512);
    assert_eq!(settings.device_id, "default");
}

#[test]
fn test_multiple_audio_settings() {
    // Test various combinations of audio settings
    let test_cases = vec![
        (44100, 16, 256, "default"),
        (48000, 24, 512, "alsa:default"),
        (96000, 32, 1024, "pipewire:123"),
        (192000, 24, 2048, "pulse:456"),
    ];

    for (sample_rate, bit_depth, buffer_size, device_id) in test_cases {
        let settings = AudioSettings::new(sample_rate, bit_depth, buffer_size, device_id.to_string());
        assert_eq!(settings.sample_rate, sample_rate);
        assert_eq!(settings.bit_depth, bit_depth);
        assert_eq!(settings.buffer_size, buffer_size);
        assert_eq!(settings.device_id, device_id);
    }
}

#[test]
fn test_detect_audio_device_integration() {
    // Integration test for device detection
    // This actually runs the system commands
    let result = detect_audio_device();
    
    // We can't guarantee success (depends on system state)
    // but we can verify the function runs and returns a Result
    match result {
        Ok(device_info) => {
            println!("Detected audio device: {}", device_info);
            assert!(!device_info.is_empty());
            // Should contain some identifiable information
            assert!(device_info.len() > 5);
        }
        Err(e) => {
            println!("Device detection failed (expected in some environments): {}", e);
            // This is acceptable in test environments without audio devices
        }
    }
}

#[test]
fn test_detect_current_audio_settings_integration() {
    // Test current settings detection
    let result = detect_current_audio_settings();
    
    match result {
        Ok(settings) => {
            println!("Current settings: {}Hz, {}bit, {} samples", 
                     settings.sample_rate, settings.bit_depth, settings.buffer_size);
            
            // Validate that settings are reasonable
            assert!(settings.sample_rate >= 8000 && settings.sample_rate <= 384000);
            assert!(settings.bit_depth == 16 || settings.bit_depth == 24 || settings.bit_depth == 32);
            assert!(settings.buffer_size >= 64 && settings.buffer_size <= 8192);
            assert!(!settings.device_id.is_empty());
        }
        Err(e) => {
            println!("Settings detection failed: {}", e);
            // Acceptable in environments without audio configuration access
        }
    }
}

#[test]
fn test_audio_settings_serialization() {
    // Test that settings can be formatted for display and logging
    let settings = AudioSettings::new(192000, 32, 2048, "pipewire:123".to_string());
    
    // Test debug formatting
    let debug_output = format!("{:?}", settings);
    assert!(debug_output.contains("192000"));
    assert!(debug_output.contains("32"));
    assert!(debug_output.contains("2048"));
    assert!(debug_output.contains("pipewire:123"));
    
    // Test display-like formatting
    let display_text = format!("{}Hz/{}bit/{}samples", 
        settings.sample_rate, settings.bit_depth, settings.buffer_size);
    assert!(display_text.contains("192000"));
    assert!(display_text.contains("32"));
    assert!(display_text.contains("2048"));
}

#[test]
fn test_edge_case_settings() {
    // Test edge cases that might occur in real usage
    let edge_cases = vec![
        // Very low values
        (8000, 8, 64, "default"),
        // Very high values  
        (384000, 32, 8192, "alsa:hw:0"),
        // Mixed case
        (44100, 32, 128, "pulse:default"),
    ];
    
    for (sample_rate, bit_depth, buffer_size, device_id) in edge_cases {
        let settings = AudioSettings::new(sample_rate, bit_depth, buffer_size, device_id.to_string());
        
        assert_eq!(settings.sample_rate, sample_rate);
        assert_eq!(settings.bit_depth, bit_depth);
        assert_eq!(settings.buffer_size, buffer_size);
        assert_eq!(settings.device_id, device_id);
        
        // Verify they can be used in string operations
        let _description = format!("{:?}", settings);
    }
}
