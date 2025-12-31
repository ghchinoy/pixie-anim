import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import init, { encodeGif, decodeGif } from './lib/pixie-wasm/pixie.js';
import { FrameExtractor } from './lib/video-engine/FrameExtractor.js';

@customElement('pixo-app')
export class PixoApp extends LitElement {
  static styles = css`
    :host { display: block; --surface: #111; --border: #2a2a2a; --accent: #3b82f6; }
    * { font-family: system-ui, -apple-system, sans-serif; }
    .main-grid { display: flex; flex-direction: column; gap: 1.5rem; }
    .settings-bar { 
      background: var(--surface); 
      border: 1px solid var(--border);
      padding: 0.75rem 1rem;
      border-radius: 4px;
      display: flex;
      gap: 1.25rem;
      align-items: center;
      font-size: 0.75rem;
    }
    .control-group { display: flex; align-items: center; gap: 0.5rem; }
    input[type='range'] { accent-color: var(--accent); width: 80px; }
    input[type='number'] { 
      background: #000; border: 1px solid var(--border); color: #fff; 
      padding: 2px 4px; font-family: inherit; width: 35px; 
    }
    .dropzone { 
      border: 1px dashed var(--border);
      padding: 5rem 2rem;
      border-radius: 4px;
      text-align: center;
      cursor: pointer;
      background: #0d0d0d;
      transition: 0.2s;
    }
    .dropzone:hover { border-color: var(--accent); background: #111; }
    .dropzone h3 { font-size: 1rem; margin-bottom: 0.5rem; }
    .dropzone p { font-size: 0.75rem; color: #666; }
    .preview-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
    .panel { 
      background: var(--surface); 
      border: 1px solid var(--border);
      padding: 1rem;
      border-radius: 4px;
      display: flex;
      flex-direction: column;
      align-items: center;
    }
    .panel-label { 
      font-size: 0.7rem; 
      color: #666; 
      margin-bottom: 1rem; 
      width: 100%;
      text-transform: uppercase;
      letter-spacing: 0.05rem;
    }
    img, video { max-width: 100%; border-radius: 2px; }
    .stats { 
      margin-top: 1rem; 
      width: 100%; 
      font-size: 0.75rem; 
      display: flex; 
      justify-content: space-between;
      color: #aaa;
      font-family: monospace;
    }
    .accent { color: var(--accent); font-weight: bold; }
    .btn {
      background: #fff; color: #000; border: none; padding: 0.4rem 1rem;
      border-radius: 2px; font-family: inherit; font-size: 0.75rem;
      font-weight: bold; cursor: pointer;
    }
    .btn:hover { background: #ccc; }
    .btn-reprocess { background: var(--accent); color: #000; }
    .btn-ghost { background: transparent; color: #666; border: 1px solid var(--border); }
    .loader { padding: 4rem; text-align: center; font-size: 0.8rem; }
    .spinner {
      width: 40px;
      height: 40px;
      border: 3px solid rgba(59, 130, 246, 0.1);
      border-top: 3px solid var(--accent);
      border-radius: 50%;
      margin: 0 auto 1.5rem;
      animation: spin 1s linear infinite;
    }
    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }
  `;

  @state() private processing = false;
  @state() private resultUrl = '';
  @state() private originalUrl = '';
  @state() private originalSize = 0;
  @state() private optimizedSize = 0;
  @state() private status = '';
  @state() private timeTaken = 0;
  @state() private originalType = '';
  @state() private sourceFps = 0;
  
  private sourceBuffer: Uint8Array | null = null;
  private sourceWidth = 0;
  private sourceHeight = 0;
  private sourceFrames = 0;

  @state() private quality = 10;
  @state() private fps = 12;
  @state() private lossy = 8;
  @state() private fuzz = 10;

  async firstUpdated() {
    await init();
    console.log('🚀 Pixie-Anim WASM Initialized');
  }

