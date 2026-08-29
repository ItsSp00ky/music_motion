import { AppConfig, AudioState, ThemeDefinition } from '../types';
import { FrostedCardTheme } from './frosted-card';
import { MinimalHudTheme } from './minimal-hud';
import { DynamicIslandTheme } from './dynamic-island';
import { CyberNeonTheme } from './cyber-neon';

export class ThemeEngine {
  private themes: Map<string, ThemeDefinition> = new Map();
  private currentTheme: ThemeDefinition | null = null;
  private container: HTMLElement;

  constructor(container: HTMLElement) {
    this.container = container;

    // Register built-in themes
    this.registerTheme(new FrostedCardTheme());
    this.registerTheme(new MinimalHudTheme());
    this.registerTheme(new DynamicIslandTheme());
    this.registerTheme(new CyberNeonTheme());
  }

  public registerTheme(theme: ThemeDefinition) {
    this.themes.set(theme.id, theme);
  }

  public getAvailableThemes(): { id: string; name: string; description: string }[] {
    return Array.from(this.themes.values()).map((t) => ({
      id: t.id,
      name: t.name,
      description: t.description,
    }));
  }

  public setTheme(themeId: string) {
    if (this.currentTheme && this.currentTheme.id === themeId) {
      return;
    }

    if (this.currentTheme && this.currentTheme.destroy) {
      this.currentTheme.destroy();
    }

    const nextTheme = this.themes.get(themeId) || this.themes.get('frosted-card')!;
    this.currentTheme = nextTheme;

    if (this.currentTheme.init) {
      this.currentTheme.init(this.container);
    }
  }

  public render(state: AudioState, config: AppConfig) {
    if (!this.currentTheme) {
      this.setTheme(config.theme || 'frosted-card');
    }

    if (this.currentTheme) {
      this.currentTheme.render(this.container, state, config);
    }
  }
}
