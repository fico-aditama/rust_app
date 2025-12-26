#!/usr/bin/env python3
"""
Object Detection dengan YOLOv8 - Single File
Requirements: pip install ultralytics opencv-python pillow
Auto-install dependencies if missing
"""

import sys
import subprocess
import importlib.util

def check_and_install_package(package_name, import_name=None):
    """Check if package is installed, install if missing"""
    if import_name is None:
        import_name = package_name
    
    spec = importlib.util.find_spec(import_name)
    if spec is None:
        print(f"⚠️  Package '{package_name}' not found. Installing...", end=" ", flush=True)
        try:
            subprocess.check_call(
                [sys.executable, "-m", "pip", "install", package_name, "--quiet", "--disable-pip-version-check"],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL
            )
            print("✅")
        except subprocess.CalledProcessError:
            print("❌")
            print(f"\n❌ Failed to install '{package_name}'. Please install manually:")
            print(f"   pip install {package_name}")
            sys.exit(1)
    # Don't print if already installed (to reduce noise)

# Auto-install dependencies (only show if installing)
missing_packages = []
for pkg, imp in [("opencv-python", "cv2"), ("ultralytics", "ultralytics"), ("pillow", "PIL")]:
    spec = importlib.util.find_spec(imp)
    if spec is None:
        missing_packages.append((pkg, imp))

if missing_packages:
    print("Checking dependencies...")
    for pkg, imp in missing_packages:
        check_and_install_package(pkg, imp)
    print("All dependencies ready!\n")

# Now import after ensuring dependencies are installed
import cv2
import numpy as np
from ultralytics import YOLO
from PIL import Image
import os

class ObjectDetector:
    def __init__(self, model_path='yolov8n.pt'):
        """
        Initialize YOLOv8 model
        model_path: Path ke model YOLOv8 (yolov8n.pt, yolov8s.pt, yolov8m.pt, dll)
        """
        print(f"Loading model: {model_path}")
        self.model = YOLO(model_path)
        print("Model loaded successfully!")
    
    def detect_image(self, image_path, output_path=None, conf_threshold=0.25):
        """
        Detect objects in an image
        
        Args:
            image_path: Path to input image
            output_path: Path to save output image (optional)
            conf_threshold: Confidence threshold (0-1)
        
        Returns:
            List of detected objects with bounding boxes
        """
        if not os.path.exists(image_path):
            print(f"Error: Image not found: {image_path}")
            return []
        
        # Run inference
        results = self.model(image_path, conf=conf_threshold)
        
        # Process results
        detections = []
        annotated_image = None
        
        for result in results:
            # Get image with annotations
            annotated_image = result.plot()
            
            # Extract detection info
            boxes = result.boxes
            for box in boxes:
                # Get box coordinates
                x1, y1, x2, y2 = box.xyxy[0].cpu().numpy()
                
                # Get class and confidence
                cls = int(box.cls[0].cpu().numpy())
                conf = float(box.conf[0].cpu().numpy())
                class_name = self.model.names[cls]
                
                detections.append({
                    'class': class_name,
                    'confidence': conf,
                    'bbox': [int(x1), int(y1), int(x2), int(y2)]
                })
        
        # Save output image if specified
        if output_path and annotated_image is not None:
            cv2.imwrite(output_path, annotated_image)
            print(f"Output saved to: {output_path}")
        
        return detections
    
    def detect_video(self, video_path, output_path=None, conf_threshold=0.25):
        """
        Detect objects in a video
        
        Args:
            video_path: Path to input video
            output_path: Path to save output video (optional)
            conf_threshold: Confidence threshold (0-1)
        """
        if not os.path.exists(video_path):
            print(f"Error: Video not found: {video_path}")
            return
        
        # Open video
        cap = cv2.VideoCapture(video_path)
        if not cap.isOpened():
            print(f"Error: Cannot open video: {video_path}")
            return
        
        # Get video properties
        fps = int(cap.get(cv2.CAP_PROP_FPS))
        width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
        height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
        total_frames = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
        
        # Setup video writer if output specified
        writer = None
        if output_path:
            fourcc = cv2.VideoWriter_fourcc(*'mp4v')
            writer = cv2.VideoWriter(output_path, fourcc, fps, (width, height))
        
        frame_count = 0
        print(f"Processing video: {total_frames} frames")
        
        while True:
            ret, frame = cap.read()
            if not ret:
                break
            
            frame_count += 1
            if frame_count % 30 == 0:
                print(f"Processing frame {frame_count}/{total_frames}")
            
            # Run inference
            results = self.model(frame, conf=conf_threshold, verbose=False)
            
            # Draw results
            annotated_frame = results[0].plot()
            
            # Write frame
            if writer:
                writer.write(annotated_frame)
            
            # Display (optional)
            # cv2.imshow('Object Detection', annotated_frame)
            # if cv2.waitKey(1) & 0xFF == ord('q'):
            #     break
        
        cap.release()
        if writer:
            writer.release()
        cv2.destroyAllWindows()
        
        print(f"Video processing complete!")
        if output_path:
            print(f"Output saved to: {output_path}")
    
    def detect_webcam(self, conf_threshold=0.25):
        """
        Real-time object detection from webcam
        
        Args:
            conf_threshold: Confidence threshold (0-1)
        """
        cap = cv2.VideoCapture(0)
        if not cap.isOpened():
            print("Error: Cannot open webcam")
            return
        
        print("Starting webcam detection. Press 'q' to quit.")
        
        while True:
            ret, frame = cap.read()
            if not ret:
                break
            
            # Run inference
            results = self.model(frame, conf=conf_threshold, verbose=False)
            
            # Draw results
            annotated_frame = results[0].plot()
            
            # Display
            cv2.imshow('Object Detection - Webcam', annotated_frame)
            
            if cv2.waitKey(1) & 0xFF == ord('q'):
                break
        
        cap.release()
        cv2.destroyAllWindows()
        print("Webcam detection stopped.")


