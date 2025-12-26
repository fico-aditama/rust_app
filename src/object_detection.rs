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
    model_path: String,
    #[cfg(feature = "webcam")]
    session: Option<ort::Session>,
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
        
        #[cfg(feature = "webcam")]
        let session = if Path::new(model_path).exists() {
            match ort::Session::builder()
                .unwrap()
                .with_model_from_file(model_path)
            {
                Ok(s) => {
                    println!("✅ Model loaded successfully!");
                    Some(s)
                }
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to load ONNX model: {}", e);
                    eprintln!("   Using demo detection mode. Download yolov8n.onnx for real detection.");
                    None
                }
            }
        } else {
            eprintln!("⚠️  Warning: Model file not found: {}", model_path);
            eprintln!("   Using demo detection mode. Download yolov8n.onnx for real detection.");
            None
        };
        
        #[cfg(not(feature = "webcam"))]
        let session = None;
        
        Self {
            model_path: model_path.to_string(),
            #[cfg(feature = "webcam")]
            session,
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
            
            // Run detection every frame for live detection (or every N frames for performance)
            let detections = self.detect_frame_opencv(&frame, conf_threshold)?;
            
            // Draw bounding boxes on frame
            if !detections.is_empty() {
                self.draw_detections(&mut frame, &detections)?;
            }
            
            // Print detections (every 30 frames to reduce spam)
            if frame_count % 30 == 0 && !detections.is_empty() {
                println!("\nFrame {} - Detections:", frame_count);
                for det in &detections {
                    println!("  {}", det);
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
    fn detect_frame_opencv(&self, frame: &core::Mat, conf_threshold: f32) -> Result<Vec<Detection>, String> {
        // Try real YOLO detection first if model is loaded
        if let Some(ref session) = self.session {
            return self.detect_yolo_real(frame, session, conf_threshold);
        }
        
        // Fallback to demo detection if no model
        self.detect_demo(frame)
    }
    
    #[cfg(feature = "webcam")]
    fn detect_yolo_real(&self, frame: &core::Mat, session: &ort::Session, conf_threshold: f32) -> Result<Vec<Detection>, String> {
        let size = frame.size().map_err(|e| format!("Failed to get size: {}", e))?;
        let orig_width = size.width as f32;
        let orig_height = size.height as f32;
        
        // 1. Resize to 640x640 (YOLO input)
        let mut resized = core::Mat::default();
        imgproc::resize(
            frame,
            &mut resized,
            core::Size::new(640, 640),
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        ).map_err(|e| format!("Failed to resize: {}", e))?;
        
        // 2. Convert BGR to RGB and normalize to [0, 1]
        let mut rgb = core::Mat::default();
        imgproc::cvt_color(&resized, &mut rgb, imgproc::COLOR_BGR2RGB, 0)
            .map_err(|e| format!("Failed to convert color: {}", e))?;
        
        // 3. Get pixel data
        let data = rgb.data_bytes()
            .map_err(|e| format!("Failed to get data: {}", e))?;
        
        // 4. Convert to tensor [1, 3, 640, 640] and normalize
        let mut input_data = Vec::with_capacity(1 * 3 * 640 * 640);
        for i in (0..data.len()).step_by(3) {
            input_data.push(data[i] as f32 / 255.0);
            input_data.push(data[i + 1] as f32 / 255.0);
            input_data.push(data[i + 2] as f32 / 255.0);
        }
        
        // 5. Create input tensor
        let input_shape = vec![1, 3, 640, 640];
        let input_tensor = ort::Value::from_array(
            (input_shape.clone(), input_data)
        ).map_err(|e| format!("Failed to create tensor: {}", e))?;
        
        // 6. Run inference
        let outputs = session.run(vec![input_tensor])
            .map_err(|e| format!("Inference failed: {}", e))?;
        
        // 7. Get output (YOLO output is [1, num_detections, 85] for COCO)
        let output = outputs[0].try_extract_tensor::<f32>()
            .map_err(|e| format!("Failed to extract output: {}", e))?;
        
        let output_shape = output.shape();
        if output_shape.len() < 2 {
            return Ok(vec![]);
        }
        
        // 8. Post-process: decode boxes and apply NMS
        let num_detections = output_shape[1];
        let mut detections = Vec::new();
        
        // COCO class names (80 classes)
        let class_names = vec![
            "person", "bicycle", "car", "motorcycle", "airplane", "bus", "train", "truck",
            "boat", "traffic light", "fire hydrant", "stop sign", "parking meter", "bench",
            "bird", "cat", "dog", "horse", "sheep", "cow", "elephant", "bear", "zebra",
            "giraffe", "backpack", "umbrella", "handbag", "tie", "suitcase", "frisbee",
            "skis", "snowboard", "sports ball", "kite", "baseball bat", "baseball glove",
            "skateboard", "surfboard", "tennis racket", "bottle", "wine glass", "cup",
            "fork", "knife", "spoon", "bowl", "banana", "apple", "sandwich", "orange",
            "broccoli", "carrot", "hot dog", "pizza", "donut", "cake", "chair", "couch",
            "potted plant", "bed", "dining table", "toilet", "tv", "laptop", "mouse",
            "remote", "keyboard", "cell phone", "microwave", "oven", "toaster", "sink",
            "refrigerator", "book", "clock", "vase", "scissors", "teddy bear", "hair drier",
            "toothbrush"
        ];
        
        // Decode YOLO output format: [x_center, y_center, width, height, conf, class_probs...]
        for i in 0..num_detections {
            let base_idx = i * 85;
            if base_idx + 4 >= output.len() {
                break;
            }
            
            let x_center = output[base_idx];
            let y_center = output[base_idx + 1];
            let width = output[base_idx + 2];
            let height = output[base_idx + 3];
            let obj_conf = output[base_idx + 4];
            
            // Find best class
            let mut max_class_idx = 0;
            let mut max_class_conf = 0.0f32;
            for class_idx in 0..80 {
                let class_conf = output[base_idx + 5 + class_idx];
                if class_conf > max_class_conf {
                    max_class_conf = class_conf;
                    max_class_idx = class_idx;
                }
            }
            
            // Calculate final confidence
            let confidence = obj_conf * max_class_conf;
            
            if confidence >= conf_threshold {
                // Convert from center format to corner format and scale to original size
                let x1 = ((x_center - width / 2.0) * orig_width / 640.0).max(0.0);
                let y1 = ((y_center - height / 2.0) * orig_height / 640.0).max(0.0);
                let x2 = ((x_center + width / 2.0) * orig_width / 640.0).min(orig_width);
                let y2 = ((y_center + height / 2.0) * orig_height / 640.0).min(orig_height);
                
                detections.push(Detection {
                    class: class_names.get(max_class_idx)
                        .unwrap_or(&"unknown")
                        .to_string(),
                    confidence,
                    bbox: [x1, y1, x2, y2],
                });
            }
        }
        
        // Simple NMS (Non-Maximum Suppression) - remove overlapping boxes
        detections = self.nms(detections, 0.45);
        
        Ok(detections)
    }
    
    #[cfg(feature = "webcam")]
    fn detect_demo(&self, frame: &core::Mat) -> Result<Vec<Detection>, String> {
        let size = frame.size().map_err(|e| format!("Failed to get frame size: {}", e))?;
        let width = size.width;
        let height = size.height;
        
        let mut detections = Vec::new();
        
        // Demo detection - simulate moving object
        if width > 100 && height > 100 {
            // Simulate detection that moves slightly (for demo)
            let center_x = width / 2;
            let center_y = height / 2;
            let box_size = (width.min(height) / 4) as f32;
            
            detections.push(Detection {
                class: "person".to_string(),
                confidence: 0.75,
                bbox: [
                    (center_x as f32 - box_size),
                    (center_y as f32 - box_size),
                    (center_x as f32 + box_size),
                    (center_y as f32 + box_size),
                ],
            });
        }
        
        Ok(detections)
    }
    
    #[cfg(feature = "webcam")]
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

