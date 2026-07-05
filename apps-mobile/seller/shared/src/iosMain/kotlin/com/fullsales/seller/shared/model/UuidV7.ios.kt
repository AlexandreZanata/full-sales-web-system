package com.fullsales.seller.shared.model

import platform.Foundation.NSDate
import platform.Foundation.timeIntervalSince1970

internal actual fun currentEpochMs(): Long =
    (NSDate().timeIntervalSince1970 * 1000.0).toLong()