  formatSize(bytes: number) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  render() {
    return html`
      <div class="main-grid">
        <div class="settings-bar">
          <div class="control-group">
            <label>QUALITY</label>
            <input type="range" min="1" max="20" .value=${this.quality} @input=${(e: any) => this.quality = parseInt(e.target.value)}>
            <span style="min-width: 20px">${this.quality}</span>
          </div>
          <div class="control-group">
            <label>LOSSY</label>
            <input type="range" min="0" max="20" .value=${this.lossy} @input=${(e: any) => this.lossy = parseInt(e.target.value)}>
            <span style="min-width: 20px">${this.lossy}</span>
          </div>
          <div class="control-group">
            <label>FUZZY</label>
            <input type="range" min="0" max="50" .value=${this.fuzz} @input=${(e: any) => this.fuzz = parseInt(e.target.value)}>
            <span style="min-width: 20px">${this.fuzz}</span>
          </div>
          <div class="control-group">
            <label>FPS</label>
            <input type="number" .value=${this.fps} @input=${(e: any) => this.fps = parseInt(e.target.value)}>
          </div>
          ${this.sourceBuffer ? html`
            <button class="btn btn-reprocess" ?disabled=${this.processing} @click=${this._reprocess}>
              RE-OPTIMIZE
            </button>
          ` : ''}
        </div>

        ${!this.resultUrl && !this.processing ? html`
          <div class="dropzone" @click=${this._triggerFile} @dragover=${this._handleDragOver} @drop=${this._handleDrop}>
            <h3>DROP MP4, WEBM OR GIF</h3>
            <p>Directly convert and optimize for the web</p>
            <button class="btn mt-2">SELECT FILE</button>
            <input type="file" id="fileInput" hidden accept="video/mp4,video/webm,image/gif" @change=${this._handleFileSelect}>
          </div>
        ` : ''}

        ${this.processing ? html`
          <div class="loader">
            <div class="spinner"></div>
            <div>${this.status}</div>
          </div>
        ` : ''}

        ${this.resultUrl ? html`
          <div class="preview-grid">
            <div class="panel">
              <div class="panel-label">Original ${this.originalType.replace('video/', '').toUpperCase().replace('IMAGE/', '')}</div>
              ${this.originalType.startsWith('video/') 
                ? html`<video src="${this.originalUrl}" autoplay loop muted></video>`
                : html`<img src="${this.originalUrl}">`}
              <div class="stats">
                <span>SIZE</span>
                <span>${this.formatSize(this.originalSize)}</span>
              </div>
              ${this.sourceFps ? html`
                <div class="stats">
                  <span>SOURCE FPS</span>
                  <span>${this.sourceFps.toFixed(1)}</span>
                </div>
              ` : ''}
            </div>
            <div class="panel">
              <div class="panel-label">Pixie-Anim Optimized (GIF)</div>
              <img src="${this.resultUrl}">
              <div class="stats">
                <span>SIZE</span>
                <span>${this.formatSize(this.optimizedSize)}</span>
              </div>
              <div class="stats">
                <span>REDUCTION</span>
                <span class="accent">${(100 - (this.optimizedSize/this.originalSize)*100).toFixed(1)}%</span>
              </div>
              <div class="stats">
                <span>TIME</span>
                <span>${(this.timeTaken / 1000).toFixed(2)}s</span>
              </div>
            </div>
          </div>
          <div style="display: flex; gap: 1rem; justify-content: center; margin-top: 1rem;">
            <button class="btn btn-ghost" @click=${this._clear}>CLEAR</button>
            <a href="${this.resultUrl}" download="pixie.gif"><button class="btn">DOWNLOAD GIF</button></a>
          </div>
        ` : ''}
      </div>
    `;
  }

  private _clear() {
    if (this.resultUrl) URL.revokeObjectURL(this.resultUrl);
    if (this.originalUrl) URL.revokeObjectURL(this.originalUrl);
    this.resultUrl = '';
    this.originalUrl = '';
    this.sourceBuffer = null;
    this.sourceFps = 0;
    this.status = '';
  }

  private _triggerFile() {
    this.shadowRoot?.getElementById('fileInput')?.click();
  }

