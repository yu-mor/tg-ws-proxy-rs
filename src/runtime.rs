use std::collections::HashMap;

use crate::config::{default_dc_ips, default_dc_overrides};
use crate::outbound::OutboundConnector;

pub struct Runtime {
    outbound: OutboundConnector,
    dc_overrides: HashMap<u32, u32>,
    dc_fallback_ips: HashMap<u32, String>,
}

impl Runtime {
    pub fn new(outbound: OutboundConnector) -> Self {
        Self {
            outbound,
            dc_overrides: default_dc_overrides(),
            dc_fallback_ips: default_dc_ips(),
        }
    }

    pub fn outbound(&self) -> &OutboundConnector {
        &self.outbound
    }

    pub fn websocket_dc(&self, dc: u32) -> u32 {
        *self.dc_overrides.get(&dc).unwrap_or(&dc)
    }

    pub fn fallback_ip(&self, dc: u32) -> Option<&str> {
        self.dc_fallback_ips.get(&dc).map(String::as_str)
    }
}
