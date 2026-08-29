import { AppConfig, AudioState, ThemeDefinition } from '../types';
import { AudioVisualizer } from '../visualizer/canvas';

export class FrostedCardTheme implements ThemeDefinition {
  id = 'frosted-card';
  name = 'Frosted Fluent Card';
  description = 'Modern acrylic glass card with dynamic equalizer and album artwork';

  private visualizer: AudioVisualizer | null = null;
  private container: HTMLElement | null = null;

  init(container: HTMLElement) {
    this.container = container;
    container.innerHTML = `
      <div class="theme-frosted-card">
        <div class="frosted-glow"></div>
        <div class="frosted-content">
          <div class="cover-wrapper">
            <img class="album-cover" src="" alt="Cover" />
            <div class="cover-fallback">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M9 18V5l12-2v13"></path>
                <circle cx="6" cy="18" r="3"></circle>
                <circle cx="18" cy="16" r="3"></circle>
              </svg>
            </div>
            <div class="playing-pulse"></div>
          </div>
          <div class="meta-section">
            <div class="title-marquee-container">
              <div class="track-title">Waiting for audio...</div>
            </div>
            <div class="artist-app-row">
              <span class="source-badge">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z"/>
                </svg>
                <span class="app-name">System</span>
              </span>
              <span class="artist-name">No active sound</span>
            </div>
            <div class="visualizer-container">
              <canvas class="visualizer-canvas"></canvas>
            </div>
          </div>
        </div>
      </div>
    `;

    const canvas = container.querySelector('.visualizer-canvas') as HTMLCanvasElement;
    if (canvas) {
      this.visualizer = new AudioVisualizer(canvas, 'bars');
      this.visualizer.setColors('rgba(129, 140, 248, 0.95)', 'rgba(236, 72, 153, 0.95)');
    }
  }

  render(container: HTMLElement, state: AudioState, config: AppConfig) {
    const card = container.querySelector('.theme-frosted-card') as HTMLElement;
    if (!card) return;

    // Update sensitivity
    if (this.visualizer) {
      this.visualizer.setSensitivity(config.sensitivity);
      this.visualizer.updatePeak(state.overall_peak);
    }

    const titleEl = card.querySelector('.track-title') as HTMLElement;
    const artistEl = card.querySelector('.artist-name') as HTMLElement;
    const appEl = card.querySelector('.app-name') as HTMLElement;
    const imgEl = card.querySelector('.album-cover') as HTMLImageElement;
    const fallbackEl = card.querySelector('.cover-fallback') as HTMLElement;
    const pulseEl = card.querySelector('.playing-pulse') as HTMLElement;

    const displayTitle = state.track_title || (state.active_apps[0]?.name ?? 'System Audio');
    const displayArtist = state.artist || (state.source_app ? `Source: ${state.source_app}` : 'Active');
    const displayApp = state.source_app || (state.active_apps[0]?.name ?? 'Windows');

    if (titleEl && titleEl.textContent !== displayTitle) {
      titleEl.textContent = displayTitle;

      // Animate marquee if long
      if (displayTitle.length > 24) {
        titleEl.classList.add('animate-marquee');
      } else {
        titleEl.classList.remove('animate-marquee');
      }
    }

    if (artistEl) artistEl.textContent = displayArtist;
    if (appEl) appEl.textContent = displayApp;

    // Artwork
    if (state.thumbnail) {
      imgEl.src = state.thumbnail;
      imgEl.style.display = 'block';
      fallbackEl.style.display = 'none';
    } else {
      imgEl.style.display = 'none';
      fallbackEl.style.display = 'flex';
    }

    if (state.has_audio || state.is_playing) {
      pulseEl.classList.add('active');
    } else {
      pulseEl.classList.remove('active');
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
