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
        
        // Capture screenshot
        let image = screen.capture()
            .map_err(|e| anyhow::anyhow!("Failed to capture screen: {}", e))?;
        
        // Convert to optimized format
        let optimized_bytes = self.optimize_image(&image)?;
        
        Ok(optimized_bytes)
    }
    
    pub fn optimize_image(&self, image: &screenshots::Image) -> Result<Vec<u8>> {
        // Convert to image::DynamicImage
        let width = image.width();
        let height = image.height();
        let rgba_data = image.buffer();
        
        let img = image::ImageBuffer::from_raw(width, height, rgba_data.to_vec())
            .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;
        
        let dynamic_img = image::DynamicImage::ImageRgba8(img);
        
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

