import { useMutation, useQueryClient } from '@tanstack/react-query';
import { tauri } from '../lib/tauri';

export function usePowerProfiles() {
  const queryClient = useQueryClient();

  const setProfile = useMutation({
    mutationFn: tauri.setPowerProfile,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['power-profile'] });
    },
  });

  const setChargeLimit = useMutation({
    mutationFn: tauri.setChargeLimit,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['charge-limit'] });
    },
  });

  return { setProfile, setChargeLimit };
}