def print_detections(detections):
    """Print detection results in a formatted way"""
    if not detections:
        print("No objects detected.")
        return
    
    print(f"\n{'='*60}")
    print(f"Detected {len(detections)} object(s):")
    print(f"{'='*60}")
    print(f"{'Class':<20} {'Confidence':<15} {'Bounding Box':<20}")
    print(f"{'-'*60}")
    
    for det in detections:
        bbox = det['bbox']
        print(f"{det['class']:<20} {det['confidence']:<15.2f} [{bbox[0]}, {bbox[1]}, {bbox[2]}, {bbox[3]}]")
    
    print(f"{'='*60}\n")


def main():
    """Main function with CLI interface"""
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python object_detection.py image <input_image> [output_image]")
        print("  python object_detection.py video <input_video> [output_video]")
        print("  python object_detection.py webcam")
        print("\nExample:")
        print("  python object_detection.py image photo.jpg output.jpg")
        print("  python object_detection.py video video.mp4 output.mp4")
        print("  python object_detection.py webcam")
        sys.exit(1)
    
    mode = sys.argv[1].lower()
    
    # Initialize detector
    detector = ObjectDetector()
    
    if mode == 'image':
        if len(sys.argv) < 3:
            print("Error: Please provide image path")
            sys.exit(1)
        
        image_path = sys.argv[2]
        output_path = sys.argv[3] if len(sys.argv) > 3 else None
        
        print(f"\nDetecting objects in: {image_path}")
        detections = detector.detect_image(image_path, output_path)
        print_detections(detections)
    
    elif mode == 'video':
        if len(sys.argv) < 3:
            print("Error: Please provide video path")
            sys.exit(1)
        
        video_path = sys.argv[2]
        output_path = sys.argv[3] if len(sys.argv) > 3 else None
        
        print(f"\nDetecting objects in: {video_path}")
        detector.detect_video(video_path, output_path)
    
    elif mode == 'webcam':
        print("\nStarting webcam detection...")
        detector.detect_webcam()
    
    else:
        print(f"Error: Unknown mode '{mode}'")
        print("Available modes: image, video, webcam")
        sys.exit(1)


if __name__ == "__main__":
    main()

