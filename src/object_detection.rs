// Object Detection dengan Rust
// Requirements: Add to Cargo.toml:
//   ort = "2.0"
//   image = "0.24"
//   opencv = "0.88"

use std::path::Path;
use std::fs;

#[cfg(feature = "webcam")]
use opencv::prelude::*;
#[cfg(feature = "webcam")]
use opencv::videoio;
#[cfg(feature = "webcam")]
use opencv::core;
#[cfg(feature = "webcam")]
use opencv::imgproc;

// Note: This is a simplified example structure
// For full YOLO implementation, you'd need to use ONNX Runtime (ort crate)
// or integrate with Python via PyO3, or use a Rust-native ML framework

pub struct ObjectDetector {
    #[allow(dead_code)]
    model_path: String,
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub class: String,
    pub confidence: f32,
    pub bbox: [f32; 4], // [x1, y1, x2, y2]
}

impl ObjectDetector {
    pub fn new(model_path: &str) -> Self {
        println!("Loading model: {}", model_path);
        // In real implementation, load ONNX model here
        // let session = ort::Session::new(model_path)?;
        Self {
            model_path: model_path.to_string(),
        }
    }

    pub fn detect_image(&self, image_path: &str) -> Result<Vec<Detection>, String> {
        if !Path::new(image_path).exists() {
            return Err(format!("Image not found: {}", image_path));
        }

        println!("Detecting objects in: {}", image_path);
        
        // Load image
        let img = image::open(image_path)
            .map_err(|e| format!("Failed to load image: {}", e))?;
        
        println!("Image loaded: {}x{}", img.width(), img.height());
        
        // Preprocess image
        let input_tensor = self.preprocess_image(&img)?;
        
        // Run inference (placeholder - would use ONNX Runtime in real implementation)
        let detections = self.run_inference(&input_tensor)?;
        
        Ok(detections)
    }

    fn preprocess_image(&self, img: &image::DynamicImage) -> Result<Vec<f32>, String> {
        // Resize to 640x640 (YOLO input size)
        let resized = img.resize_exact(640, 640, image::imageops::FilterType::Triangle);
        
        // Convert to RGB and normalize
        let rgb = resized.to_rgb8();
        let mut tensor = Vec::with_capacity(640 * 640 * 3);
        
        for pixel in rgb.pixels() {
            // Normalize to [0, 1]
            tensor.push(pixel[0] as f32 / 255.0);
            tensor.push(pixel[1] as f32 / 255.0);
            tensor.push(pixel[2] as f32 / 255.0);
        }
        
        Ok(tensor)
    }

    fn run_inference(&self, _input: &[f32]) -> Result<Vec<Detection>, String> {
        // Placeholder - in real implementation, this would:
        // 1. Load ONNX model using ort crate
        // 2. Run inference
        // 3. Post-process outputs (NMS, etc.)
        // 4. Return detections
        
        println!("Running inference (placeholder - implement with ONNX Runtime)");
        
        // Example output
        Ok(vec![
            Detection {
                class: "person".to_string(),
                confidence: 0.85,
                bbox: [100.0, 150.0, 300.0, 500.0],
            },
            Detection {
                class: "car".to_string(),
                confidence: 0.72,
                bbox: [400.0, 200.0, 600.0, 400.0],
            },
        ])
    }

    pub fn save_results(&self, detections: &[Detection], output_path: &str) -> Result<(), String> {
        // Save detections to file (JSON format)
        let json = serde_json::json!({
            "detections": detections.iter().map(|d| {
                serde_json::json!({
                    "class": d.class,
                    "confidence": d.confidence,
                    "bbox": d.bbox
                })
            }).collect::<Vec<_>>()
        });
        
        fs::write(output_path, serde_json::to_string_pretty(&json).unwrap())
            .map_err(|e| format!("Failed to save results: {}", e))?;
        
        println!("Results saved to: {}", output_path);
        Ok(())
    }

    #[cfg(feature = "webcam")]
    pub fn detect_webcam(&self, conf_threshold: f32) -> Result<(), String> {
        
        println!("Opening webcam...");
        
        // Open webcam (device 0)
        let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)
            .map_err(|e| format!("Failed to open webcam: {}", e))?;
        
        let opened = videoio::VideoCapture::is_opened(&cam)
            .map_err(|e| format!("Failed to check webcam: {}", e))?;
        
        if !opened {
            return Err("Cannot open webcam. Make sure webcam is connected.".to_string());
        }
        
