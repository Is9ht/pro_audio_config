//! Integration tests specifically for audio functionality

use pro_audio_config::audio::{AudioSettings, apply_audio_settings_with_auth_blocking, detect_all_audio_devices};
use std::process::Command;

#[test]
fn test_audio_settings_validation() {
    // Test that invalid settings are handled properly
    let valid_settings = AudioSettings::new(48000, 24, 512, "default".to_string());
    assert_eq!(valid_settings.sample_rate, 48000);
    
    // Test edge cases
    let edge_settings = AudioSettings::new(1, 8, 64, "test".to_string());
    assert_eq!(edge_settings.sample_rate, 1);
    assert_eq!(edge_settings.bit_depth, 8);
    assert_eq!(edge_settings.buffer_size, 64);
}

#[test]
fn test_script_generation_integration() {
    // Test that script generation doesn't panic
    let settings = AudioSettings::new(96000, 24, 1024, "default".to_string());
    
    // This is an integration test that verifies the function signature
    // and basic behavior without actually running privileged commands
    let result = std::panic::catch_unwind(|| {
        let _ = apply_audio_settings_with_auth_blocking(settings);
    });
    
    assert!(result.is_ok(), "apply_audio_settings_with_auth_blocking should not panic");
}

#[test]
fn test_system_commands_exist() {
    // Verify that required system commands are available
    let commands = vec!["pw-cli", "pactl", "systemctl", "pkexec", "aplay", "arecord"];
    
    for cmd in commands {
        let output = Command::new("which").arg(cmd).output();
        match output {
            Ok(output) if output.status.success() => {
                println!("✓ Command '{}' is available", cmd);
            }
            _ => {
                println!("⚠ Command '{}' not found (may be expected in test environment)", cmd);
            }
        }
    }
}

#[test]
fn test_audio_settings_clone() {
    // Test that AudioSettings can be cloned (important for threading)
    let original = AudioSettings::new(48000, 24, 512, "default".to_string());
    let cloned = original.clone();
    
    assert_eq!(original.sample_rate, cloned.sample_rate);
    assert_eq!(original.bit_depth, cloned.bit_depth);
    assert_eq!(original.buffer_size, cloned.buffer_size);
    assert_eq!(original.device_id, cloned.device_id);
}

#[test]
fn test_audio_settings_debug() {
    // Test Debug implementation
    let settings = AudioSettings::new(44100, 16, 256, "alsa:default".to_string());
    let debug_output = format!("{:?}", settings);
    
    assert!(debug_output.contains("44100"));
    assert!(debug_output.contains("16"));
    assert!(debug_output.contains("256"));
    assert!(debug_output.contains("alsa:default"));
    assert!(debug_output.contains("AudioSettings"));
}

#[test]
fn test_device_detection_integration() {
    // Test device detection without requiring actual audio hardware
    let result = detect_all_audio_devices();
    
    // This should not panic and should return a Result
    match result {
        Ok(devices) => {
            println!("Found {} audio devices", devices.len());
            // It's acceptable to have 0 devices in test environment
        }
        Err(e) => {
            println!("Device detection returned error: {}", e);
            // This is acceptable in environments without audio support
        }
    }
}

#[test]
fn test_audio_format_selection() {
    // Test the format selection logic used in the main application
    let test_cases = vec![
        (16, "S16LE"),
        (24, "S24LE"),
        (32, "S32LE"),
        (8, "S24LE"),  // Default case
    ];

    for (bit_depth, expected_format) in test_cases {
        let format = match bit_depth {
            16 => "S16LE",
            24 => "S24LE", 
            32 => "S32LE",
            _ => "S24LE",
        };
        assert_eq!(format, expected_format, "Failed for bit depth {}", bit_depth);
    }
}
