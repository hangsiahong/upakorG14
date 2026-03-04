import { useMutation, useQuery } from '@tanstack/react-query';
import { tauri } from '../lib/tauri';

export function useGpuState() {
  const status = useQuery({
    queryKey: ['gpu-status'],
    queryFn: tauri.getGpuStatus,
  });

  const setMode = useMutation({
    mutationFn: (mode: string) => tauri.setGpuMode(mode),
    onSuccess: () => {
      // Refetch GPU status after setting mode
      status.refetch();
    },
  });

  return {
    status: status.data,
    setMode,
    isLoading: status.isLoading,
    isChanging: setMode.isPending,
    error: status.error || setMode.error,
  };
}
