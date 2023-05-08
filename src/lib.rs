// Apache Licence - https://apache.org/licenses/LICENSE-2.0.txt
// author: hihellobolke@github

use cookie::Cookie;

use log::{debug, error, info};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

mod config;
mod constants;
mod errors;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(FilterRoot {
            config: crate::config::FilterConfig::default(),
        })
    });
}}

struct FilterRoot {
    config: crate::config::FilterConfig,
}

struct Filter {
    config: crate::config::FilterConfig,
}

impl Context for FilterRoot {}
impl Context for Filter {}

impl RootContext for FilterRoot {
    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            self.config = serde_json::from_slice(config_bytes.as_slice()).unwrap();
            info!("Loaded configuration: `{:?}` ", self.config);
            true
        } else {
            error!("Could not load configuration");
            false
        }
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(Filter {
            config: self.config.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl HttpContext for Filter {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_request_header("cookie") {
            Some(c) => {
                debug!("Original header ==> `cookie: {}`", c);
                let c: String = Cookie::split_parse(c)
                    .filter_map(|x| x.ok())
                    .filter(|x| !self.config.cookie_names.contains(&x.name().to_owned()))
                    .map(|y| y.to_string())
                    .collect::<Vec<String>>()
                    .join("; ");
                debug!("Modified header ==> `cookie: {}`", c);
                self.set_http_request_header("cookie", Some(c.as_str()));
                Action::Continue
            }
            _ => Action::Continue,
        }
    }
}
