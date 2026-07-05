type CatalogSkeletonProps = {
  categoryCount?: number;
  productCount?: number;
  viewMode?: 'list' | 'grid';
};

function SkeletonBlock({ className }: { className: string }) {
  return <div className={`animate-pulse rounded-md bg-surface-muted ${className}`} aria-hidden />;
}

export function CatalogSkeleton({
  categoryCount = 5,
  productCount = 8,
  viewMode = 'grid',
}: CatalogSkeletonProps) {
  return (
    <div className="space-y-4" aria-busy="true" aria-label="Loading catalog">
      <div className="flex gap-2 overflow-hidden">
        {Array.from({ length: categoryCount }, (_, index) => (
          <SkeletonBlock
            key={`category-${String(index)}`}
            className="h-20 w-20 shrink-0 rounded-2xl"
          />
        ))}
      </div>
      <SkeletonBlock className="h-10 w-full max-w-sm" />
      {viewMode === 'grid' ? (
        <div className="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-4 lg:gap-4">
          {Array.from({ length: productCount }, (_, index) => (
            <SkeletonBlock key={`product-${String(index)}`} className="h-56 w-full rounded-lg" />
          ))}
        </div>
      ) : (
        <div className="space-y-3">
          {Array.from({ length: productCount }, (_, index) => (
            <SkeletonBlock key={`product-${String(index)}`} className="h-36 w-full rounded-xl" />
          ))}
        </div>
      )}
    </div>
  );
}
