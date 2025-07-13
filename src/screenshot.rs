use anyhow::Result;
use screenshots::Screen;

pub struct ScreenshotCapture {
    screens: Vec<Screen>,
}

impl ScreenshotCapture {
    pub fn new() -> Result<Self> {
        let screens = Screen::all()
            .map_err(|e| anyhow::anyhow!("Failed to get screens: {}", e))?;
        
        if screens.is_empty() {
            return Err(anyhow::anyhow!("No screens found"));
        }
        
        Ok(Self { screens })
    }
    
    pub async fn capture(&self) -> Result<Vec<u8>> {
        // Use primary screen (first screen)
        let screen = &self.screens[0];
        
        tracing::info!("Capturing screenshot from screen with display info: {:?}", screen.display_info);
        
        // Capture screenshot
        let image = screen.capture()
            .map_err(|e| anyhow::anyhow!("Failed to capture screen: {}", e))?;
        
        tracing::info!("Screenshot captured successfully");
        
        // Try to save raw data first to see if the screenshot library is working
        let width = image.width();
        let height = image.height();
        let buffer = image.buffer();
        
        tracing::info!("Raw image data - Width: {}, Height: {}, Buffer length: {}", width, height, buffer.len());
        
        // Convert to optimized format
        let optimized_bytes = self.optimize_image(&image)?;
        
        Ok(optimized_bytes)
    }
    
    pub fn optimize_image(&self, image: &screenshots::Image) -> Result<Vec<u8>> {
        // Alternative approach: try to save the screenshot as PNG directly
        // since the buffer size doesn't match RGBA expectations
        
        let width = image.width();
        let height = image.height();
        let rgba_data = image.buffer();
        
        tracing::info!("Screenshot dimensions: {}x{}", width, height);
        tracing::info!("Buffer length: {}", rgba_data.len());
        
        // Check if this might be compressed data already
        let bytes_per_pixel = rgba_data.len() as f32 / (width as f32 * height as f32);
        tracing::info!("Bytes per pixel: {:.2}", bytes_per_pixel);
        
        if bytes_per_pixel < 1.0 {
            // This looks like compressed data, maybe PNG or JPEG
            // Try to decode it first
            tracing::info!("Buffer appears to be compressed, attempting to decode...");
            
            // Try to decode as PNG first
            if let Ok(decoded) = image::load_from_memory_with_format(rgba_data, image::ImageFormat::Png) {
                tracing::info!("Successfully decoded as PNG");
                let mut buffer = Vec::new();
                decoded.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;
                return Ok(buffer);
            }
            
            // Try to decode as JPEG
            if let Ok(decoded) = image::load_from_memory_with_format(rgba_data, image::ImageFormat::Jpeg) {
                tracing::info!("Successfully decoded as JPEG");
                let mut buffer = Vec::new();
                decoded.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;
                return Ok(buffer);
            }
            
            // If neither work, return the raw data as-is
            tracing::warn!("Could not decode compressed data, returning raw buffer");
            return Ok(rgba_data.to_vec());
        }
        
        // Check for common pixel formats
        let expected_rgba = (width as usize) * (height as usize) * 4;
        let expected_rgb = (width as usize) * (height as usize) * 3;
        let _expected_bgra = expected_rgba;
        
        let dynamic_img = if rgba_data.len() == expected_rgba {
            // RGBA format
            tracing::info!("Using RGBA format");
            let img = image::ImageBuffer::from_raw(width, height, rgba_data.to_vec())
                .ok_or_else(|| anyhow::anyhow!("Failed to create RGBA image buffer"))?;
            image::DynamicImage::ImageRgba8(img)
        } else if rgba_data.len() == expected_rgb {
            // RGB format - convert to RGBA
            tracing::info!("Converting from RGB to RGBA format");
            let mut rgba_vec = Vec::with_capacity(expected_rgba);
            for chunk in rgba_data.chunks(3) {
                rgba_vec.push(chunk[0]); // R
                rgba_vec.push(chunk[1]); // G  
                rgba_vec.push(chunk[2]); // B
                rgba_vec.push(255);      // A
            }
            let img = image::ImageBuffer::from_raw(width, height, rgba_vec)
                .ok_or_else(|| anyhow::anyhow!("Failed to create RGBA image buffer from RGB"))?;
            image::DynamicImage::ImageRgba8(img)
        } else {
            return Err(anyhow::anyhow!(
                "Unsupported pixel format: {} bytes for {}x{} (expected {} for RGBA or {} for RGB)",
                rgba_data.len(), width, height, expected_rgba, expected_rgb
            ));
        };
        
        // Choose optimal format based on content
        let (buffer, _mime_type) = self.choose_optimal_format(&dynamic_img)?;
        
        Ok(buffer)
    }
    
    pub fn choose_optimal_format(&self, image: &image::DynamicImage) -> Result<(Vec<u8>, &'static str)> {
        // For screenshots, PNG is usually better due to text and UI elements
        // But we can optimize based on content analysis
        
        let complexity = self.analyze_image_complexity(image);
        
        if complexity < 0.3 {
            // Low complexity - use PNG for better text preservation
            let mut buffer = Vec::new();
            image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
            Ok((buffer, "image/png"))
        } else {
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

