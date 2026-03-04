import { Cpu, Zap } from 'lucide-react';
import { useHardwareState } from '../../hooks/useHardwareState';
import { usePowerProfiles } from '../../hooks/usePowerProfiles';

export function QuickActions() {
  const { profile, chargeLimit } = useHardwareState();
  const { setProfile, setChargeLimit } = usePowerProfiles();

  return (
    <div className="card p-6">
      <h2 className="text-xl font-semibold mb-4">Quick Actions</h2>

      <div className="space-y-4">
        <div>
          <label className="flex items-center gap-2 text-sm font-medium mb-2">
            <Cpu className="w-4 h-4" />
            Power Profile
          </label>
          <select
            className="w-full p-2 border rounded-md bg-white"
            value={profile || 'balanced'}
            onChange={(e) => setProfile.mutate(e.target.value)}
          >
            <option value="quiet">Quiet</option>
            <option value="balanced">Balanced</option>
            <option value="performance">Performance</option>
          </select>
        </div>

        <div>
          <label className="flex items-center gap-2 text-sm font-medium mb-2">
            <Zap className="w-4 h-4" />
            Charge Limit: {chargeLimit?.limit || 100}%
          </label>
          <input
            type="range"
            min="50"
            max="100"
            step="10"
            value={chargeLimit?.limit || 100}
            onChange={(e) => setChargeLimit.mutate(Number(e.target.value))}
            className="w-full"
          />
        </div>
      </div>
    </div>
  );
}
