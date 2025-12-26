// Object Detection CLI dengan Rust
// Run: cargo run --bin rust_object_detection -- image photo.jpg

mod object_detection;

use std::env;
use object_detection::{ObjectDetector, Detection};

fn print_detections(detections: &[Detection]) {
    if detections.is_empty() {
        println!("No objects detected.");
        return;
    }

    println!("\n{}", "=".repeat(60));
    println!("Detected {} object(s):", detections.len());
    println!("{}", "=".repeat(60));
    println!("{:<20} {:<15} {:<20}", "Class", "Confidence", "Bounding Box");
    println!("{}", "-".repeat(60));

    for det in detections {
        println!(
            "{:<20} {:<15.2} [{:.0}, {:.0}, {:.0}, {:.0}]",
            det.class,
            det.confidence * 100.0,
            det.bbox[0],
            det.bbox[1],
            det.bbox[2],
            det.bbox[3]
        );
    }

    println!("{}\n", "=".repeat(60));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run --bin rust_object_detection -- image <input_image> [output_json]");
        println!("  cargo run --bin rust_object_detection -- webcam");
        println!("\nExample:");
        println!("  cargo run --bin rust_object_detection -- image photo.jpg results.json");
        println!("  cargo run --bin rust_object_detection -- webcam");
        std::process::exit(1);
    }

    let mode = &args[1];

    // Initialize detector
    let detector = ObjectDetector::new("yolov8n.onnx");

    match mode.as_str() {
        "image" => {
            if args.len() < 3 {
                println!("Error: Please provide image path");
                println!("Usage: cargo run --bin rust_object_detection -- image <input_image> [output_json]");
                std::process::exit(1);
            }

            let input_path = &args[2];
            let output_path = if args.len() > 3 { Some(&args[3]) } else { None };

            // Detect objects
            match detector.detect_image(input_path) {
                Ok(detections) => {
                    print_detections(&detections);

                    // Save results if output path specified
                    if let Some(output) = output_path {
                        if let Err(e) = detector.save_results(&detections, output) {
                            eprintln!("Error saving results: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "webcam" => {
            #[cfg(feature = "webcam")]
            {
                println!("Starting webcam detection...");
                println!("Note: Requires opencv feature enabled in Cargo.toml");
                match detector.detect_webcam(0.25) {
                    Ok(_) => println!("Webcam detection completed."),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        eprintln!("\nMake sure:");
                        eprintln!("  1. Webcam is connected");
                        eprintln!("  2. opencv feature is enabled in Cargo.toml");
                        eprintln!("  3. Required system libraries are installed:");
                        eprintln!("     Ubuntu/Debian: sudo apt-get install libopencv-dev");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "webcam"))]
            {
                eprintln!("Error: Webcam feature not enabled!");
                eprintln!("\nTo enable webcam support:");
                eprintln!("1. Uncomment opencv dependency in Cargo.toml:");
                eprintln!("   opencv = {{ version = \"0.88\", features = [\"opencv-4\", \"videoio\", \"highgui\"] }}");
                eprintln!("\n2. Run with feature flag:");
                eprintln!("   cargo run --bin rust_object_detection --features webcam -- webcam");
                std::process::exit(1);
            }
        }
        _ => {
            println!("Error: Unknown mode '{}'", mode);
            println!("Available modes: image, webcam");
            std::process::exit(1);
        }
    }
}

