// Apache Licence - https://apache.org/licenses/LICENSE-2.0.txt
// author: hihellobolke@github

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
        let mut cookie_collector = String::new();

        for (k, v) in self.get_http_request_headers().iter() {
            if k == "cookie" {
                debug!("original cookie ==> `cookie: {:?}`", v);
                let mut cookie_modified = String::new();

                let s: Vec<&str> = v.split(';').collect();
                for &x in s.iter() {
                    if let Some(c) = x.split('=').next() {
                        if self.config.cookie_names.contains(&c.trim().to_owned()) {
                            debug!(" ++ remove cookie: {}", c);
                            continue;
                        }
                    }

                    let x = x.trim();
                    if x.is_empty() {
                        continue;
                    }

                    cookie_modified.push_str(x);
                    cookie_modified.push_str("; ");
                }

                let cookie_modified = cookie_modified.trim();
                if !cookie_modified.is_empty() {
                    cookie_collector.push_str(cookie_modified);
                    cookie_collector.push(' ');
                    debug!(" ++ updated cookie ==> `cookie: {:?}`", cookie_modified);
                }
            }
        }

        if !cookie_collector.is_empty() {
            cookie_collector = cookie_collector.trim().to_owned();
            if cookie_collector.ends_with(';') {
                cookie_collector.pop();
            }
            debug!("final cookie ==> `cookie: {:?}`", cookie_collector);
            self.set_http_request_header("cookie", Some(cookie_collector.as_str()));
        }

        Action::Continue
    }
}
