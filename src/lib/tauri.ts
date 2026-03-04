import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export const tauri = {
  // Power profiles
  getPowerProfile: () => invoke<string>('get_power_profile'),
  setPowerProfile: (profile: string) => invoke<void>('set_power_profile', { profile }),

  // Charge limit
  getChargeLimit: () => invoke<number>('get_charge_limit'),
  setChargeLimit: (limit: number) => invoke<void>('set_charge_limit', { limit }),

  // Hardware monitoring
  getHardwareMetrics: () => invoke<any>('get_hardware_metrics'),
};
