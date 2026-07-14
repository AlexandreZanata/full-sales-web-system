package com.fullsales.seller.shared.db.sqldelight

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.fullsales.seller.shared.model.CreateSaleItem
import com.fullsales.seller.shared.model.CreateSaleRequest
import com.fullsales.seller.shared.model.LocalSaleStatus
import com.fullsales.seller.shared.model.SyncEntityType
import com.fullsales.seller.shared.model.SyncOutboxEntry
import com.fullsales.seller.shared.model.currentEpochMs
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import java.io.File

/**
 * T-16G: GIVEN createLocalSale + enqueue outbox
 * WHEN SQLite file is closed and reopened
 * THEN sale and outbox rows survive.
 */
class SqlDelightLocalStorePersistenceTest {

    @Test
    fun given_localSaleAndOutbox_when_reopenSqlite_then_rowsSurvive() = runTest {
        val path = File.createTempFile("seller-sqldelight-", ".db").absolutePath
        try {
            val written = JdbcSqliteDriver("jdbc:sqlite:$path").use { driver ->
                createSellerLocalSchema(driver)
                val db = createSellerLocalDatabase(driver)
                val sales = SqlDelightSaleRepository(db)
                val outbox = SqlDelightOutboxRepository(db)

                val sale = sales.createLocalSale(
                    CreateSaleRequest(
                        commerceId = "commerce-1",
                        paymentMethod = "cash",
                        items = listOf(CreateSaleItem("product-1", 2)),
                    ),
                    totalAmount = 20.0,
                )
                sales.updateStatus(sale.localId, LocalSaleStatus.PendingSync)
                val outboxId = "${sale.localId}:create"
                outbox.enqueue(
                    SyncOutboxEntry(
                        id = outboxId,
                        aggregateId = sale.localId,
                        method = "POST",
                        path = "/sales",
                        bodyJson = "{}",
                        idempotencyKey = sale.idempotencyKey,
                        createdAtEpochMs = currentEpochMs(),
                        entityType = SyncEntityType.Sale,
                    ),
                )
                WrittenIds(sale.localId, outboxId)
            }

            JdbcSqliteDriver("jdbc:sqlite:$path").use { driver ->
                val db = createSellerLocalDatabase(driver)
                val restored = SqlDelightSaleRepository(db).getSale(written.localId)
                val pending = SqlDelightOutboxRepository(db).listPendingFifo()

                assertNotNull(restored)
                assertEquals(LocalSaleStatus.PendingSync, restored!!.status)
                assertEquals(1, restored.items.size)
                assertTrue(
                    pending.any { it.id == written.outboxId && it.aggregateId == written.localId },
                )
            }
        } finally {
            File(path).delete()
        }
    }

    private data class WrittenIds(val localId: String, val outboxId: String)
}

private inline fun <T> JdbcSqliteDriver.use(block: (JdbcSqliteDriver) -> T): T {
    try {
        return block(this)
    } finally {
        close()
    }
}
