pub fn sample() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/users", post(create_user).get(list_users))
        .route("/v1/users/{id}", get(get_user))
        .route(
            "/v1/categories/{id}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        )
        // ignored .route("/v1/ghost", get(ghost))
}
