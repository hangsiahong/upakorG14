import { Monitor, Cpu, Zap } from 'lucide-react';
import { useGpuState } from '../../hooks/useGpuState';
import { useState } from 'react';

export function GpuControl() {
  const { status, setMode, isLoading } = useGpuState();
  const [selectedMode, setSelectedMode] = useState(status?.current_mode || 'hybrid');

  const modes = [
    { id: 'integrated', label: 'Integrated', icon: Cpu, description: 'Best battery life, lower performance' },
    { id: 'hybrid', label: 'Hybrid', icon: Zap, description: 'Balanced performance and battery' },
    { id: 'dedicated', label: 'Dedicated', icon: Monitor, description: 'Maximum performance' },
  ];

  const handleModeChange = (mode: string) => {
    setSelectedMode(mode);
    setMode.mutate(mode);
  };

  return (
    <div className="space-y-6">
      <div className="card p-6">
        <h2 className="text-xl font-semibold mb-4">GPU Mode</h2>

        {isLoading ? (
          <div className="text-center py-8 text-gray-500">Loading GPU status...</div>
        ) : (
          <>
            <div className="mb-4 p-4 bg-blue-50 border border-blue-200 rounded-lg">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium text-blue-900">Current Mode</span>
                <span className="text-sm font-bold text-blue-700 capitalize">
                  {status?.current_mode || 'Unknown'}
                </span>
              </div>
              {status?.dedicated_available && (
                <div className="text-xs text-blue-600 mt-2">
                  Dedicated GPU: Available
                </div>
              )}
            </div>

            <div className="space-y-3">
              {modes.map((mode) => {
                const Icon = mode.icon;
                const isSelected = selectedMode === mode.id;

                return (
                  <button
                    key={mode.id}
                    onClick={() => handleModeChange(mode.id)}
                    disabled={!status?.dedicated_available && mode.id === 'dedicated'}
                    className={`w-full p-4 rounded-lg border-2 text-left transition-all ${
                      isSelected
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                    } ${
                      !status?.dedicated_available && mode.id === 'dedicated'
                        ? 'opacity-50 cursor-not-allowed'
                        : 'cursor-pointer'
                    }`}
                  >
                    <div className="flex items-start gap-3">
                      <Icon className={`w-6 h-6 mt-1 ${isSelected ? 'text-blue-600' : 'text-gray-600'}`} />
                      <div className="flex-1">
                        <div className="font-semibold text-gray-900">{mode.label}</div>
                        <div className="text-sm text-gray-600">{mode.description}</div>
                      </div>
                      {isSelected && (
                        <div className="w-3 h-3 bg-blue-500 rounded-full mt-2" />
                      )}
                    </div>
                  </button>
                );
              })}
            </div>

            {!status?.dedicated_available && (
              <div className="mt-4 p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                <p className="text-sm text-yellow-800">
                  ⚠️ Dedicated GPU not available on this system
                </p>
              </div>
            )}
          </>
        )}
      </div>

      <div className="card p-6">
        <h3 className="text-lg font-semibold mb-3">GPU Mode Guide</h3>
        <div className="space-y-3 text-sm">
          <div className="flex items-start gap-2">
            <span className="font-semibold text-gray-700 min-w-fit">Integrated:</span>
            <span className="text-gray-600">Best for battery life, suitable for basic tasks</span>
          </div>
          <div className="flex items-start gap-2">
            <span className="font-semibold text-gray-700 min-w-fit">Hybrid:</span>
            <span className="text-gray-600">Automatically switches between GPUs for optimal performance</span>
          </div>
          <div className="flex items-start gap-2">
            <span className="font-semibold text-gray-700 min-w-fit">Dedicated:</span>
            <span className="text-gray-600">Maximum performance for gaming and intensive tasks</span>
          </div>
        </div>
        <div className="mt-4 p-3 bg-gray-50 border border-gray-200 rounded-lg">
          <p className="text-xs text-gray-600">
            ⚠️ GPU mode switching may require a restart or take a few seconds to apply
          </p>
        </div>
      </div>
    </div>
  );
}
