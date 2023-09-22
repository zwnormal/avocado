use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ServiceAddress {
    pub(crate) user: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) service_address: ServiceAddress,
}

impl Config {
    #[cfg(not(any(test, debug_assertions)))]
    pub(crate) fn new() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::new(
                "config.crm.yaml",
                config::FileFormat::Yaml,
            ))
            .build()
            .unwrap();
        config.try_deserialize::<Config>().unwrap()
    }

    #[cfg(any(test, debug_assertions))]
    pub(crate) fn new() -> Self {
        let service_address = ServiceAddress {
            user: "http://[::1]:50051".to_string(),
        };
        Config { service_address }
    }
}
