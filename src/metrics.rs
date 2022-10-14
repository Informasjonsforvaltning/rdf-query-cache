use lazy_static::lazy_static;
use prometheus::{Encoder, Histogram, HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry};

use crate::error::Error;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref PROCESSED_REQUESTS: IntCounterVec = IntCounterVec::new(
        Opts::new("processed_requests", "Processed Query Requests"),
        &["endpoint", "status"]
    )
    .unwrap_or_else(|e| {
        tracing::error!(error = e.to_string(), "processed_requests metric error");
        std::process::exit(1);
    });
    pub static ref RESPONSE_TIME: HistogramVec = HistogramVec::new(
        HistogramOpts {
            common_opts: Opts::new("response_time", "Response Times"),
            buckets: vec![0.5, 1.0, 2.5, 5.0, 10.0, 25.0, 50.0],
        },
        &["endpoint", "cache_lvl"]
    )
    .unwrap_or_else(|e| {
        tracing::error!(error = e.to_string(), "response_time");
        std::process::exit(1);
    });
    pub static ref QUERY_PROCESSING_TIME: Histogram = Histogram::with_opts(HistogramOpts {
        common_opts: Opts::new("query_processing_time", "Query Processing Times"),
        buckets: vec![0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1],
    })
    .unwrap_or_else(|e| {
        tracing::error!(error = e.to_string(), "query_processing_time");
        std::process::exit(1);
    });
    pub static ref DATA_FETCH_TIME: Histogram = Histogram::with_opts(HistogramOpts {
        common_opts: Opts::new("data_fetch_time", "Data Fetch Times"),
        buckets: vec![0.5, 1.0, 2.5, 5.0, 10.0, 25.0, 50.0],
    })
    .unwrap_or_else(|e| {
        tracing::error!(error = e.to_string(), "data_fetch_time");
        std::process::exit(1);
    });
    pub static ref GRAPH_PARSE_TIME: Histogram = Histogram::with_opts(HistogramOpts {
        common_opts: Opts::new("graph_parse_time", "Data Fetch Times"),
        buckets: vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 25.0],
    })
    .unwrap_or_else(|e| {
        tracing::error!(error = e.to_string(), "graph_parse_time");
        std::process::exit(1);
    });
}

pub fn register_metrics() {
    REGISTRY
        .register(Box::new(PROCESSED_REQUESTS.clone()))
        .unwrap_or_else(|e| {
            tracing::error!(
                error = e.to_string(),
                "processed_query_requests collector error"
            );
            std::process::exit(1);
        });

    REGISTRY
        .register(Box::new(RESPONSE_TIME.clone()))
        .unwrap_or_else(|e| {
            tracing::error!(error = e.to_string(), "response_time collector error");
            std::process::exit(1);
        });

    REGISTRY
        .register(Box::new(QUERY_PROCESSING_TIME.clone()))
        .unwrap_or_else(|e| {
            tracing::error!(
                error = e.to_string(),
                "query_processing_time collector error"
            );
            std::process::exit(1);
        });

    REGISTRY
        .register(Box::new(DATA_FETCH_TIME.clone()))
        .unwrap_or_else(|e| {
            tracing::error!(error = e.to_string(), "data_fetch_time collector error");
            std::process::exit(1);
        });

    REGISTRY
        .register(Box::new(GRAPH_PARSE_TIME.clone()))
        .unwrap_or_else(|e| {
            tracing::error!(error = e.to_string(), "graph_parse_time collector error");
            std::process::exit(1);
        });
}

pub fn get_metrics() -> Result<String, Error> {
    let mut buffer = Vec::new();

    prometheus::TextEncoder::new()
        .encode(&REGISTRY.gather(), &mut buffer)
        .map_err(|e| e.to_string())?;

    let metrics = String::from_utf8(buffer).map_err(|e| e.to_string())?;
    Ok(metrics)
}
