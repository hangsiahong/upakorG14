interface MetricCardProps {
  title: string;
  value: string | number;
  unit?: string;
  icon?: React.ReactNode;
}

export function MetricCard({ title, value, unit, icon }: MetricCardProps) {
  return (
    <div className="card p-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-gray-500">{title}</p>
          <p className="text-2xl font-bold">
            {value}{unit && <span className="text-lg text-gray-500 ml-1">{unit}</span>}
          </p>
        </div>
        {icon && <div className="text-adwaita-accent">{icon}</div>}
      </div>
    </div>
  );
}
