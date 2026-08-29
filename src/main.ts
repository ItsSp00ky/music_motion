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
  auto_hide_seconds: 3,
  margin_x: 24,
  margin_y: 24,
};

let lastActiveTime = Date.now();
let isIdleHidden = false;

// Apply theme
themeEngine.setTheme(currentConfig.theme);

// Idle check loop
setInterval(() => {
  if (currentConfig.auto_hide_seconds <= 0) return;
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
  }

  themeEngine.render(state, currentConfig);
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
    }

    await listen<AudioState>('audio-update', (event) => {
      handleAudioUpdate(event.payload);
    });

    await listen<AppConfig>('config-update', (event) => {
      currentConfig = event.payload;
      themeEngine.setTheme(currentConfig.theme);
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
      track_title: 'Starboy (feat. Daft Punk)',
      artist: 'The Weeknd',
      album: 'Starboy',
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
