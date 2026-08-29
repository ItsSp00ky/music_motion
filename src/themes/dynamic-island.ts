import { AppConfig, AudioState, ThemeDefinition } from '../types';
import { AudioVisualizer } from '../visualizer/canvas';

export class DynamicIslandTheme implements ThemeDefinition {
  id = 'dynamic-island';
  name = 'Dynamic Island';
  description = 'Morphing dynamic pill that expands smoothly during active music playback';

  private visualizer: AudioVisualizer | null = null;
  private container: HTMLElement | null = null;

  init(container: HTMLElement) {
    this.container = container;
    container.innerHTML = `
      <div class="theme-dynamic-island">
        <div class="island-pill">
          <div class="island-left">
            <div class="island-icon">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>
              </svg>
            </div>
            <div class="island-text">
              <div class="island-title">Music</div>
              <div class="island-subtitle">Active</div>
            </div>
          </div>
          <div class="island-right">
            <canvas class="island-canvas"></canvas>
          </div>
        </div>
      </div>
    `;

    const canvas = container.querySelector('.island-canvas') as HTMLCanvasElement;
    if (canvas) {
      this.visualizer = new AudioVisualizer(canvas, 'wave');
      this.visualizer.setColors('#ec4899', '#a855f7');
    }
  }

  render(container: HTMLElement, state: AudioState, config: AppConfig) {
    const pill = container.querySelector('.island-pill') as HTMLElement;
    if (!pill) return;

    if (this.visualizer) {
      this.visualizer.setSensitivity(config.sensitivity);
      this.visualizer.updatePeak(state.overall_peak);
    }

    const titleEl = pill.querySelector('.island-title') as HTMLElement;
    const subEl = pill.querySelector('.island-subtitle') as HTMLElement;

    const displayTitle = state.track_title || (state.active_apps[0]?.name ?? 'System Audio');
    const displaySub = state.artist || state.source_app || 'Sound Playing';

    if (titleEl) titleEl.textContent = displayTitle;
    if (subEl) subEl.textContent = displaySub;

    if (state.has_audio || state.is_playing) {
      pill.classList.add('expanded');
    } else {
      pill.classList.remove('expanded');
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
