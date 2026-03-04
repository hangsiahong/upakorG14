import {
  LayoutDashboard,
  Cpu,
  Zap,
  Monitor,
  Settings,
  Laptop,
  Activity
} from 'lucide-react';

interface SidebarProps {
  currentPage: string;
  onPageChange: (page: string) => void;
}

export function Sidebar({ currentPage, onPageChange }: SidebarProps) {
  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDesktop },
    { id: 'power', label: 'Power', icon: Cpu },
    { id: 'gpu', label: 'GPU', icon: Monitor },
    { id: 'battery', label: 'Battery', icon: Zap },
    { id: 'settings', label: 'Settings', icon: Settings },
  ];

  return (
    <aside className="w-72 bg-upakor-bg-alt border-r border-upakor-border p-6 min-h-screen flex flex-col relative">
      {/* Technical marker line */}
      <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-upakor-accent" />

      {/* Header */}
      <div className="mb-10 pl-4 animate-fade-in">
        <div className="flex items-center gap-4 mb-3">
          <div className="w-12 h-12 bg-upakor-accent/10 border-2 border-upakor-accent flex items-center justify-center">
            <Laptop className="w-6 h-6 text-upakor-accent" />
          </div>
          <div>
            <h1 className="font-display font-bold text-lg tracking-tight">
              UPAKOR<span className="text-upakor-accent">.G14</span>
            </h1>
            <p className="font-display text-xs uppercase tracking-wider text-upakor-fg-muted mt-0.5">
              ASUS Control Unit
            </p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 pl-4 animate-slide-in" style={{ animationDelay: '100ms' }}>
        <div className="label">Navigation</div>
        {menuItems.map((item, index) => {
          const Icon = item.icon;
          const isActive = currentPage === item.id;

          return (
            <button
              key={item.id}
              onClick={() => onPageChange(item.id)}
              className={`
                w-full flex items-center gap-3 px-4 py-3 mb-1
                font-display text-sm font-medium tracking-wide
                border transition-all duration-200 snappy
                animate-scale-in
                ${isActive
                  ? 'bg-upakor-accent text-white border-upakor-accent shadow-technical translate-x-1'
                  : 'bg-upakor-bg text-upakor-fg border-upakor-border hover:border-upakor-accent hover:shadow-technical hover:translate-x-0.5'
                }
              `}
              style={{ animationDelay: `${200 + index * 50}ms` }}
            >
              <Icon className="w-4 h-4 flex-shrink-0" />
              <span className="uppercase tracking-wider">{item.label}</span>
              {isActive && (
                <Activity className="w-3 h-3 ml-auto animate-pulse" />
              )}
            </button>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="mt-auto pt-6 border-t border-upakor-border pl-4 animate-fade-in" style={{ animationDelay: '500ms' }}>
        <div className="flex items-center gap-2 mb-2">
          <div className="status-dot active" />
          <span className="font-display text-xs text-upakor-fg-muted uppercase tracking-wider">
            System Active
          </span>
        </div>
        <div className="font-display text-xs text-upakor-fg-muted/60">
          <p>v0.1.0 // BUILD 2025.03</p>
          <p className="font-mono mt-1">MOCK_MODE_ENABLED</p>
        </div>
      </div>
    </aside>
  );
}

// Add LayoutDesktop icon mapping
function LayoutDesktop(props: React.SVGProps<SVGSVGElement>) {
  return <LayoutDashboard {...props} />;
}
