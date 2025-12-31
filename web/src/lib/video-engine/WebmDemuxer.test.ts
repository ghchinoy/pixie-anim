import { describe, it, expect } from 'vitest';
import { WebmDemuxer } from './WebmDemuxer';

describe('WebmDemuxer', () => {
  it('should parse EBML VINTs correctly', () => {
    // 0x81 -> length 1, value 1
    // 0x4002 -> length 2, value 2
    const data = new Uint8Array([0x1A, 0x45, 0xDF, 0xA3, 0x81, 0x01]);
    const demuxer = new WebmDemuxer(data);
    
    const element = demuxer.getNextElement();
    expect(element).not.toBeNull();
    // 0x1A45DFA3 is the EBML header ID
    expect(element?.id).toBe(WebmDemuxer.EBML);
    expect(element?.size).toBe(1);
    expect(element?.dataOffset).toBe(5);
  });

  it('should handle multi-byte sizes', () => {
    // ID: 0xA3 (SimpleBlock), Size: 0x4080 (128 bytes)
    const data = new Uint8Array([0xA3, 0x40, 0x80, 0x00, 0x01, 0x02]);
    const demuxer = new WebmDemuxer(data);
    
    const element = demuxer.getNextElement();
    expect(element?.id).toBe(WebmDemuxer.SimpleBlock);
    expect(element?.size).toBe(128);
  });
});
