/**
 * Contract: catalog share URL uses portal origin + sharePath, with env catalog fallback.
 */
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import com.fullsales.seller.shared.model.SellerShare
import com.fullsales.seller.shared.share.buildCatalogShareUrl
import com.fullsales.seller.shared.share.resolveCatalogShareUrl

class CatalogShareUrlTest {
    @Test
    fun build_joins_origin_and_path() {
        assertEquals(
            "https://catalogo.comerc.app.br/s/abc",
            buildCatalogShareUrl("https://catalogo.comerc.app.br/", "/s/abc"),
        )
    }

    @Test
    fun resolve_prefers_active_share_url() {
        val share = SellerShare(
            publicCode = "abc",
            sharePath = "/s/abc",
            shareUrl = "https://catalogo.comerc.app.br/s/abc",
            shareLinkActive = true,
        )
        assertEquals(
            "https://catalogo.comerc.app.br/s/abc",
            resolveCatalogShareUrl(share, "https://catalogo.comerc.app.br"),
        )
    }

    @Test
    fun resolve_builds_from_path_when_share_url_blank() {
        val share = SellerShare(
            publicCode = "abc",
            sharePath = "/s/abc",
            shareUrl = "",
            shareLinkActive = true,
        )
        assertEquals(
            "https://catalogo.comerc.app.br/s/abc",
            resolveCatalogShareUrl(share, "https://catalogo.comerc.app.br"),
        )
    }

    @Test
    fun resolve_falls_back_to_catalog_origin_when_inactive() {
        val share = SellerShare(
            publicCode = "abc",
            sharePath = "/s/abc",
            shareUrl = "https://catalogo.comerc.app.br/s/abc",
            shareLinkActive = false,
        )
        assertEquals(
            "https://catalogo.comerc.app.br",
            resolveCatalogShareUrl(share, "https://catalogo.comerc.app.br"),
        )
    }

    @Test
    fun resolve_null_share_uses_origin() {
        assertEquals(
            "https://catalogo.comerc.app.br",
            resolveCatalogShareUrl(null, "https://catalogo.comerc.app.br"),
        )
        assertNull(resolveCatalogShareUrl(null, ""))
    }
}
