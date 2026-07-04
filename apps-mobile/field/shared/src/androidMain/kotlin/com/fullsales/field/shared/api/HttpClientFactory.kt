package com.fullsales.field.shared.api

import io.ktor.client.HttpClient
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json

fun createFieldHttpClient(json: Json = Json { ignoreUnknownKeys = true }): HttpClient =
    HttpClient {
        install(ContentNegotiation) { json(json) }
    }