  private _handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
  }

  private _handleDrop(e: DragEvent) {
    e.preventDefault();
    const files = e.dataTransfer?.files;
    if (files) this._processFiles(Array.from(files));
  }

  private _handleFileSelect(e: Event) {
    const files = (e.target as HTMLInputElement).files;
    if (files) this._processFiles(Array.from(files));
  }

  private async _reprocess() {
    if (!this.sourceBuffer) return;
    await this._runOptimization(this.sourceBuffer, this.sourceWidth, this.sourceHeight, this.sourceFrames);
  }

  private async _processFiles(files: File[]) {
    if (files.length === 0) return;
    this.processing = true;
    
    if (this.resultUrl) URL.revokeObjectURL(this.resultUrl);
    if (this.originalUrl) URL.revokeObjectURL(this.originalUrl);
    
    this.resultUrl = '';
    const file = files[0];
    this.originalSize = file.size;
    this.originalType = file.type;
    this.originalUrl = URL.createObjectURL(file);
    
    try {
      if (file.type === 'image/gif') {
        this.status = 'DECODING GIF...';
        const arrayBuffer = await file.arrayBuffer();
        const rawData = decodeGif(new Uint8Array(arrayBuffer));
        
        const view = new DataView(rawData.buffer, rawData.byteOffset, rawData.byteLength);
        this.sourceWidth = view.getUint16(0, true);
        this.sourceHeight = view.getUint16(2, true);
        this.sourceFrames = view.getUint32(4, true);
        const avgDelayMs = view.getUint32(8, true);
        
        this.sourceFps = avgDelayMs === 0 ? 10 : 1000 / avgDelayMs;
        this.fps = Math.round(this.sourceFps);
        
        this.sourceBuffer = new Uint8Array(rawData.buffer, rawData.byteOffset + 12, rawData.byteLength - 12);
      } else if (file.type.startsWith('video/')) {
        this.status = `ANALYZING ${file.type.split('/')[1].toUpperCase()}...`;
        
        let result;
        // Prefer WebCodecs for WebM (and MP4 if not on Safari which has limited WebCodecs support for some containers)
        if (file.type === 'video/webm' || file.type === 'video/mp4') {
          try {
            result = await this._extractFramesViaWebCodecs(file);
          } catch (e) {
            console.warn('WebCodecs failed, falling back to Canvas extraction:', e);
            result = await this._extractFramesFromVideo(file);
          }
        } else {
          result = await this._extractFramesFromVideo(file);
        }

        const { buffer, width, height, numFrames, estimatedFps } = result;
        this.sourceWidth = width;
        this.sourceHeight = height;
        this.sourceFrames = numFrames;
        this.sourceBuffer = buffer;
        this.sourceFps = estimatedFps;
        this.fps = Math.round(estimatedFps);
      } else {
        throw new Error('Unsupported file type. Please use MP4, WebM or GIF.');
      }

      await this._runOptimization(this.sourceBuffer!, this.sourceWidth, this.sourceHeight, this.sourceFrames);
    } catch (e) {
      console.error(e);
      alert('Error: ' + e);
      this.processing = false;
    }
  }

  private async _extractFramesViaWebCodecs(file: File): Promise<{buffer: Uint8Array, width: number, height: number, numFrames: number, estimatedFps: number}> {
    const arrayBuffer = await file.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);
    
    // We need to know width/height/fps. For this P0, we'll use a temporary video element 
    // just to get metadata, as parsing full WebM metadata in JS is complex.
    const metadata = await new Promise<{width: number, height: number, duration: number}>((resolve) => {
      const v = document.createElement('video');
      v.src = URL.createObjectURL(file);
      v.onloadedmetadata = () => {
        resolve({ width: v.videoWidth, height: v.videoHeight, duration: v.duration });
        URL.revokeObjectURL(v.src);
      };
    });

    const captureFps = 15;
    const numFramesExpected = Math.floor(metadata.duration * captureFps);
    const buffer = new Uint8Array(metadata.width * metadata.height * numFramesExpected * 4);
    let framesProcessed = 0;

    const extractor = new FrameExtractor(async (frame) => {
      if (framesProcessed < numFramesExpected) {
        // Copy the VideoFrame directly to our buffer as RGBA
        await frame.copyTo(buffer.subarray(framesProcessed * metadata.width * metadata.height * 4), {
          format: 'RGBA'
        });
      }
      framesProcessed++;
      frame.close();
      this.status = `EXTRACTING FRAME ${framesProcessed}...`;
    });

    await extractor.extract(bytes);

    return { 
      buffer, 
      width: metadata.width, 
      height: metadata.height, 
      numFrames: framesProcessed, 
      estimatedFps: captureFps 
    };
  }

  private async _extractFramesFromVideo(file: File): Promise<{buffer: Uint8Array, width: number, height: number, numFrames: number, estimatedFps: number}> {
    return new Promise((resolve, reject) => {
      const video = document.createElement('video');
      video.preload = 'auto';
      video.muted = true;
      video.src = URL.createObjectURL(file);
      
      video.onloadedmetadata = async () => {
        const width = video.videoWidth;
        const height = video.videoHeight;
        const duration = video.duration;
        
        const captureFps = 15;
        const interval = 1 / captureFps;
        const numFrames = Math.floor(duration * captureFps);
        
        const canvas = document.createElement('canvas');
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext('2d')!;
        
        const buffer = new Uint8Array(width * height * numFrames * 4);
        
        for (let i = 0; i < numFrames; i++) {
          this.status = `EXTRACTING FRAME ${i+1}/${numFrames}...`;
          video.currentTime = i * interval;
          await new Promise(r => video.onseeked = r);
          ctx.drawImage(video, 0, 0, width, height);
          const pixels = ctx.getImageData(0, 0, width, height).data;
          buffer.set(pixels, i * width * height * 4);
        }
        
        URL.revokeObjectURL(video.src);
        resolve({ buffer, width, height, numFrames, estimatedFps: captureFps });
      };
      
      video.onerror = () => reject('Failed to load video');
    });
  }

  private async _runOptimization(buffer: Uint8Array, width: number, height: number, numFrames: number) {
    if (this.resultUrl) {
      URL.revokeObjectURL(this.resultUrl);
      this.resultUrl = '';
    }
    
    this.processing = true;
    this.status = `OPTIMIZING ${numFrames} FRAMES...`;
    await new Promise(r => setTimeout(r, 100));

    try {
      const startTime = performance.now();
      const gifBytes = encodeGif(buffer, width, height, numFrames, this.fps, this.quality, this.lossy, this.fuzz);
      this.timeTaken = performance.now() - startTime;
      console.log(`✅ WASM Encoding took ${this.timeTaken}ms`);

      this.optimizedSize = gifBytes.length;
      const blob = new Blob([new Uint8Array(gifBytes)], { type: 'image/gif' });
      this.resultUrl = URL.createObjectURL(blob);
    } finally {
      this.processing = false;
    }
  }
}
