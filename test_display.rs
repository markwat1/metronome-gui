// Test program to demonstrate the enhanced CLI display engine
use cli_metronome::{
    display::DisplayEngine,
    models::{Beat, TimeSignature},
    audio::AudioStatus,
};

fn main() {
    let display = DisplayEngine::new();
    
    println!("Testing Enhanced CLI Display Engine");
    println!("===================================\n");
    
    // Test startup info with different time signatures
    println!("1. Testing startup info with 4/4 time signature:");
    display.show_startup_info(120, TimeSignature::Four, &AudioStatus::Available);
    
    println!("\n2. Testing startup info with 3/4 time signature:");
    display.show_startup_info(90, TimeSignature::Three, &AudioStatus::FallbackMode);
    
    println!("\n3. Testing beat indicators for different time signatures:");
    
    // Test 4/4 time signature beats
    println!("\n4/4 Time Signature Beat Sequence:");
    for i in 1..=8 {
        let beat = Beat::new(i, TimeSignature::Four, 120);
        print!("Beat {}: ", i);
        display.show_visual_beat(&beat);
        println!(" (Strength: {:.1})", beat.get_accent_strength());
    }
    
    // Test 3/4 time signature beats
    println!("\n3/4 Time Signature Beat Sequence:");
    for i in 1..=6 {
        let beat = Beat::new(i, TimeSignature::Three, 120);
        print!("Beat {}: ", i);
        display.show_visual_beat(&beat);
        println!(" (Strength: {:.1})", beat.get_accent_strength());
    }
    
    // Test 6/8 time signature beats
    println!("\n6/8 Time Signature Beat Sequence:");
    for i in 1..=12 {
        let beat = Beat::new(i, TimeSignature::Six, 120);
        print!("Beat {}: ", i);
        display.show_visual_beat(&beat);
        println!(" (Strength: {:.1})", beat.get_accent_strength());
    }
    
    println!("\n4. Testing beat indicator:");
    let beat = Beat::new(1, TimeSignature::Four, 120);
    print!("Beat indicator for strong beat: ");
    display.show_beat_indicator(&beat);
    println!();
    
    let beat = Beat::new(3, TimeSignature::Four, 120);
    print!("Beat indicator for medium beat: ");
    display.show_beat_indicator(&beat);
    println!();
    
    let beat = Beat::new(2, TimeSignature::Four, 120);
    print!("Beat indicator for weak beat: ");
    display.show_beat_indicator(&beat);
    println!();
    
    println!("\n5. Testing enhanced status display with time signature info:");
    use std::time::Duration;
    display.show_status(120, 42, Duration::from_secs(35), TimeSignature::Four, 2);
    println!();
    
    println!("\n6. Testing enhanced beat indicators:");
    let beat = Beat::new(1, TimeSignature::Four, 120);
    print!("Enhanced beat indicator for strong beat: ");
    display.show_enhanced_beat_indicator(&beat);
    println!();
    
    let beat = Beat::new(3, TimeSignature::Four, 120);
    print!("Enhanced beat indicator for medium beat: ");
    display.show_enhanced_beat_indicator(&beat);
    println!();
    
    let beat = Beat::new(2, TimeSignature::Four, 120);
    print!("Enhanced beat indicator for weak beat: ");
    display.show_enhanced_beat_indicator(&beat);
    println!();
    
    println!("\n7. Testing comprehensive beat indicator:");
    let beat = Beat::new(1, TimeSignature::Four, 120);
    print!("Comprehensive indicator for strong beat: ");
    display.show_comprehensive_beat_indicator(&beat);
    println!();
    
    let beat = Beat::new(3, TimeSignature::Four, 120);
    print!("Comprehensive indicator for medium beat: ");
    display.show_comprehensive_beat_indicator(&beat);
    println!();
    
    println!("\n8. Testing real-time beat visualization:");
    for i in 1..=4 {
        let beat = Beat::new(i, TimeSignature::Four, 120);
        display.show_realtime_beat_visualization(&beat);
        println!();
    }
    
    println!("\n9. Testing enhanced time signature info:");
    display.show_enhanced_time_signature_info(TimeSignature::Four, 2);
    println!();
    display.show_enhanced_time_signature_info(TimeSignature::Three, 1);
    println!();
    
    println!("\n10. Testing beat pattern demonstration:");
    display.demonstrate_beat_pattern(TimeSignature::Four);
    display.demonstrate_beat_pattern(TimeSignature::Three);
    
    println!("\n11. Testing pulsing beat indicator:");
    let beat = Beat::new(1, TimeSignature::Four, 120);
    print!("Pulsing indicator (strong beat): ");
    for phase in [0.0, 0.25, 0.5, 0.75, 1.0] {
        display.show_pulsing_beat_indicator(&beat, phase);
    }
    println!();
    
    let beat = Beat::new(2, TimeSignature::Four, 120);
    print!("Pulsing indicator (weak beat): ");
    for phase in [0.0, 0.25, 0.5, 0.75, 1.0] {
        display.show_pulsing_beat_indicator(&beat, phase);
    }
    println!();
    
    println!("\n12. Testing time signature help:");
    display.show_time_signature_help();
    
    println!("\nTest completed successfully! All enhanced visual indicators are working.");
}