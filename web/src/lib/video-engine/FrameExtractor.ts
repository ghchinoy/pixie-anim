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
    while (element) {
      if (element.id === WebmDemuxer.Segment || element.id === WebmDemuxer.Cluster) {
        // Step into container
      } else if (element.id === WebmDemuxer.SimpleBlock) {
        const data = fileBuffer.slice(element.dataOffset, element.dataOffset + element.size);
        
        // WebM SimpleBlock format:
        // [Track Number (VINT)] [Timecode (2 bytes)] [Flags (1 byte)] [Payload]
        // For simplicity, we assume track 1 is video.
        const chunk = new EncodedVideoChunk({
          type: (data[3] & 0x80) ? 'key' : 'delta', // SimpleBlock keyframe bit
          timestamp: this.frameCount * 33333, // Placeholder timestamp (30fps)
          data: data.slice(4), // Approximate payload offset
        });
        
        this.decoder.decode(chunk);
      } else {
        this.demuxer.skip(element.size);
      }
      element = this.demuxer.getNextElement();
    }

    await this.decoder.flush();
    return this.frameCount;
  }
}
