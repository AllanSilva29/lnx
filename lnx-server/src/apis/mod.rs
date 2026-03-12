use poem_openapi::Tags;

mod document;
mod health;
mod info;
mod index;
mod query;

pub use self::document::LnxDocumentApi;
pub use self::health::LnxHealthApi;
pub use self::info::LnxInfoApi;
pub use self::index::LnxIndexApi;
pub use self::query::LnxQueryApi;

#[derive(Tags)]
pub(super) enum Tag {
    #[oai(rename = "Health Endpoints")]
    /// Service health related endpoints
    ///
    /// This can be used when behind load balancers or to ensure an instance is operating
    /// correctly.
    ///
    /// The basic health check endpoint will always return `200 OK` and can be used to
    /// check the service is reachable.
    HealthEndpoints,
    #[oai(rename = "Info Endpoints")]
    /// Service information endpoints allows access to inspect:
    ///
    /// - Service Info
    ///     * Version
    ///     * Listen address
    ///     * Number of indexes
    ///     * Documents stored
    ///     * Uptime
    /// - Host machine Info
    ///     * OS & Version
    ///     * System limits
    ///     * Disk space
    ///     * CPU usage/allowance
    ///     * Memory usage/allowance
    ///
    InfoEndpoints,
    #[oai(rename = "Index Endpoints")]
    /// Index management endpoints
    ///
    /// Create, delete, and list indexes.
    IndexEndpoints,
    #[oai(rename = "Document Endpoints")]
    /// Document indexing endpoints
    ///
    /// Add documents to indexes.
    DocumentEndpoints,
    #[oai(rename = "Query Endpoints")]
    QueryEndpoints,
}
