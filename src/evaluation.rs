//! Evaluation and Benchmarking utilities.

use crate::error::Result;
use gemini_client_api::gemini::{ask::Gemini, types::sessions::Session, utils::MarkdownToParts};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

/// An automated judge that uses Vision AI to evaluate quality.
pub struct Judge {
    ai: Gemini,
}

impl Judge {
    /// Creates a new Judge with the provided API key and model.
    pub fn new(api_key: String, model: &str) -> Self {
        let ai = Gemini::new(
            api_key,
            model,
            Some(r"You are a meticulous Vision QA Engineer specialized in video compression and GIF optimization. Your goal is to perform a side-by-side comparison of ORIGINAL video frames versus OPTIMIZED GIF frames. Focus specifically on:
- Color Fidelity: Check for banding in gradients and accuracy of skin tones or vibrant colors.
- Texture & Detail: Look for dithering artifacts, graininess, or loss of fine patterns.
- Motion & Temporal: While analyzing static frames, look for inconsistencies between frames that might suggest jitter or unnatural movement.".to_string().into()),
        );
        Self { ai }
    }

    /// Extracts a few key frames from a video or GIF for evaluation.
    pub fn extract_evaluation_frames(input: &Path, prefix: &str) -> Vec<PathBuf> {
        let mut frames = Vec::new();
        let temp_dir = std::env::temp_dir();

        // Extract 3 frames: start, middle, end
        let f1 = temp_dir.join(format!("{}_1.png", prefix));
        Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                input.to_str().unwrap(),
                "-frames:v",
                "1",
                "-update",
                "1",
                f1.to_str().unwrap(),
            ])
            .output()
            .ok();
        frames.push(f1);

        let f2 = temp_dir.join(format!("{}_2.png", prefix));
        Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                input.to_str().unwrap(),
                "-vf",
                "select='not(mod(n,60))'",
                "-frames:v",
                "1",
                "-update",
                "1",
                f2.to_str().unwrap(),
            ])
            .output()
            .ok();
        frames.push(f2);

        let f3 = temp_dir.join(format!("{}_3.png", prefix));
        Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                input.to_str().unwrap(),
                "-sseof",
                "-1",
                "-frames:v",
                "1",
                "-update",
                "1",
                f3.to_str().unwrap(),
            ])
            .output()
            .ok();
        frames.push(f3);

        frames
    }

    /// Performs a side-by-side evaluation of an original video and an optimized GIF.
    pub async fn evaluate(&self, original: &Path, optimized: &Path) -> Result<Value> {
        let orig_frames = Self::extract_evaluation_frames(original, "orig");
        let opt_frames = Self::extract_evaluation_frames(optimized, "opt");

        let mut session = Session::new(1);
        let prompt = format!(
            r"Compare these pairs of frames from an ORIGINAL video and an OPTIMIZED GIF.

START FRAME: Original ![o1]({}), Optimized ![p1]({})
MIDDLE FRAME: Original ![o2]({}), Optimized ![p2]({})
END FRAME: Original ![o3]({}), Optimized ![p3]({})

Provide a 'Synthetic MOS' (Mean Opinion Score) from 1 to 10 for the overall quality of the optimized version. Analyze: 1. Color banding, 2. Texture loss, 3. Temporal consistency. Return only JSON: {{ 'score': number, 'reasoning': 'string', 'artifacts': ['string'] }}",
            orig_frames[0].to_str().unwrap(), opt_frames[0].to_str().unwrap(),
            orig_frames[1].to_str().unwrap(), opt_frames[1].to_str().unwrap(),
            orig_frames[2].to_str().unwrap(), opt_frames[2].to_str().unwrap()
        ).replace("'", "\""); // Use double quotes for valid JSON template

        let parts = MarkdownToParts::new(&prompt, |_| mime::IMAGE_PNG)
            .await
            .process();
        let response = self
            .ai
            .ask(session.ask(parts))
            .await
            .map_err(|e| crate::error::Error::Internal(e.to_string()))?;

        let text = response.get_chat().get_text_no_think("");

        if let Some(json_start) = text.find('{') {
            if let Some(json_end) = text.rfind('}') {
                let json_str = &text[json_start..=json_end];
                let mut val: Value = serde_json::from_str(json_str)
                    .map_err(|e| crate::error::Error::Internal(e.to_string()))?;

                // 2. Calculate Objective Metrics (SSIM / PSNR)
                // Use the extracted middle frames for objective comparison
                let img_orig = image::open(&orig_frames[1])
                    .map_err(|e| crate::error::Error::Internal(e.to_string()))?
                    .to_rgb8();
                let mut img_opt_raw = image::open(&opt_frames[1])
                    .map_err(|e| crate::error::Error::Internal(e.to_string()))?;

                // Resize if dimensions differ
                if img_orig.width() != img_opt_raw.width()
                    || img_orig.height() != img_opt_raw.height()
                {
                    img_opt_raw = img_opt_raw.resize_exact(
                        img_orig.width(),
                        img_orig.height(),
                        image::imageops::FilterType::Lanczos3,
                    );
                }
                let img_opt = img_opt_raw.to_rgb8();

                let ssim_result = image_compare::rgb_similarity_structure(
                    &image_compare::Algorithm::MSSIMSimple,
                    &img_orig,
                    &img_opt,
                )
                .map_err(|e| crate::error::Error::Internal(format!("SSIM error: {:?}", e)))?;

                let rms_result = image_compare::rgb_similarity_structure(
                    &image_compare::Algorithm::RootMeanSquared,
                    &img_orig,
                    &img_opt,
                )
                .map_err(|e| crate::error::Error::Internal(format!("RMS error: {:?}", e)))?;

                let rms_score = rms_result.score;
                // PSNR = 20 * log10(MAX_I / RMSE)
                // In image-compare, RMS score is 1.0 - (RMSE / 255)
                let rmse = (1.0 - rms_score) * 255.0;
                let psnr = if rmse > 0.0 {
                    20.0 * (255.0f64 / rmse).log10()
                } else {
                    99.0 // Perfect match
                };

                if let Some(obj) = val.as_object_mut() {
                    obj.insert("ssim".to_string(), serde_json::json!(ssim_result.score));
                    obj.insert("psnr".to_string(), serde_json::json!(psnr));
                }

                return Ok(val);
            }
        }

        Err(crate::error::Error::Internal(format!(
            "Failed to parse JSON response from AI: {}",
            text
        )))
    }
}
