use anyhow::Result;
use screenshots::Screen;
use tracing::{debug, error, info};

pub struct ScreenshotCapture {
    screens: Vec<Screen>,
}

impl ScreenshotCapture {
    pub fn new() -> Result<Self> {
        info!("Initializing ScreenshotCapture...");
        let screens = Screen::all()
            .map_err(|e| anyhow::anyhow!("Failed to get screens: {}", e))?;
        
        if screens.is_empty() {
            error!("No screens found during initialization");
            return Err(anyhow::anyhow!("No screens found"));
        }
        
        info!("Found {} screen(s) available", screens.len());
        for (i, screen) in screens.iter().enumerate() {
            info!("Screen {}: {}x{}", i, screen.display_info.width, screen.display_info.height);
        }
        
        Ok(Self { screens })
    }
    
    pub async fn capture(&self) -> Result<Vec<u8>> {
        debug!("Screenshot capture requested with {} available screens", self.screens.len());
        info!("Starting screenshot capture...");
        
        // Use primary screen (first screen)
        let screen = &self.screens[0];
        debug!("Selected screen 0 from {} available screens", self.screens.len());
        info!("Using primary screen: {}x{}", screen.display_info.width, screen.display_info.height);
        
        // Capture screenshot
        debug!("About to call screen.capture() on screen {}x{}", screen.display_info.width, screen.display_info.height);
        info!("Capturing screen image...");
        let image = screen.capture()
            .map_err(|e| {
                error!("Screen capture failed: {}", e);
                anyhow::anyhow!("Failed to capture screen: {}", e)
            })?;
        
        info!("Screenshot captured successfully: {}x{} pixels", image.width(), image.height());
        
        // Convert to optimized format
        info!("Optimizing image...");
        let optimized_bytes = self.optimize_image(&image)?;
        
        info!("Image optimized to {} bytes", optimized_bytes.len());
        Ok(optimized_bytes)
    }
    
    pub fn optimize_image(&self, image: &screenshots::Image) -> Result<Vec<u8>> {
        debug!("Converting screenshot to optimized format...");
        debug!("Input image dimensions: {}x{}", image.width(), image.height());
        
        // The screenshots library might return PNG data directly
        // Let's try to use it as PNG data first
        let image_data = image.buffer();
        debug!("Raw image data size: {} bytes", image_data.len());
        
        debug!("Checking image format signatures...");
        // Check if this looks like PNG data (starts with PNG signature)
        if image_data.len() > 8 && &image_data[0..8] == b"\x89PNG\r\n\x1a\n" {
            info!("Detected PNG format from screenshots library");
            return Ok(image_data.to_vec());
        }
        
        // Check if this looks like JPEG data (starts with JPEG signature)
        if image_data.len() > 2 && &image_data[0..2] == b"\xFF\xD8" {
            info!("Detected JPEG format from screenshots library");
            return Ok(image_data.to_vec());
        }
        
        // If not a known format, treat as raw pixel data
        let width = image.width();
        let height = image.height();
        
        debug!("Raw pixel data detected - Image dimensions: {}x{}, data size: {} bytes", width, height, image_data.len());
        
        // Calculate bytes per pixel to determine format
        let total_pixels = (width * height) as usize;
        if total_pixels == 0 {
            return Err(anyhow::anyhow!("Invalid image dimensions: {}x{}", width, height));
        }
        
        let bytes_per_pixel = image_data.len() / total_pixels;
        debug!("Detected {} bytes per pixel (total pixels: {})", bytes_per_pixel, total_pixels);
        
        let dynamic_img = match bytes_per_pixel {
            4 => {
                // RGBA format
                debug!("Using RGBA format");
                let img = image::ImageBuffer::from_raw(width, height, image_data.to_vec())
                    .ok_or_else(|| {
                        error!("Failed to create RGBA image buffer from raw data");
                        anyhow::anyhow!("Failed to create RGBA image buffer")
                    })?;
                image::DynamicImage::ImageRgba8(img)
            },
            3 => {
                // RGB format
                debug!("Using RGB format");
                let img = image::ImageBuffer::from_raw(width, height, image_data.to_vec())
                    .ok_or_else(|| {
                        error!("Failed to create RGB image buffer from raw data");
                        anyhow::anyhow!("Failed to create RGB image buffer")
                    })?;
                image::DynamicImage::ImageRgb8(img)
            },
            _ => {
                error!("Unsupported image format: {} bytes per pixel (expected 3 or 4)", bytes_per_pixel);
                debug!("Raw data starts with: {:?}", &image_data[0..std::cmp::min(16, image_data.len())]);
                return Err(anyhow::anyhow!("Unsupported image format: {} bytes per pixel", bytes_per_pixel));
            }
        };
        
        // Choose optimal format based on content
        debug!("Choosing optimal image format based on content analysis...");
        let (buffer, mime_type) = self.choose_optimal_format(&dynamic_img)?;
        
        info!("Image converted to {} format, final size: {} bytes", mime_type, buffer.len());
        Ok(buffer)
    }
    
    pub fn choose_optimal_format(&self, image: &image::DynamicImage) -> Result<(Vec<u8>, &'static str)> {
        // For screenshots, PNG is usually better due to text and UI elements
        // But we can optimize based on content analysis
        
        let complexity = self.analyze_image_complexity(image);
        debug!("Image complexity analysis result: {:.3}", complexity);
        
        if complexity < 0.3 {
            debug!("Low complexity ({:.3} < 0.3), choosing PNG format", complexity);
            // Low complexity - use PNG for better text preservation
            let mut buffer = Vec::new();
            image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
            Ok((buffer, "image/png"))
        } else {
            debug!("High complexity ({:.3} >= 0.3), choosing JPEG format", complexity);
            // High complexity - use high-quality JPEG
            let mut buffer = Vec::new();
            image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(95))?;
            Ok((buffer, "image/jpeg"))
        }
    }
    
    pub fn analyze_image_complexity(&self, image: &image::DynamicImage) -> f32 {
        // Simple complexity analysis based on color variance
        let rgb_image = image.to_rgb8();
        let pixels = rgb_image.pixels();
        
        let mut total_variance = 0.0;
        let mut pixel_count = 0;
        
        for pixel in pixels {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            
            // Calculate variance from grayscale
            let gray = (r + g + b) / 3.0;
            let variance = ((r - gray).powi(2) + (g - gray).powi(2) + (b - gray).powi(2)) / 3.0;
            
            total_variance += variance;
            pixel_count += 1;
        }
        
        if pixel_count > 0 {
            (total_variance / pixel_count as f32) / 255.0
        } else {
            0.0
        }
    }
}

