# Dev seed demo assets

FoodKing-themed PNG photos synced from `foodking/web/public/images/seeder`.

| Folder | Count | Source (FoodKing) |
|--------|-------|-------------------|
| `products/` | 12 | `seeder/item/*.png` |
| `categories/` | 5 | `seeder/category/*.png` |
| `banners/` | 3 | `seeder/slider/slider_*.png` |
| `promotions/` | 2 | `seeder/offer/*.png` |
| `tenants/` | 1 | `site-logo.png` — default portal header logo |

Refresh bundled files:

```bash
./backend/crates/dev-seed/scripts/sync-foodking-assets.sh
pnpm seed:dev
```

Default object storage: `MEDIA_LOCAL_PATH` or `backend/.local/object-storage`.