        println!("Webcam opened successfully!");
        println!("Press 'q' to quit, 's' to save current frame");
        
        let mut frame_count = 0;
        let mut frame = core::Mat::default();
        
        loop {
            // Read frame
            match cam.read(&mut frame) {
                Ok(_) => {
                    if frame.size().unwrap().width == 0 {
                        break; // End of stream
                    }
                }
                Err(e) => {
                    eprintln!("Error reading frame: {}", e);
                    break;
                }
            }
            
            frame_count += 1;
            
            // Run detection every N frames (to improve performance)
            if frame_count % 5 == 0 {
                // Convert OpenCV Mat to image::DynamicImage for processing
                let detections = self.detect_frame_opencv(&frame, conf_threshold)?;
                
                // Draw bounding boxes on frame
                self.draw_detections(&mut frame, &detections)?;
                
                // Print detections (every 30 frames to reduce spam)
                if frame_count % 30 == 0 && !detections.is_empty() {
                    println!("\nFrame {} - Detections:", frame_count);
                    for det in &detections {
                        println!("  {}", det);
                    }
                }
            }
            
            // Display frame
            #[cfg(feature = "opencv-highgui")]
            {
                use opencv::highgui;
                highgui::imshow("Object Detection - Webcam", &frame)
                    .map_err(|e| format!("Failed to display frame: {}", e))?;
                
                // Check for 'q' key to quit
                let key = highgui::wait_key(1)
                    .map_err(|e| format!("Failed to wait for key: {}", e))?;
                
                if key == 113 || key == 27 { // 'q' or ESC
                    println!("Quitting...");
                    break;
                }
                
                // Save frame on 's' key
                if key == 115 { // 's'
                    let filename = format!("webcam_frame_{}.jpg", frame_count);
                    use opencv::imgcodecs;
                    imgcodecs::imwrite(&filename, &frame, &core::Vector::new())
                        .map_err(|e| format!("Failed to save frame: {}", e))?;
                    println!("Frame saved to: {}", filename);
                }
            }
            
            #[cfg(not(feature = "opencv-highgui"))]
            {
                // Without highgui, just process frames
                // In production, you might want to send frames to a display server
                if frame_count % 30 == 0 {
                    print!(".");
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                }
            }
        }
        
        println!("\nWebcam detection stopped. Processed {} frames.", frame_count);
        Ok(())
    }

    #[cfg(feature = "webcam")]
    fn detect_frame_opencv(&self, _frame: &core::Mat, _conf_threshold: f32) -> Result<Vec<Detection>, String> {
        // Placeholder - implement actual inference here
        // Convert OpenCV Mat to RGB image format
        // This is a placeholder - in real implementation, you'd:
        // 1. Convert Mat to tensor format
        // 2. Run ONNX inference
        // 3. Post-process results
        
        // For now, return empty or example detections
        // In production, implement actual inference here
        Ok(vec![])
    }

    #[cfg(feature = "webcam")]
    fn draw_detections(&self, frame: &mut core::Mat, detections: &[Detection]) -> Result<(), String> {
        use core::Scalar;
        
        for det in detections {
            let bbox = det.bbox;
            let x1 = bbox[0] as i32;
            let y1 = bbox[1] as i32;
            let x2 = bbox[2] as i32;
            let y2 = bbox[3] as i32;
            
            // Draw rectangle
            let rect = core::Rect::new(x1, y1, x2 - x1, y2 - y1);
            imgproc::rectangle(
                frame,
                rect,
                Scalar::new(0.0, 255.0, 0.0, 0.0), // Green
                2,
                imgproc::LINE_8,
                0,
            ).map_err(|e| format!("Failed to draw rectangle: {}", e))?;
            
            // Draw label
            let label = format!("{}: {:.1}%", det.class, det.confidence * 100.0);
            imgproc::put_text(
                frame,
                &label,
                core::Point::new(x1, y1 - 10),
                imgproc::FONT_HERSHEY_SIMPLEX,
                0.5,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                1,
                imgproc::LINE_8,
                false,
            ).map_err(|e| format!("Failed to draw text: {}", e))?;
        }
        
        Ok(())
    }
}

impl std::fmt::Display for Detection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {:.2}% at [{:.0}, {:.0}, {:.0}, {:.0}]",
            self.class,
            self.confidence * 100.0,
            self.bbox[0],
            self.bbox[1],
            self.bbox[2],
            self.bbox[3]
        )
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = ObjectDetector::new("yolov8n.onnx");
        assert_eq!(detector.model_path, "yolov8n.onnx");
    }
}

