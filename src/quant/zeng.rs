//! Zeng Palette Reordering.
//!
//! Reorders palette indices to improve LZW compression by ensuring 
//! visually similar colors have adjacent indices.

use crate::quant::Palette;

/// Reorders the palette using a greedy TSP approximation.
/// Returns (new_palette, index_mapping) where index_mapping[old_idx] = new_idx.
pub fn reorder_palette(palette: &Palette) -> (Palette, Vec<u8>) {
    let colors = &palette.colors;
    if colors.is_empty() {
        return (Palette { colors: Vec::new() }, Vec::new());
    }

    let mut new_colors = Vec::with_capacity(colors.len());
    let mut mapping = vec![0u8; colors.len()];
    let mut visited = vec![false; colors.len()];

    // Start with the first color (usually the most significant from K-Means)
    let mut current_idx = 0;
    new_colors.push(colors[current_idx]);
    visited[current_idx] = true;
    mapping[current_idx] = 0;

    for i in 1..colors.len() {
        let mut min_dist = u32::MAX;
        let mut next_idx = 0;

        let current_color = colors[current_idx];

        for (j, &color) in colors.iter().enumerate() {
            if !visited[j] {
                let dr = current_color.r as i32 - color.r as i32;
                let dg = current_color.g as i32 - color.g as i32;
                let db = current_color.b as i32 - color.b as i32;
                let dist = (dr * dr + dg * dg + db * db) as u32;

                if dist < min_dist {
                    min_dist = dist;
                    next_idx = j;
                }
            }
        }

        visited[next_idx] = true;
        mapping[next_idx] = i as u8;
        new_colors.push(colors[next_idx]);
        current_idx = next_idx;
    }

    (Palette { colors: new_colors }, mapping)
}
