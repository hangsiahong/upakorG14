import { MetricCard } from './MetricCard';
import { QuickActions } from './QuickActions';
import { useHardwareState } from '../../hooks/useHardwareState';
import { Thermometer, HardDrive, Gauge } from 'lucide-react';

export function Dashboard() {
  const { metrics, isLoading, error } = useHardwareState();

  if (isLoading) {
    return <div className="p-8">Loading...</div>;
  }

  if (error) {
    return <div className="p-8 text-adwaita-error">Error: {error.message}</div>;
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <MetricCard
          title="CPU Temperature"
          value={metrics?.temperatures.cpu.toFixed(1) || '0'}
          unit="°C"
          icon={<Thermometer className="w-6 h-6" />}
        />
        <MetricCard
          title="Total Power"
          value={metrics?.power_draw.total.toFixed(1) || '0'}
          unit="W"
          icon={<Gauge className="w-6 h-6" />}
        />
        <MetricCard
          title="CPU Fan"
          value={metrics?.fan_speeds.cpu_percentage || '0'}
          unit="%"
          icon={<HardDrive className="w-6 h-6" />}
        />
      </div>

      <QuickActions />
    </div>
  );
}
