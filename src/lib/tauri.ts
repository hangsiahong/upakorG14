import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export const tauri = {
  // Power profiles
  getPowerProfile: () => invoke<string>('get_power_profile'),
  setPowerProfile: (profile: string) => invoke<void>('set_power_profile', { profile }),

  // Charge limit
  getChargeLimit: () => invoke<{ limit: number }>('get_charge_limit'),
  setChargeLimit: (limit: number) => invoke<void>('set_charge_limit', { limit }),

  // Hardware monitoring
  getHardwareMetrics: () => invoke<any>('get_hardware_metrics'),

  // GPU mode
  getGpuStatus: () => invoke<any>('get_gpu_status'),
  setGpuMode: (mode: string) => invoke<void>('set_gpu_mode', { mode }),

  // Settings
  getSettings: () => invoke<any>('get_settings'),
  saveSettings: (settings: any) => invoke<void>('save_settings', { settings }),
  resetSettings: () => invoke<any>('reset_settings'),
  getConfigPath: () => invoke<string>('get_config_path'),

  // Autostart
  getAutostartStatus: () => invoke<boolean>('get_autostart_status'),
  setAutostart: (enabled: boolean) => invoke<void>('set_autostart', { enabled }),

  // Hardware capabilities
  getHardwareCapabilities: () => invoke<any>('get_hardware_capabilities'),

  // Metrics polling
  startMetricsPolling: (intervalMs: number) => invoke<void>('start_metrics_polling', { intervalMs }),
  stopMetricsPolling: () => invoke<void>('stop_metrics_polling'),
  isPollingMetrics: () => invoke<boolean>('is_polling_metrics'),
};

// Event listeners
export const events = {
  onHardwareMetricsUpdate: (callback: (metrics: any) => void) => {
    return listen<any>('hardware_metrics_update', (event) => callback(event.payload));
  },
  onMetricsPollingError: (callback: (error: string) => void) => {
    return listen<string>('metrics_polling_error', (event) => callback(event.payload));
  },
};
