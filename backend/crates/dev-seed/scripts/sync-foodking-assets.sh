#!/usr/bin/env bash
# Copy FoodKing demo catalog images into dev-seed bundled assets.
# Source: foodking/web/public/images/seeder (FoodKing reference theme).

set -euo pipefail

FK="${FOODKING_SEEDER_DIR:-/data/dev/projects/webstorm/foodking/web/public/images/seeder}"
ROOT="$(cd "$(dirname "$0")/../assets" && pwd)"

if [[ ! -d "$FK" ]]; then
  echo "FoodKing seeder dir not found: $FK" >&2
  exit 1
fi

mkdir -p "$ROOT/products" "$ROOT/categories" "$ROOT/banners" "$ROOT/promotions"

cp "$FK/item/soda_(bottle).png"           "$ROOT/products/coca-cola-2l.png"
cp "$FK/item/soda_(can).png"              "$ROOT/products/guarana-2l.png"
cp "$FK/item/iced_coffee.png"             "$ROOT/products/agua-mineral-500ml.png"
cp "$FK/item/homemade_lemonade.png"       "$ROOT/products/suco-laranja-1l.png"
cp "$FK/item/french_fries.png"            "$ROOT/products/batata-chips.png"
cp "$FK/item/onion_rings.png"             "$ROOT/products/amendoim.png"
cp "$FK/item/cappuccino.png"              "$ROOT/products/chocolate.png"
cp "$FK/item/mojito.png"                  "$ROOT/products/detergente.png"
cp "$FK/item/espresso.png"                "$ROOT/products/desinfetante.png"
cp "$FK/item/whopper.png"                 "$ROOT/products/pizza-congelada.png"
cp "$FK/item/homemade_mashed_potato.png"  "$ROOT/products/arroz-1kg.png"
cp "$FK/item/baked_potato.png"            "$ROOT/products/feijao-1kg.png"

cp "$FK/category/beverages.png"              "$ROOT/categories/bebidas.png"
cp "$FK/category/side-orders.png"            "$ROOT/categories/snacks.png"
cp "$FK/category/house-special-salads.png"     "$ROOT/categories/limpeza.png"
cp "$FK/category/flame-grill-burgers.png"      "$ROOT/categories/congelados.png"
cp "$FK/category/appetizers.png"               "$ROOT/categories/mercearia.png"

cp "$FK/slider/slider_one.png"               "$ROOT/banners/hero-burger.png"
cp "$FK/slider/slider_two.png"               "$ROOT/banners/hero-fresh-food.png"
cp "$FK/slider/slider_three.png"             "$ROOT/banners/hero-breakfast.png"

cp "$FK/offer/savory_and_satisfying.png"     "$ROOT/promotions/promo-burger.png"
cp "$FK/offer/uplifting_anytime.png"         "$ROOT/promotions/promo-drinks.png"

echo "Synced FoodKing assets → $ROOT"
