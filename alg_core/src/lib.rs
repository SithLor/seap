//shity shader functions for cpus

//#[should_panic(expected = "Cannot normalize a zero vector in GLSL")]


pub mod uri;
#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;
    use std::str::FromStr;

    use crate::uri::{Host, URI,UriGetterSetter};
    #[test]
    fn test_ipv6_addr() {
        let addr = Ipv6Addr::from_str("2001:0db8:85a3:0000:0000:8a2e:0370:7334").unwrap();
        assert_eq!(addr.to_string(), "2001:db8:85a3::8a2e:370:7334");
    }

    #[test]
    fn test_uri_to_string() {
        let uri = URI {
            scheme: "http".to_string(),
            user_info: Some("user:password".to_string()),
            host: Host::RegisteredName("example.com".to_string()),
            port: Some(8080),
            path: "/path".to_string(),
            query: Some("query".to_string()),
            fragment: Some("fragment".to_string()),
        };

        let expected = "http://user:password@example.com:8080/path?query#fragment";
        assert_eq!(uri.to_string(), expected);
    }
    #[test]
    fn test_uri_urigetter_setter() {
        let mut uri = URI::from_str("http://example.com/path").unwrap();
        assert_eq!(uri.get_scheme(), "http");
        uri.set_scheme("https".to_string());
        assert_eq!(uri.get_scheme(), "https");

        assert_eq!(uri.get_user_info(), None);
        uri.set_user_info(Some("user:password".to_string()));
        assert_eq!(uri.get_user_info(), Some("user:password".to_string()));
        
        assert_eq!(uri.get_host(), Host::RegisteredName("example.com".to_string()));
        uri.set_host(Host::IP("127.0.0.1".parse().unwrap()));
        assert_eq!(uri.get_host(), Host::IP("127.0.0.1".parse().unwrap()));

        assert_eq!(uri.get_port(), None);
        uri.set_port(Some(8080));
        assert_eq!(uri.get_port(), Some(8080));

        assert_eq!(uri.get_path(), "/path");
        uri.set_path("/new_path".to_string());
        assert_eq!(uri.get_path(), "/new_path");

        assert_eq!(uri.get_query(), None);
        uri.set_query(Some("query".to_string()) );
        assert_eq!(uri.get_query(), Some("query".to_string()));

        assert_eq!(uri.get_fragment(), None);
        uri.set_fragment(Some("fragment".to_string()));
        assert_eq!(uri.get_fragment(), Some("fragment".to_string()));
    }
}
