import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ThemeEngine } from './themes/theme-engine';
import { AppConfig, AudioState } from './types';

const appEl = document.getElementById('app') as HTMLElement;
const contextMenu = document.getElementById('context-menu') as HTMLElement;
const menuCtStatus = document.getElementById('menu-ct-status') as HTMLElement;
const menuThemeName = document.getElementById('menu-theme-name') as HTMLElement;

const themeEngine = new ThemeEngine(appEl);

let currentConfig: AppConfig = {
  position: 'bottom-right',
  click_through: false,
  theme: 'frosted-card',
  sensitivity: 1.0,
  auto_hide_seconds: 0, // Keep visible in standby by default
  margin_x: 24,
  margin_y: 24,
};

const initialIdleState: AudioState = {
  has_audio: false,
  overall_peak: 0,
  is_playing: false,
  track_title: 'MusicMotion Active',
  artist: 'Play music in Spotify, YouTube, or apps',
  album: '',
  thumbnail: null,
  source_app: 'Standby',
  active_apps: [],
};

let lastActiveTime = Date.now();
let isIdleHidden = false;

// Initialize theme and render initial standby state immediately
themeEngine.setTheme(currentConfig.theme);
themeEngine.render(initialIdleState, currentConfig);

// Idle check loop
setInterval(() => {
  if (currentConfig.auto_hide_seconds <= 0) {
    if (isIdleHidden) {
      isIdleHidden = false;
      appEl.classList.remove('idle-hidden');
    }
    return;
  }

  const elapsed = (Date.now() - lastActiveTime) / 1000;
  if (elapsed >= currentConfig.auto_hide_seconds && !isIdleHidden) {
    isIdleHidden = true;
    appEl.classList.add('idle-hidden');
  }
}, 500);

function handleAudioUpdate(state: AudioState) {
  if (state.has_audio || state.is_playing) {
    lastActiveTime = Date.now();
    if (isIdleHidden) {
      isIdleHidden = false;
      appEl.classList.remove('idle-hidden');
    }
    themeEngine.render(state, currentConfig);
  } else {
    // Idle / no active sound state
    themeEngine.render(
      {
        ...state,
        track_title: state.track_title || 'MusicMotion Active',
        artist: state.artist || 'Waiting for audio playback...',
        source_app: state.source_app || 'Standby',
      },
      currentConfig
    );
  }
}

// Right-click context menu handling
window.addEventListener('contextmenu', (e) => {
  e.preventDefault();
  const x = Math.min(e.clientX, window.innerWidth - 180);
  const y = Math.min(e.clientY, window.innerHeight - 120);
  contextMenu.style.left = `${x}px`;
  contextMenu.style.top = `${y}px`;
  contextMenu.classList.add('open');

  if (menuCtStatus) menuCtStatus.textContent = currentConfig.click_through ? 'ON' : 'OFF';
  if (menuThemeName) menuThemeName.textContent = currentConfig.theme;
});

window.addEventListener('click', (e) => {
  if (!contextMenu.contains(e.target as Node)) {
    contextMenu.classList.remove('open');
  }
});

contextMenu.addEventListener('click', async (e) => {
  const target = (e.target as HTMLElement).closest('.menu-item') as HTMLElement;
  if (!target) return;

  const action = target.dataset.action;
  if (action === 'toggle-click-through') {
    currentConfig.click_through = !currentConfig.click_through;
    try {
      await invoke('set_click_through', { enabled: currentConfig.click_through });
    } catch {
      // Dev mode fallback
    }
    if (menuCtStatus) menuCtStatus.textContent = currentConfig.click_through ? 'ON' : 'OFF';
  } else if (action === 'cycle-theme') {
    const themes = themeEngine.getAvailableThemes();
    const currentIndex = themes.findIndex((t) => t.id === currentConfig.theme);
    const nextTheme = themes[(currentIndex + 1) % themes.length];
    currentConfig.theme = nextTheme.id;
    themeEngine.setTheme(nextTheme.id);
    themeEngine.render(initialIdleState, currentConfig);
    try {
      await invoke('set_theme', { theme: nextTheme.id });
    } catch {
      // Dev mode fallback
    }
    if (menuThemeName) menuThemeName.textContent = nextTheme.name;
  } else if (action === 'open-themes') {
    try {
      await invoke('open_themes_folder');
    } catch {
      // Dev mode fallback
    }
  }

  contextMenu.classList.remove('open');
});

// Setup Tauri Event Listeners & Initial Config
async function init() {
  try {
    const config = await invoke<AppConfig>('get_config');
    if (config) {
      currentConfig = config;
      themeEngine.setTheme(config.theme);
      themeEngine.render(initialIdleState, currentConfig);
    }

    await listen<AudioState>('audio-update', (event) => {
      handleAudioUpdate(event.payload);
    });

    await listen<AppConfig>('config-update', (event) => {
      currentConfig = event.payload;
      themeEngine.setTheme(currentConfig.theme);
      themeEngine.render(initialIdleState, currentConfig);
    });
  } catch (err) {
    console.warn('Running outside Tauri environment. Starting mock simulation...', err);
    startMockSimulation();
  }
}

// Dev Simulation for Browser Testing
function startMockSimulation() {
  let mockAngle = 0;
  setInterval(() => {
    mockAngle += 0.08;
    const peak = (Math.sin(mockAngle) * 0.5 + 0.5) * (Math.sin(mockAngle * 2.3) * 0.3 + 0.7);

    handleAudioUpdate({
      has_audio: true,
      overall_peak: peak,
      is_playing: true,
      track_title: 'Blinding Lights',
      artist: 'The Weeknd',
      album: 'After Hours',
      thumbnail: null,
      source_app: 'Spotify',
      active_apps: [
        { name: 'Spotify.exe', pid: 14208, peak: peak },
        { name: 'chrome.exe', pid: 8820, peak: peak * 0.2 },
      ],
    });
  }, 33);
}

init();
