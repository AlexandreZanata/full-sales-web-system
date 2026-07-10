import { cn } from '@/lib/utils';

type TabItem = {
  id: string;
  label: string;
};

type TabsProps = {
  tabs: TabItem[];
  activeId: string;
  onChange: (id: string) => void;
  children: React.ReactNode;
};

export function Tabs({ tabs, activeId, onChange, children }: TabsProps) {
  return (
    <div>
      <div className="flex gap-1 border-b border-hairline">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            type="button"
            onClick={() => {
              onChange(tab.id);
            }}
            className={cn(
              'min-h-10 px-4 py-2 text-sm font-medium transition-colors',
              activeId === tab.id
                ? 'border-b-2 border-primary text-foreground'
                : 'text-muted-foreground hover:text-foreground',
            )}
          >
            {tab.label}
          </button>
        ))}
      </div>
      <div className="pt-4">{children}</div>
    </div>
  );
}
