package com.fullsales.seller.shared.catalog

import com.fullsales.seller.shared.api.SellerApiClient
import com.fullsales.seller.shared.model.Commerce
import com.fullsales.seller.shared.model.CommerceAddress
import com.fullsales.seller.shared.repository.CatalogRepository
import com.fullsales.seller.shared.repository.CommerceAddressCache

data class CachedCommerceDetail(
    val commerce: Commerce,
    val addresses: List<CommerceAddress>,
    val fromCache: Boolean,
)

/**
 * Contract (Phase 14D): Room/cache first; fetch when Online.
 */
class CommerceDetailLoader(
    private val catalog: CatalogRepository,
    private val addressCache: CommerceAddressCache,
    private val apiClient: SellerApiClient,
) {
    suspend fun load(commerceId: String, online: Boolean): CachedCommerceDetail {
        val cachedCommerce = catalog.getCommerce(commerceId)
        val cachedAddresses = addressCache.get(commerceId).orEmpty()
        if (!online) {
            val commerce = cachedCommerce ?: error("COMMERCE_NOT_FOUND")
            return CachedCommerceDetail(commerce, cachedAddresses, fromCache = true)
        }
        val remoteCommerce = runCatching { apiClient.getCommerce(commerceId) }.getOrNull()
        val commerce = remoteCommerce ?: cachedCommerce ?: error("COMMERCE_NOT_FOUND")
        val addresses = runCatching { apiClient.listCommerceAddresses(commerceId) }
            .onSuccess { addressCache.put(commerceId, it) }
            .getOrNull()
            ?: cachedAddresses
        return CachedCommerceDetail(commerce, addresses, fromCache = remoteCommerce == null)
    }
}
