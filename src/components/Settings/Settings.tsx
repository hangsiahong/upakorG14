import React, { useState } from 'react';
import {
  Settings as SettingsIcon,
  Save,
  RotateCcw,
  Power,
  Monitor,
  Bell,
  Palette
} from 'lucide-react';
import { useSettingsState } from '../../hooks/useSettingsState';

type TabId = 'general' | 'power' | 'gpu' | 'ui';

export function Settings() {
  const { settings, saveSettings, resetSettings, isLoading } = useSettingsState();
  const [activeTab, setActiveTab] = useState<TabId>('general');
  const [localSettings, setLocalSettings] = useState(settings);

  const handleSave = () => {
    saveSettings.mutate(localSettings);
  };

  const handleReset = () => {
    if (confirm('Are you sure you want to reset all settings to defaults?')) {
      resetSettings.mutate();
    }
  };

  if (isLoading || !localSettings) {
    return <div className="p-8">Loading settings...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="card p-6">
        <h1 className="text-2xl font-bold mb-6">Settings</h1>

        {/* Tabs */}
        <div className="flex gap-2 border-b mb-6">
          <TabButton
            id="general"
            active={activeTab}
            onClick={setActiveTab}
            icon={<SettingsIcon className="w-4 h-4" />}
            label="General"
          />
          <TabButton
            id="power"
            active={activeTab}
            onClick={setActiveTab}
            icon={<Power className="w-4 h-4" />}
            label="Power"
          />
          <TabButton
            id="gpu"
            active={activeTab}
            onClick={setActiveTab}
            icon={<Monitor className="w-4 h-4" />}
            label="GPU"
          />
          <TabButton
            id="ui"
            active={activeTab}
            onClick={setActiveTab}
            icon={<Palette className="w-4 h-4" />}
            label="Appearance"
          />
        </div>

        {/* General Settings */}
        {activeTab === 'general' && (
          <div className="space-y-4">
            <SettingItem
              title="Notifications"
              description="Show system notifications for important events"
            >
              <Bell className="w-5 h-5 text-gray-600" />
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={localSettings.ui.show_notifications}
                  onChange={(e) => setLocalSettings({
                    ...localSettings,
                    ui: { ...localSettings.ui, show_notifications: e.target.checked }
                  })}
                  className="w-4 h-4"
                />
                <span className="text-sm">{localSettings.ui.show_notifications ? 'Enabled' : 'Disabled'}</span>
              </label>
            </SettingItem>

            <SettingItem
              title="Start Minimized"
              description="Start application minimized to system tray"
            >
              <Monitor className="w-5 h-5 text-gray-600" />
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={localSettings.ui.start_minimized}
                  onChange={(e) => setLocalSettings({
                    ...localSettings,
                    ui: { ...localSettings.ui, start_minimized: e.target.checked }
                  })}
                  className="w-4 h-4"
                />
                <span className="text-sm">{localSettings.ui.start_minimized ? 'Yes' : 'No'}</span>
              </label>
            </SettingItem>

            <SettingItem
              title="Auto-start with System"
              description="Launch application on system startup"
            >
              <Power className="w-5 h-5 text-gray-600" />
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={localSettings.ui.auto_start}
                  onChange={(e) => setLocalSettings({
                    ...localSettings,
                    ui: { ...localSettings.ui, auto_start: e.target.checked }
                  })}
                  className="w-4 h-4"
                />
                <span className="text-sm">{localSettings.ui.auto_start ? 'Enabled' : 'Disabled'}</span>
              </label>
            </SettingItem>
          </div>
        )}

        {/* Power Settings */}
        {activeTab === 'power' && (
          <div className="space-y-4">
            <SettingItem
              title="Default Power Profile"
              description="Power profile to apply on startup"
            >
              <Power className="w-5 h-5 text-gray-600" />
              <select
                className="p-2 border rounded-md bg-white min-w-[200px]"
                value={localSettings.power.default_profile}
                onChange={(e) => setLocalSettings({
                  ...localSettings,
                  power: { ...localSettings.power, default_profile: e.target.value as any }
                })}
              >
                <option value="quiet">Quiet</option>
                <option value="balanced">Balanced</option>
                <option value="performance">Performance</option>
              </select>
            </SettingItem>

            <SettingItem
              title="Auto-Switch Profile"
              description="Automatically switch power profiles based on AC/battery"
            >
              <Power className="w-5 h-5 text-gray-600" />
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={localSettings.power.auto_switch_profile}
                  onChange={(e) => setLocalSettings({
                    ...localSettings,
                    power: { ...localSettings.power, auto_switch_profile: e.target.checked }
                  })}
                  className="w-4 h-4"
                />
                <span className="text-sm">{localSettings.power.auto_switch_profile ? 'Enabled' : 'Disabled'}</span>
              </label>
            </SettingItem>

            {localSettings.power.auto_switch_profile && (
              <>
                <div className="ml-8 space-y-2">
                  <SettingItem
                    title="On Battery"
                    description="Profile when running on battery"
                  >
                    <select
                      className="p-2 border rounded-md bg-white"
                      value={localSettings.power.battery_profile}
                      onChange={(e) => setLocalSettings({
                        ...localSettings,
                        power: { ...localSettings.power, battery_profile: e.target.value as any }
                      })}
                    >
                      <option value="quiet">Quiet</option>
                      <option value="balanced">Balanced</option>
                      <option value="performance">Performance</option>
                    </select>
                  </SettingItem>

                  <SettingItem
                    title="Plugged In"
                    description="Profile when connected to AC power"
                  >
                    <select
                      className="p-2 border rounded-md bg-white"
                      value={localSettings.power.ac_profile}
                      onChange={(e) => setLocalSettings({
                        ...localSettings,
                        power: { ...localSettings.power, ac_profile: e.target.value as any }
                      })}
                    >
                      <option value="quiet">Quiet</option>
                      <option value="balanced">Balanced</option>
                      <option value="performance">Performance</option>
                    </select>
                  </SettingItem>
                </div>
              </>
            )}
          </div>
        )}

        {/* GPU Settings */}
        {activeTab === 'gpu' && (
          <div className="space-y-4">
            <SettingItem
              title="Default GPU Mode"
              description="Preferred GPU mode on startup"
            >
              <Monitor className="w-5 h-5 text-gray-600" />
              <select
                className="p-2 border rounded-md bg-white min-w-[200px]"
                value={localSettings.gpu.mode}
                onChange={(e) => setLocalSettings({
                  ...localSettings,
                  gpu: { ...localSettings.gpu, mode: e.target.value as any }
                })}
              >
                <option value="integrated">Integrated</option>
                <option value="hybrid">Hybrid</option>
                <option value="dedicated">Dedicated</option>
              </select>
            </SettingItem>

            <SettingItem
              title="Auto-Switch GPU on Battery"
              description="Switch to integrated GPU when on battery power"
            >
              <Power className="w-5 h-5 text-gray-600" />
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={localSettings.gpu.auto_switch_on_battery}
                  onChange={(e) => setLocalSettings({
                    ...localSettings,
                    gpu: { ...localSettings.gpu, auto_switch_on_battery: e.target.checked }
                  })}
                  className="w-4 h-4"
                />
                <span className="text-sm">{localSettings.gpu.auto_switch_on_battery ? 'Enabled' : 'Disabled'}</span>
              </label>
            </SettingItem>
          </div>
        )}

        {/* Appearance Settings */}
        {activeTab === 'ui' && (
          <div className="space-y-4">
            <SettingItem
              title="Theme"
              description="Application color theme"
            >
              <Palette className="w-5 h-5 text-gray-600" />
              <select
                className="p-2 border rounded-md bg-white min-w-[200px]"
                value={localSettings.ui.theme}
                onChange={(e) => setLocalSettings({
                  ...localSettings,
                  ui: { ...localSettings.ui, theme: e.target.value as any }
                })}
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="system">System</option>
              </select>
            </SettingItem>
          </div>
        )}

        {/* Action Buttons */}
        <div className="flex justify-end gap-3 pt-6 border-t">
          <button
            onClick={handleReset}
            className="flex items-center gap-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
          >
            <RotateCcw className="w-4 h-4" />
            Reset to Defaults
          </button>
          <button
            onClick={handleSave}
            disabled={saveSettings.isPending}
            className="flex items-center gap-2 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors"
          >
            <Save className="w-4 h-4" />
            {saveSettings.isPending ? 'Saving...' : 'Save Settings'}
          </button>
        </div>
      </div>
    </div>
  );
}

