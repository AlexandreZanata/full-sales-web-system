# Keep rules for Seller release (R8 / OD-21-5).

-keepattributes *Annotation*, InnerClasses, EnclosingMethod, Signature
-keepattributes RuntimeVisibleAnnotations, RuntimeVisibleParameterAnnotations

# Kotlinx Serialization
-keepclassmembers class kotlinx.serialization.json.** { *** Companion; }
-keepclasseswithmembers class **$$serializer { *; }
-if @kotlinx.serialization.Serializable class **
-keepclassmembers class <1> {
    static <1>$Companion Companion;
}

# Ktor / OkHttp-style clients
-dontwarn okhttp3.**
-dontwarn okio.**
-dontwarn org.slf4j.**

# Room entities / DAOs (androidMain)
-keep class * extends androidx.room.RoomDatabase
-keep @androidx.room.Entity class *
-keep @androidx.room.Dao interface *

# EncryptedSharedPreferences / Tink
-dontwarn com.google.crypto.tink.**
