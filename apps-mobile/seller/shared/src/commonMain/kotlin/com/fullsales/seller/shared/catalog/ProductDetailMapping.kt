package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.model.Product
import com.fullsales.seller.shared.model.ProductDetail

fun Product.toDetail(): ProductDetail = ProductDetail(
    id = id,
    name = name,
    sku = sku,
    priceAmount = priceAmount,
    priceCurrency = priceCurrency,
    active = active,
    categoryId = categoryId,
    categoryName = categoryName,
    categorySlug = categorySlug,
    primaryImageUrl = primaryImageUrl,
    primaryImageFileId = primaryImageFileId,
)
