//! Evaluation and Benchmarking utilities.

use crate::error::Result;
use gemini_client_api::gemini::{
    ask::Gemini,
    types::sessions::Session,
    utils::MarkdownToParts,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use serde_json::Value;

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
        Command::new("ffmpeg").args(&["-y", "-i", input.to_str().unwrap(), "-frames:v", "1", "-update", "1", f1.to_str().unwrap()])
            .output().ok();
        frames.push(f1);

        let f2 = temp_dir.join(format!("{}_2.png", prefix));
        Command::new("ffmpeg").args(&["-y", "-i", input.to_str().unwrap(), "-vf", "select='not(mod(n,60))'", "-frames:v", "1", "-update", "1", f2.to_str().unwrap()])
            .output().ok();
        frames.push(f2);

        let f3 = temp_dir.join(format!("{}_3.png", prefix));
        Command::new("ffmpeg").args(&["-y", "-i", input.to_str().unwrap(), "-sseof", "-1", "-frames:v", "1", "-update", "1", f3.to_str().unwrap()])
            .output().ok();
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

        let parts = MarkdownToParts::new(&prompt, |_| mime::IMAGE_PNG).await.process();
        let response = self.ai.ask(session.ask(parts)).await
            .map_err(|e| crate::error::Error::Internal(e.to_string()))?;
        
        let text = response.get_chat().get_text_no_think("");
        
        if let Some(json_start) = text.find('{') {
            if let Some(json_end) = text.rfind('}') {
                let json_str = &text[json_start..=json_end];
                let val: Value = serde_json::from_str(json_str)
                    .map_err(|e| crate::error::Error::Internal(e.to_string()))?;
                return Ok(val);
            }
        }
        
        Err(crate::error::Error::Internal(format!("Failed to parse JSON response from AI: {}", text)))
    }
}
