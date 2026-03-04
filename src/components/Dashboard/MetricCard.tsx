interface MetricCardProps {
  title: string;
  value: string | number;
  unit?: string;
  icon?: React.ReactNode;
  delay?: number;
}

export function MetricCard({ title, value, unit, icon, delay = 0 }: MetricCardProps) {
  return (
    <div
      className="card p-5 transition-all duration-200 snappy hover:shadow-technical-hover animate-scale-in"
      style={{ animationDelay: `${delay}ms` }}
    >
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <div className="label mb-2">{title}</div>
          <div className="flex items-baseline gap-1">
            <span className="font-display text-3xl font-bold tracking-tight">
              {value}
            </span>
            {unit && (
              <span className="font-display text-sm font-medium text-upakor-fg-muted uppercase">
                {unit}
              </span>
            )}
          </div>
        </div>
        {icon && (
          <div className="w-10 h-10 bg-upakor-accent/10 border border-upakor-accent/20 flex items-center justify-center flex-shrink-0">
            <div className="text-upakor-accent">
              {icon}
            </div>
          </div>
        )}
      </div>

      {/* Technical decoration line */}
      <div className="h-0.5 bg-gradient-to-r from-upakor-accent/50 to-transparent" />
    </div>
  );
}
