use crate::core::v1::PROTOCOL_TCP;

use super::{EndpointPort, EndpointSlice};

pub fn apply_defaults_endpoint_slice(slice: &mut EndpointSlice) {
    for port in &mut slice.ports {
        apply_defaults_endpoint_port(port);
    }
}

fn apply_defaults_endpoint_port(port: &mut EndpointPort) {
    if port.name.is_none() {
        port.name = Some(String::new());
    }
    if port.protocol.is_none() {
        port.protocol = Some(PROTOCOL_TCP.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_endpoint_port_fields() {
        let mut slice = EndpointSlice {
            ports: vec![EndpointPort {
                port: Some(80),
                ..Default::default()
            }],
            ..Default::default()
        };

        apply_defaults_endpoint_slice(&mut slice);

        let port = &slice.ports[0];
        assert_eq!(port.name.as_deref(), Some(""));
        assert_eq!(port.protocol.as_deref(), Some(PROTOCOL_TCP));
        assert_eq!(port.port, Some(80));
    }

    #[test]
    fn test_default_endpoint_port_preserves_values() {
        let mut slice = EndpointSlice {
            ports: vec![EndpointPort {
                name: Some("foo".to_string()),
                protocol: Some("UDP".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        apply_defaults_endpoint_slice(&mut slice);

        let port = &slice.ports[0];
        assert_eq!(port.name.as_deref(), Some("foo"));
        assert_eq!(port.protocol.as_deref(), Some("UDP"));
    }
}
