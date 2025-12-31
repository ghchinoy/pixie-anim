/**
 * A minimal, zero-dependency EBML parser for WebM demuxing.
 * Focused only on extracting video clusters and blocks for WebCodecs.
 */

export interface EbmlElement {
  id: number;
  size: number;
  dataOffset: number;
}

export class WebmDemuxer {
  private offset = 0;
  private data: Uint8Array;

  constructor(buffer: Uint8Array) {
    this.data = buffer;
  }

  /**
   * Reads a Variable Size Integer (VINT) used in EBML.
   */
  private readVint(): { value: number; length: number } {
    const firstByte = this.data[this.offset];
    let length = 1;
    let mask = 0x80;

    while (length <= 8 && !(firstByte & mask)) {
      length++;
      mask >>= 1;
    }

    if (length > 8) throw new Error('VINT too long');

    let value = firstByte & (mask - 1);
    for (let i = 1; i < length; i++) {
      value = (value << 8) | this.data[this.offset + i];
    }

    return { value, length };
  }

  /**
   * Parses the next EBML element at the current offset.
   */
  public getNextElement(): EbmlElement | null {
    if (this.offset >= this.data.length) return null;

    const idVint = this.readVint();
    const id = idVint.value | (1 << (7 * idVint.length)); // Keep the VINT marker for the ID
    this.offset += idVint.length;

    const sizeVint = this.readVint();
    const size = sizeVint.value;
    this.offset += sizeVint.length;

    const element = { id, size, dataOffset: this.offset };
    return element;
  }

  public skip(size: number) {
    this.offset += size;
  }

  public seek(offset: number) {
    this.offset = offset;
  }

  public getOffset(): number {
    return this.offset;
  }

  // EBML IDs for WebM
  static EBML = 0x1A45DFA3;
  static Segment = 0x18538067;
  static Tracks = 0x1654AE6B;
  static Cluster = 0x1F43B675;
  static SimpleBlock = 0xA3;
  static BlockGroup = 0xA0;
  static Block = 0xA1;
}
