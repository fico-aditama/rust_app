// Object Detection dengan Rust
// Requirements: Add to Cargo.toml:
//   ort = "2.0.0-rc.10"
//   image = "0.24"
//   opencv = "0.88"

use std::path::Path;
use std::fs;

// #[cfg(feature = "webcam")]
// use ort::Session;  // Uncomment when ONNX is ready

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
    // #[cfg(feature = "webcam")]
    // session: Option<ort::Session>,  // Uncomment when ONNX is ready
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
            
            // Run detection every frame for live detection
            let detections = self.detect_frame_opencv(&frame, conf_threshold)?;
            
            // Draw bounding boxes on frame
            if !detections.is_empty() {
                self.draw_detections(&mut frame, &detections)?;
            }
            
            // Print status every 30 frames to show it's working
            if frame_count % 30 == 0 {
                if !detections.is_empty() {
                    println!("\n📹 Frame {} - Detections:", frame_count);
                    for det in &detections {
                        println!("  ✅ {}", det);
                    }
                } else {
                    print!(".");
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                }
            }
            
            // Display frame - Always try to show window if highgui available
            {
                #[cfg(feature = "opencv-highgui")]
                {
                    use opencv::highgui;
                    match highgui::imshow("Object Detection - Webcam", &frame) {
                        Ok(_) => {
                            // Check for 'q' key to quit
                            match highgui::wait_key(1) {
                                Ok(key) => {
                                    if key == 113 || key == 27 { // 'q' or ESC
                                        println!("\nQuitting...");
                                        break;
                                    }
                                    
                                    // Save frame on 's' key
                                    if key == 115 { // 's'
                                        let filename = format!("webcam_frame_{}.jpg", frame_count);
                                        use opencv::imgcodecs;
                                        if let Err(e) = imgcodecs::imwrite(&filename, &frame, &core::Vector::new()) {
                                            eprintln!("Failed to save frame: {}", e);
                                        } else {
                                            println!("\nFrame saved to: {}", filename);
                                        }
                                    }
                                }
                                Err(_) => {
                                    // Continue if wait_key fails
                                }
                            }
                        }
                        Err(e) => {
                            // Window display failed, but continue processing
                            if frame_count % 30 == 0 {
                                eprintln!("\nWarning: Cannot display window: {}. Processing frames...", e);
                            }
                        }
                    }
                }
                
                #[cfg(not(feature = "opencv-highgui"))]
                {
                    // Without highgui, just process frames
                    if frame_count % 30 == 0 {
                        print!(".");
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
        }
        
        println!("\nWebcam detection stopped. Processed {} frames.", frame_count);
        Ok(())
    }

    #[cfg(feature = "webcam")]
    fn detect_frame_opencv(&self, frame: &core::Mat, _conf_threshold: f32) -> Result<Vec<Detection>, String> {
        // Dynamic demo detection - objects move and follow patterns
        // This simulates live detection like Python version
        // TODO: Implement real YOLO with ONNX Runtime for actual object detection
        self.detect_demo(frame)
    }
    #[cfg(feature = "webcam")]
    fn detect_demo(&self, frame: &core::Mat) -> Result<Vec<Detection>, String> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let size = frame.size().map_err(|e| format!("Failed to get frame size: {}", e))?;
        let width = size.width;
        let height = size.height;
        
        let mut detections = Vec::new();
        
        // Demo detection - simulate moving object that follows movement
        if width > 100 && height > 100 {
            // Use time to simulate movement (makes it "live" and dynamic)
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f32();
            
            // Simulate object moving in a circle pattern
            let center_x = (width as f32 / 2.0) + (width as f32 / 4.0) * (time * 0.5).sin();
            let center_y = (height as f32 / 2.0) + (height as f32 / 4.0) * (time * 0.5).cos();
            let box_size = (width.min(height) / 5) as f32;
            
            // Simulate multiple objects
            detections.push(Detection {
                class: "person".to_string(),
                confidence: 0.85,
                bbox: [
                    (center_x - box_size).max(0.0),
                    (center_y - box_size).max(0.0),
                    (center_x + box_size).min(width as f32),
                    (center_y + box_size).min(height as f32),
                ],
            });
            
            // Add another moving object
            let center_x2 = (width as f32 / 2.0) + (width as f32 / 3.0) * (time * 0.3).cos();
            let center_y2 = (height as f32 / 2.0) + (height as f32 / 3.0) * (time * 0.3).sin();
            
            detections.push(Detection {
                class: "car".to_string(),
                confidence: 0.72,
                bbox: [
                    (center_x2 - box_size * 0.8).max(0.0),
                    (center_y2 - box_size * 0.8).max(0.0),
                    (center_x2 + box_size * 0.8).min(width as f32),
                    (center_y2 + box_size * 0.8).min(height as f32),
                ],
            });
        }
        
        Ok(detections)
    }
    
    #[cfg(feature = "webcam")]
    #[allow(dead_code)]
    fn nms(&self, mut detections: Vec<Detection>, iou_threshold: f32) -> Vec<Detection> {
        // Simple NMS implementation
        detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        let mut result = Vec::new();
        let mut used = vec![false; detections.len()];
        
        for i in 0..detections.len() {
            if used[i] {
                continue;
            }
            
            result.push(detections[i].clone());
            
            for j in (i + 1)..detections.len() {
                if used[j] {
                    continue;
                }
                
                let iou = self.calculate_iou(&detections[i].bbox, &detections[j].bbox);
                if iou > iou_threshold {
                    used[j] = true;
                }
            }
        }
        
        result
    }
    
    #[cfg(feature = "webcam")]
    #[allow(dead_code)]
    fn calculate_iou(&self, box1: &[f32; 4], box2: &[f32; 4]) -> f32 {
        let x1 = box1[0].max(box2[0]);
        let y1 = box1[1].max(box2[1]);
        let x2 = box1[2].min(box2[2]);
        let y2 = box1[3].min(box2[3]);
        
        if x2 <= x1 || y2 <= y1 {
            return 0.0;
        }
        
        let intersection = (x2 - x1) * (y2 - y1);
        let area1 = (box1[2] - box1[0]) * (box1[3] - box1[1]);
        let area2 = (box2[2] - box2[0]) * (box2[3] - box2[1]);
        let union = area1 + area2 - intersection;
        
        if union <= 0.0 {
            return 0.0;
        }
        
        intersection / union
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

