import {
  LayoutDashboard,
  Cpu,
  Zap,
  Monitor,
  Settings,
  ChevronRight,
  Laptop
} from 'lucide-react';

interface SidebarProps {
  currentPage: string;
  onPageChange: (page: string) => void;
}

export function Sidebar({ currentPage, onPageChange }: SidebarProps) {
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { id: 'power', label: 'Power Profiles', icon: Cpu },
    { id: 'gpu', label: 'GPU Control', icon: Monitor },
    { id: 'battery', label: 'Battery', icon: Zap },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  return (
    <div className="w-64 bg-gray-900 text-white p-4 min-h-screen">
      <div className="flex items-center gap-3 mb-8 pb-4 border-b border-gray-700">
        <Laptop className="w-8 h-8 text-blue-400" />
        <div>
          <h1 className="text-lg font-bold">Upakor-G14</h1>
          <p className="text-xs text-gray-400">ASUS Laptop Control</p>
        </div>
      </div>

      <nav className="space-y-2">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const isActive = currentPage === item.id;

          return (
            <button
              key={item.id}
              onClick={() => onPageChange(item.id)}
              className={`w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-all ${
                isActive
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-300 hover:bg-gray-800'
              }`}
            >
              <Icon className="w-5 h-5" />
              <span className="font-medium">{item.label}</span>
              {isActive && <ChevronRight className="w-4 h-4 ml-auto" />}
            </button>
          );
        })}
      </nav>

      <div className="mt-auto pt-4 border-t border-gray-700">
        <div className="text-xs text-gray-500">
          <p>Version 0.1.0</p>
          <p>Mock Mode</p>
        </div>
      </div>
    </div>
  );
}
