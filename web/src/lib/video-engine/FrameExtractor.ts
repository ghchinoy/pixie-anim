import { WebmDemuxer } from './WebmDemuxer';

export interface FrameCallback {
  (frame: VideoFrame): Promise<void>;
}

export class FrameExtractor {
  private decoder: VideoDecoder;
  private demuxer: WebmDemuxer | null = null;
  private frameCount = 0;
  private onFrame: FrameCallback;

  constructor(onFrame: FrameCallback) {
    this.onFrame = onFrame;
    this.decoder = new VideoDecoder({
      output: async (frame) => {
        this.frameCount++;
        await this.onFrame(frame);
      },
      error: (e) => console.error('VideoDecoder Error:', e),
    });
  }

  public async extract(fileBuffer: Uint8Array): Promise<number> {
    this.demuxer = new WebmDemuxer(fileBuffer);
    this.frameCount = 0;

    // 1. Basic configuration (VP8/VP9 detection)
    // For a production demuxer, we'd parse the Tracks element properly.
    // For this P0, we'll try to find the Cluster and start decoding.
    
    // Minimal configuration for VP9 (most common in modern WebM)
    // In a real implementation, we'd extract the codec string from EBML.
    const config: VideoDecoderConfig = {
      codec: 'vp09.00.10.08', 
      optimizeForLatency: false,
    };

    if (!(await VideoDecoder.isConfigSupported(config)).supported) {
      throw new Error('VP9 decoding not supported by this browser');
    }

    this.decoder.configure(config);

    // 2. Scan for Clusters and Blocks
    let element = this.demuxer.getNextElement();
    let hasFoundFirstKeyframe = false;

    while (element) {
      if (element.id === WebmDemuxer.Segment || element.id === WebmDemuxer.Cluster) {
        // Step into container
      } else if (element.id === WebmDemuxer.SimpleBlock) {
        // SimpleBlock Header:
        // 1. Track Number (VINT)
        // 2. Timecode (2 bytes)
        // 3. Flags (1 byte)
        
        // Save current offset to restore after reading track number
        const startOffset = this.demuxer.getOffset();
        const trackVint = this.demuxer.readVint();
        const flagsOffset = trackVint.length + 2;
        
        const data = fileBuffer.slice(element.dataOffset, element.dataOffset + element.size);
        const flags = data[flagsOffset];
        const isKeyframe = (flags & 0x80) !== 0;

        if (!hasFoundFirstKeyframe && !isKeyframe) {
          // Skip until we find a keyframe to satisfy VideoDecoder
          this.demuxer.seek(startOffset + element.size); 
        } else {
          hasFoundFirstKeyframe = true;
          const chunk = new EncodedVideoChunk({
            type: isKeyframe ? 'key' : 'delta',
            timestamp: this.frameCount * 33333, // Placeholder 30fps
            data: data.slice(flagsOffset + 1), // Payload follows flags
          });
          this.decoder.decode(chunk);
        }
        
        // Restore/Advance offset
        this.demuxer.seek(startOffset + element.size);
      } else {
        this.demuxer.skip(element.size);
      }
      element = this.demuxer.getNextElement();
    }

    await this.decoder.flush();
    return this.frameCount;
  }
}
