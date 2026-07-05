export function ProductDetailSkeleton() {
  return (
    <div className="animate-pulse space-y-6">
      <div className="h-4 w-48 rounded bg-surface-muted" />
      <div className="grid gap-8 lg:grid-cols-2">
        <div className="aspect-square rounded-lg bg-surface-muted lg:sticky lg:top-20" />
        <div className="space-y-4">
          <div className="h-6 w-24 rounded-full bg-surface-muted" />
          <div className="h-8 w-3/4 rounded bg-surface-muted" />
          <div className="h-5 w-1/3 rounded bg-surface-muted" />
          <div className="h-24 rounded-lg bg-surface-muted" />
          <div className="space-y-2 rounded-lg border border-hairline p-4">
            <div className="h-4 w-full rounded bg-surface-muted" />
            <div className="h-4 w-5/6 rounded bg-surface-muted" />
            <div className="h-4 w-2/3 rounded bg-surface-muted" />
          </div>
        </div>
      </div>
    </div>
  );
}
