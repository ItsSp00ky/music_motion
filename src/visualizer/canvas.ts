export class AudioVisualizer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private bars: number[] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  private targetBars: number[] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  private isRunning: boolean = false;
  private peak: number = 0;
  private sensitivity: number = 1.0;
  private mode: 'bars' | 'wave' | 'dots' = 'bars';
  private primaryColor: string = 'rgba(99, 102, 241, 0.9)';
  private secondaryColor: string = 'rgba(236, 72, 153, 0.9)';

  constructor(canvas: HTMLCanvasElement, mode: 'bars' | 'wave' | 'dots' = 'bars') {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d')!;
    this.mode = mode;
    this.startAnimation();
  }

  public setMode(mode: 'bars' | 'wave' | 'dots') {
    this.mode = mode;
  }

  public setColors(primary: string, secondary: string) {
    this.primaryColor = primary;
    this.secondaryColor = secondary;
  }

  public setSensitivity(s: number) {
    this.sensitivity = Math.max(0.2, Math.min(3.0, s));
  }

  public updatePeak(rawPeak: number) {
    this.peak = Math.min(1.0, rawPeak * this.sensitivity);

    // Simulate spectrum frequencies from real-time peak with natural variation
    for (let i = 0; i < this.targetBars.length; i++) {
      const freqWeight = Math.sin((i / (this.targetBars.length - 1)) * Math.PI) * 0.4 + 0.6;
      const noise = (Math.random() * 0.4 + 0.8);
      this.targetBars[i] = Math.min(1.0, this.peak * freqWeight * noise);
    }
  }

  private startAnimation() {
    if (this.isRunning) return;
    this.isRunning = true;

    const render = () => {
      if (!this.isRunning) return;

      const dpr = window.devicePixelRatio || 1;
      const rect = this.canvas.getBoundingClientRect();
      if (this.canvas.width !== rect.width * dpr || this.canvas.height !== rect.height * dpr) {
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;
        this.ctx.scale(dpr, dpr);
      }

      const w = rect.width;
      const h = rect.height;

      this.ctx.clearRect(0, 0, w, h);

      // Smooth bar lerping (fast rise, smooth decay)
      for (let i = 0; i < this.bars.length; i++) {
        const target = this.targetBars[i];
        if (target > this.bars[i]) {
          this.bars[i] += (target - this.bars[i]) * 0.4;
        } else {
          this.bars[i] += (target - this.bars[i]) * 0.15;
        }
      }

      if (this.mode === 'bars') {
        this.drawBars(w, h);
      } else if (this.mode === 'wave') {
        this.drawWave(w, h);
      } else {
        this.drawDots(w, h);
      }

      requestAnimationFrame(render);
    };

    requestAnimationFrame(render);
  }

  private drawBars(w: number, h: number) {
    const barCount = this.bars.length;
    const gap = 3;
    const barWidth = Math.max(2, (w - (barCount - 1) * gap) / barCount);

    const gradient = this.ctx.createLinearGradient(0, h, 0, 0);
    gradient.addColorStop(0, this.primaryColor);
    gradient.addColorStop(1, this.secondaryColor);

    this.ctx.fillStyle = gradient;

    for (let i = 0; i < barCount; i++) {
      const val = Math.max(0.08, this.bars[i]);
      const barHeight = Math.max(3, val * h);
      const x = i * (barWidth + gap);
      const y = h - barHeight;

      // Rounded top rectangle
      const radius = Math.min(barWidth / 2, 2);
      this.ctx.beginPath();
      this.ctx.roundRect(x, y, barWidth, barHeight, [radius, radius, 0, 0]);
      this.ctx.fill();
    }
  }

  private drawWave(w: number, h: number) {
    const gradient = this.ctx.createLinearGradient(0, 0, w, 0);
    gradient.addColorStop(0, this.primaryColor);
    gradient.addColorStop(1, this.secondaryColor);

    this.ctx.beginPath();
    this.ctx.moveTo(0, h / 2);

    const points = this.bars.length;
    for (let i = 0; i < points; i++) {
      const x = (i / (points - 1)) * w;
      const offset = (this.bars[i] * h * 0.45) * Math.sin(Date.now() * 0.005 + i);
      const y = (h / 2) + offset;
      if (i === 0) {
        this.ctx.moveTo(x, y);
      } else {
        this.ctx.lineTo(x, y);
      }
    }

    this.ctx.strokeStyle = gradient;
    this.ctx.lineWidth = 2.5;
    this.ctx.lineCap = 'round';
    this.ctx.lineJoin = 'round';
    this.ctx.stroke();
  }

  private drawDots(w: number, h: number) {
    const barCount = this.bars.length;
    const spacing = w / barCount;

    for (let i = 0; i < barCount; i++) {
      const val = Math.max(0.1, this.bars[i]);
      const x = i * spacing + spacing / 2;
      const y = h / 2;
      const r = Math.max(2, val * (h / 3));

      this.ctx.beginPath();
      this.ctx.arc(x, y, r, 0, Math.PI * 2);
      this.ctx.fillStyle = this.primaryColor;
      this.ctx.fill();
    }
  }

  public destroy() {
    this.isRunning = false;
  }
}
