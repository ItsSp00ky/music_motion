export interface ProcessAudioInfo {
  name: string;
  pid: number;
  peak: number;
}

export interface AudioState {
  has_audio: boolean;
  overall_peak: number;
  is_playing: boolean;
  track_title: string;
  artist: string;
  album: string;
  thumbnail: string | null;
  source_app: string;
  active_apps: ProcessAudioInfo[];
}

export interface AppConfig {
  position: 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left';
  click_through: boolean;
  theme: string;
  sensitivity: number;
  auto_hide_seconds: number;
  margin_x: number;
  margin_y: number;
}

export interface ThemeDefinition {
  id: string;
  name: string;
  description: string;
  render: (container: HTMLElement, state: AudioState, config: AppConfig) => void;
  init?: (container: HTMLElement) => void;
  destroy?: () => void;
}