function TabButton({
  id,
  active,
  onClick,
  icon,
  label
}: {
  id: TabId;
  active: string;
  onClick: (id: TabId) => void;
  icon: React.ReactNode;
  label: string;
}) {
  return (
    <button
      onClick={() => onClick(id)}
      className={`flex items-center gap-2 px-4 py-2 border-b-2 transition-colors ${
        active === id
          ? 'border-blue-600 text-blue-600'
          : 'border-transparent text-gray-600 hover:text-gray-900'
      }`}
    >
      {icon}
      <span className="font-medium">{label}</span>
    </button>
  );
}

function SettingItem({
  title,
  description,
  children
}: {
  title: string;
  description: string;
  children: React.ReactNode;
}) {
  // Convert children to array for flexible handling
  const childArray = React.Children.toArray(children);

  return (
    <div className="flex items-start gap-4 p-4 bg-gray-50 rounded-lg">
      {childArray.length > 0 && (
        <div className="text-gray-600 mt-1">{childArray[0]}</div>
      )}
      <div className="flex-1">
        <div className="font-semibold text-gray-900">{title}</div>
        <div className="text-sm text-gray-600 mb-3">{description}</div>
        <div className="flex items-center gap-2">
          {childArray.length > 1 && childArray.slice(1)}
        </div>
      </div>
    </div>
  );
}
