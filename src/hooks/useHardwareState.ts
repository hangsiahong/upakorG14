import { useQuery } from '@tanstack/react-query';
import { tauri } from '../lib/tauri';

export function useHardwareState() {
  const metrics = useQuery({
    queryKey: ['hardware-metrics'],
    queryFn: tauri.getHardwareMetrics,
  });

  const profile = useQuery({
    queryKey: ['power-profile'],
    queryFn: tauri.getPowerProfile,
  });

  const chargeLimit = useQuery({
    queryKey: ['charge-limit'],
    queryFn: tauri.getChargeLimit,
  });

  return {
    metrics: metrics.data,
    profile: profile.data,
    chargeLimit: chargeLimit.data,
    isLoading: metrics.isLoading || profile.isLoading || chargeLimit.isLoading,
    error: metrics.error || profile.error || chargeLimit.error,
  };
}
