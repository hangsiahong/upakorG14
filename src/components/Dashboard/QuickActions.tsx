import { Cpu, Zap, Check } from 'lucide-react';
import { useHardwareState } from '../../hooks/useHardwareState';
import { usePowerProfiles } from '../../hooks/usePowerProfiles';

export function QuickActions() {
  const { profile, chargeLimit } = useHardwareState();
  const { setProfile, setChargeLimit } = usePowerProfiles();

  const profiles = [
    { id: 'quiet', label: 'Quiet', description: 'Maximum efficiency, minimal noise' },
    { id: 'balanced', label: 'Balanced', description: 'Optimized for daily use' },
    { id: 'performance', label: 'Performance', description: 'Maximum power output' },
  ];

  return (
    <div className="card p-6">
      <div className="section-header">
        <div>
          <div className="label">Quick Actions</div>
          <h3 className="font-display text-lg font-bold">SYSTEM CONTROLS</h3>
        </div>
      </div>

      <div className="space-y-6">
        {/* Power Profile Selector */}
        <div>
          <label className="flex items-center gap-2 mb-3">
            <Cpu className="w-4 h-4 text-upakor-accent" />
            <span className="label mb-0">Power Profile</span>
          </label>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
            {profiles.map((p) => {
              const isActive = profile === p.id;
              return (
                <button
                  key={p.id}
                  onClick={() => setProfile.mutate(p.id)}
                  disabled={setProfile.isPending}
                  className={`
                    relative p-4 border-2 transition-all duration-200 snappy
                    font-display text-sm font-medium tracking-wide text-left
                    ${isActive
                      ? 'border-upakor-accent bg-upakor-accent/5 shadow-technical'
                      : 'border-upakor-border bg-upakor-bg hover:border-upakor-accent/50 hover:shadow-technical'
                    }
                  `}
                >
                  {isActive && (
                    <div className="absolute top-2 right-2">
                      <Check className="w-4 h-4 text-upakor-accent" />
                    </div>
                  )}
                  <div className="font-bold mb-1">{p.label}</div>
                  <div className="text-xs text-upakor-fg-muted font-normal">
                    {p.description}
                  </div>
                </button>
              );
            })}
          </div>
        </div>

        {/* Charge Limit Slider */}
        <div className="pt-4 border-t border-upakor-border">
          <label className="flex items-center gap-2 mb-3">
            <Zap className="w-4 h-4 text-upakor-accent" />
            <span className="label mb-0">Charge Limit</span>
            <span className="ml-auto font-display text-sm font-bold text-upakor-accent">
              {chargeLimit?.limit || 100}%
            </span>
          </label>

          <div className="space-y-3">
            <input
              type="range"
              min="50"
              max="100"
              step="10"
              value={chargeLimit?.limit || 100}
              onChange={(e) => setChargeLimit.mutate(Number(e.target.value))}
              disabled={setChargeLimit.isPending}
              className="slider w-full"
            />

            <div className="flex justify-between font-display text-xs text-upakor-fg-muted uppercase tracking-wider">
              <span>50%</span>
              <span>60%</span>
              <span>70%</span>
              <span>80%</span>
              <span>90%</span>
              <span>100%</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
