import { MetricCard } from './MetricCard';
import { QuickActions } from './QuickActions';
import { useHardwareState } from '../../hooks/useHardwareState';
import { Thermometer, HardDrive, Gauge, Cpu, Zap } from 'lucide-react';

export function Dashboard() {
  const { metrics, isLoading, error } = useHardwareState();

  if (isLoading) {
    return (
      <div className="p-8 flex items-center justify-center">
        <div className="flex items-center gap-3 font-display text-sm text-upakor-fg-muted">
          <div className="w-4 h-4 border-2 border-upakor-accent border-t-transparent animate-spin" />
          <span className="uppercase tracking-wider">Loading telemetry...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-8 card border-upakor-danger/50">
        <div className="flex items-center gap-3 text-upakor-danger">
          <div className="w-2 h-2 bg-upakor-danger rounded-full animate-pulse" />
          <span className="font-display text-sm font-medium uppercase tracking-wider">
            Error: {error.message}
          </span>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="section-header animate-fade-in">
        <div>
          <h2 className="font-display text-2xl font-bold tracking-tight">
            SYSTEM DASHBOARD
          </h2>
          <p className="font-body text-sm text-upakor-fg-muted mt-1">
            Real-time hardware monitoring and control
          </p>
        </div>
      </div>

      {/* Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 animate-slide-in" style={{ animationDelay: '100ms' }}>
        <MetricCard
          title="CPU Temperature"
          value={metrics?.temperatures.cpu.toFixed(1) || '0'}
          unit="°C"
          icon={<Thermometer className="w-5 h-5" />}
          delay={0}
        />
        <MetricCard
          title="Total Power"
          value={metrics?.power_draw.total.toFixed(1) || '0'}
          unit="W"
          icon={<Gauge className="w-5 h-5" />}
          delay={100}
        />
        <MetricCard
          title="CPU Fan"
          value={metrics?.fan_speeds.cpu_percentage || '0'}
          unit="%"
          icon={<HardDrive className="w-5 h-5" />}
          delay={200}
        />
        <MetricCard
          title="GPU Temperature"
          value={metrics?.temperatures.gpu?.toFixed(1) || 'N/A'}
          unit="°C"
          icon={<Cpu className="w-5 h-5" />}
          delay={300}
        />
      </div>

      {/* Quick Actions */}
      <div className="animate-slide-in" style={{ animationDelay: '200ms' }}>
        <QuickActions />
      </div>

      {/* Additional Metrics Section */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 animate-slide-in" style={{ animationDelay: '300ms' }}>
        {/* Power Usage */}
        <div className="card p-6">
          <div className="flex items-center justify-between mb-6">
            <div>
              <div className="label">Power Distribution</div>
              <h3 className="font-display text-lg font-bold">ENERGY CONSUMPTION</h3>
            </div>
            <Zap className="w-5 h-5 text-upakor-accent" />
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between py-2 border-b border-upakor-border">
              <span className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted">CPU Package</span>
              <span className="font-display text-sm font-bold">
                {metrics?.power_draw.cpu?.toFixed(1) || '0'}W
              </span>
            </div>
            <div className="flex items-center justify-between py-2 border-b border-upakor-border">
              <span className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted">GPU Power</span>
              <span className="font-display text-sm font-bold">
                {metrics?.power_draw.gpu?.toFixed(1) || '0'}W
              </span>
            </div>
            <div className="flex items-center justify-between py-2">
              <span className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted">Total</span>
              <span className="font-display text-sm font-bold text-upakor-accent">
                {metrics?.power_draw.total?.toFixed(1) || '0'}W
              </span>
            </div>
          </div>
        </div>

        {/* Fan Speeds */}
        <div className="card p-6">
          <div className="flex items-center justify-between mb-6">
            <div>
              <div className="label">Fan Status</div>
              <h3 className="font-display text-lg font-bold">THERMAL MANAGEMENT</h3>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-upakor-accent animate-pulse" />
            </div>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-between py-2 border-b border-upakor-border">
              <span className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted">CPU Fan</span>
              <span className="font-display text-sm font-bold">
                {metrics?.fan_speeds.cpu_percentage || '0'}%
              </span>
            </div>
            <div className="flex items-center justify-between py-2 border-b border-upakor-border">
              <span className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted">GPU Fan</span>
              <span className="font-display text-sm font-bold">
                {metrics?.fan_speeds.gpu_percentage || '0'}%
              </span>
            </div>
            <div className="flex items-center justify-between py-2">
              <span className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted">Mid Fan</span>
              <span className="font-display text-sm font-bold">
                {metrics?.fan_speeds.mid_percentage || '0'}%
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
