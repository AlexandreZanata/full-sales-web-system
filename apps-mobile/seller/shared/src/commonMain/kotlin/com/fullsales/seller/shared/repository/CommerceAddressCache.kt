package com.fullsales.seller.shared.repository

import com.fullsales.seller.shared.model.CommerceAddress

interface CommerceAddressCache {
    suspend fun get(commerceId: String): List<CommerceAddress>?
    suspend fun put(commerceId: String, addresses: List<CommerceAddress>)
}
