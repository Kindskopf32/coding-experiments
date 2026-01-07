#!/usr/bin/env python3
"""
Ultra-fast batch resizer for same-resolution images with GPU acceleration
"""

import os
import argparse
from pathlib import Path
from PIL import Image
import torch
import torchvision.transforms as transforms
from torchvision.transforms.functional import resize
import time
import numpy as np

def batch_resize_same_resolution(input_dir, output_dir, scale_factor=0.5, 
                                 target_size=None, device='cuda'):
    """
    Batch resize images with SAME resolution using GPU
    """
    input_path = Path(input_dir)
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    
    # Check if CUDA is available
    if device == 'cuda' and not torch.cuda.is_available():
        print("CUDA not available, falling back to CPU")
        device = 'cpu'
    
    # Supported image extensions
    extensions = {'.jpg', '.jpeg', '.png', '.bmp', '.tiff', '.webp'}
    
    # Get all image files
    image_files = [f for f in input_path.iterdir() 
                  if f.suffix.lower() in extensions and f.is_file()]
    
    if not image_files:
        print(f"No images found in {input_dir}")
        return
    
    print(f"Found {len(image_files)} images")
    print(f"Using device: {device}")
    if device == 'cuda':
        print(f"GPU: {torch.cuda.get_device_name(0)}")
    
    # First, load one image to check resolution
    with Image.open(image_files[0]) as sample_img:
        original_size = (sample_img.width, sample_img.height)
        print(f"Original resolution: {original_size[0]}x{original_size[1]}")
        
        # Calculate target size
        if target_size:
            new_width, new_height = target_size
        else:
            new_height = int(sample_img.height * scale_factor)
            new_width = int(sample_img.width * scale_factor)
        
        print(f"Target resolution: {new_width}x{new_height}")
    
    # Pre-allocate tensor on GPU for maximum speed
    start_time = time.time()
    
    # Load all images into a single batch tensor
    print(f"\nLoading all {len(image_files)} images into batch...")
    batch_tensors = []
    batch_filenames = []
    
    load_start = time.time()
    for img_path in image_files:
        try:
            with Image.open(img_path) as img:
                # Ensure RGB format
                if img.mode != 'RGB':
                    img = img.convert('RGB')
                
                # Convert to tensor and add batch dimension
                tensor_img = transforms.ToTensor()(img).unsqueeze(0)
                batch_tensors.append(tensor_img)
                batch_filenames.append(img_path.name)
                
        except Exception as e:
            print(f"Warning: Could not load {img_path}: {e}")
            continue
    
    if not batch_tensors:
        print("No valid images found!")
        return
    
    # Stack all tensors into a single batch
    batch = torch.cat(batch_tensors, dim=0)
    batch_size = batch.size(0)
    
    load_time = time.time() - load_start
    print(f"Batch loaded: {batch_size} images")
    print(f"Batch tensor shape: {batch.shape}")
    print(f"Load time: {load_time:.2f}s")
    
    # Move entire batch to GPU
    print(f"\nMoving batch to {device}...")
    gpu_start = time.time()
    batch = batch.to(device)
    
    # Resize entire batch at once (MUCH faster!)
    print(f"Resizing entire batch on {device}...")
    resized_batch = resize(batch, [new_height, new_width], antialias=True)
    
    gpu_time = time.time() - gpu_start
    print(f"GPU batch processing time: {gpu_time:.2f}s")
    
    # Save all images
    print(f"\nSaving {batch_size} resized images...")
    save_start = time.time()
    
    for i in range(batch_size):
        try:
            # Convert tensor back to CPU and then to PIL
            img_tensor = resized_batch[i].cpu()
            img_pil = transforms.ToPILImage()(img_tensor)
            
            # Save image
            output_file = output_path / batch_filenames[i]
            img_pil.save(output_file)
            
            if (i + 1) % 10 == 0:
                print(f"  Saved {i + 1}/{batch_size} images")
                
        except Exception as e:
            print(f"Error saving {batch_filenames[i]}: {e}")
    
    save_time = time.time() - save_start
    total_time = time.time() - start_time
    
    print(f"\n{'='*50}")
    print(f"Summary:")
    print(f"  Total images processed: {batch_size}")
    print(f"  Load time: {load_time:.2f}s")
    print(f"  GPU processing time: {gpu_time:.2f}s")
    print(f"  Save time: {save_time:.2f}s")
    print(f"  Total time: {total_time:.2f}s")
    print(f"  Average time per image: {total_time/batch_size:.3f}s")
    print(f"  Speedup vs sequential: ~{load_time * batch_size / total_time:.1f}x")
    print(f"  Output saved to: {output_dir}")
    print(f"{'='*50}")
    
    # Clean up
    if device == 'cuda':
        torch.cuda.empty_cache()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Ultra-fast batch resizer for same-resolution images")
    parser.add_argument("input_dir", help="Input directory containing images")
    parser.add_argument("output_dir", help="Output directory for resized images")
    parser.add_argument("--scale", type=float, default=0.5, 
                       help="Scale factor (default: 0.5)")
    parser.add_argument("--width", type=int, help="Target width (overrides scale)")
    parser.add_argument("--height", type=int, help="Target height (overrides scale)")
    parser.add_argument("--cpu", action="store_true", help="Use CPU instead of GPU")
    
    args = parser.parse_args()
    
    # Set device
    device = 'cpu' if args.cpu else 'cuda'
    
    # Prepare target size if provided
    target_size = None
    if args.width and args.height:
        target_size = (args.width, args.height)
    
    # Run batch resize
    batch_resize_same_resolution(
        args.input_dir,
        args.output_dir,
        scale_factor=args.scale,
        target_size=target_size,
        device=device
    )
