import { AppConfig, AudioState, ThemeDefinition } from '../types';
import { AudioVisualizer } from '../visualizer/canvas';

export class CyberNeonTheme implements ThemeDefinition {
  id = 'cyber-neon';
  name = 'Cyber Neon';
  description = 'High contrast cyberpunk glowing neon audio visualizer card';

  private visualizer: AudioVisualizer | null = null;
  private container: HTMLElement | null = null;

  init(container: HTMLElement) {
    this.container = container;
    container.innerHTML = `
      <div class="theme-cyber-neon">
        <div class="neon-card">
          <div class="neon-header">
            <span class="neon-badge">AUDIO ACTIVE</span>
            <span class="neon-app">CYBER // SYSTEM</span>
          </div>
          <div class="neon-track-name">READY</div>
          <div class="neon-visualizer">
            <canvas class="neon-canvas"></canvas>
          </div>
        </div>
      </div>
    `;

    const canvas = container.querySelector('.neon-canvas') as HTMLCanvasElement;
    if (canvas) {
      this.visualizer = new AudioVisualizer(canvas, 'bars');
      this.visualizer.setColors('#06b6d4', '#f43f5e');
    }
  }

  render(container: HTMLElement, state: AudioState, config: AppConfig) {
    const card = container.querySelector('.theme-cyber-neon') as HTMLElement;
    if (!card) return;

    if (this.visualizer) {
      this.visualizer.setSensitivity(config.sensitivity);
      this.visualizer.updatePeak(state.overall_peak);
    }

    const appEl = card.querySelector('.neon-app') as HTMLElement;
    const trackEl = card.querySelector('.neon-track-name') as HTMLElement;

    const displayApp = (state.source_app || state.active_apps[0]?.name || 'AUDIO').toUpperCase();
    const displayTitle = (state.track_title || 'LIVE SOUND OUTPUT').toUpperCase();

    if (appEl) appEl.textContent = displayApp;
    if (trackEl) trackEl.textContent = displayTitle;
  }

  destroy() {
    if (this.visualizer) {
      this.visualizer.destroy();
      this.visualizer = null;
    }
    if (this.container) {
      this.container.innerHTML = '';
    }
  }
}
