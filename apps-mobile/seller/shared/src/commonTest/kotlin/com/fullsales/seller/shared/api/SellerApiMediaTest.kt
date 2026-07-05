package com.fullsales.seller.shared.api

import io.ktor.http.HttpStatusCode
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.test.runTest

class SellerApiMediaTest {
    @Test
    fun uploadMedia_postsToUploadPath() = runTest {
        val recorder = RecordedMockEngine { request ->
            assertEquals("/v1/media/upload", request.url.encodedPath)
            HttpStatusCode.OK to """
                {"id":"file-1","entityType":"Product","entityId":"p1","mimeType":"image/jpeg","sizeBytes":128,"sha256":"abc"}
            """.trimIndent()
        }
        val client = testClient(engine = recorder.engine())
        val response = client.uploadMedia(
            fileBytes = byteArrayOf(1, 2, 3),
            fileName = "photo.jpg",
            mimeType = "image/jpeg",
            entityType = "Product",
            entityId = "p1",
        )
        assertEquals("file-1", response.id)
        assertEquals("Product", response.entityType)
    }
}
