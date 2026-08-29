import { AppConfig, AudioState, ThemeDefinition } from '../types';
import { AudioVisualizer } from '../visualizer/canvas';

export class MinimalHudTheme implements ThemeDefinition {
  id = 'minimal-hud';
  name = 'Minimal HUD';
  description = 'Ultra-compact floating pill with mini equalizer and app icon';

  private visualizer: AudioVisualizer | null = null;
  private container: HTMLElement | null = null;

  init(container: HTMLElement) {
    this.container = container;
    container.innerHTML = `
      <div class="theme-minimal-hud">
        <div class="hud-pill">
          <div class="hud-indicator"></div>
          <div class="hud-meta">
            <span class="hud-app">Audio</span>
            <span class="hud-divider">/</span>
            <span class="hud-title">Active</span>
          </div>
          <div class="hud-visualizer">
            <canvas class="hud-canvas"></canvas>
          </div>
        </div>
      </div>
    `;

    const canvas = container.querySelector('.hud-canvas') as HTMLCanvasElement;
    if (canvas) {
      this.visualizer = new AudioVisualizer(canvas, 'bars');
      this.visualizer.setColors('#38bdf8', '#818cf8');
    }
  }

  render(container: HTMLElement, state: AudioState, config: AppConfig) {
    const hud = container.querySelector('.theme-minimal-hud') as HTMLElement;
    if (!hud) return;

    if (this.visualizer) {
      this.visualizer.setSensitivity(config.sensitivity);
      this.visualizer.updatePeak(state.overall_peak);
    }

    const appEl = hud.querySelector('.hud-app') as HTMLElement;
    const titleEl = hud.querySelector('.hud-title') as HTMLElement;
    const indicator = hud.querySelector('.hud-indicator') as HTMLElement;

    const displayApp = state.source_app || (state.active_apps[0]?.name ?? 'System');
    const displayTitle = state.track_title || (state.is_playing ? 'Playing' : 'Idle');

    if (appEl) appEl.textContent = displayApp;
    if (titleEl) titleEl.textContent = displayTitle;

    if (state.has_audio || state.is_playing) {
      indicator.classList.add('playing');
    } else {
      indicator.classList.remove('playing');
    }
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
