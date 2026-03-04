import { useMutation, useQuery } from '@tanstack/react-query';
import { tauri } from '../lib/tauri';

export function useSettingsState() {
  const settings = useQuery({
    queryKey: ['settings'],
    queryFn: tauri.getSettings,
  });

  const capabilities = useQuery({
    queryKey: ['capabilities'],
    queryFn: tauri.getHardwareCapabilities,
  });

  const saveSettings = useMutation({
    mutationFn: (newSettings: any) => tauri.saveSettings(newSettings),
    onSuccess: () => {
      settings.refetch();
    },
  });

  const resetSettings = useMutation({
    mutationFn: () => tauri.resetSettings(),
    onSuccess: () => {
      settings.refetch();
    },
  });

  return {
    settings: settings.data,
    capabilities: capabilities.data,
    saveSettings,
    resetSettings,
    isLoading: settings.isLoading,
    isSaving: saveSettings.isPending,
    isResetting: resetSettings.isPending,
  };
}
