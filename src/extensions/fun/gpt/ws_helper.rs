use url::Url;

pub async fn open_tls_stream(ws_url: &Url) -> tokio_native_tls::TlsStream<tokio::net::TcpStream> {
    let mut connector = tokio_native_tls::native_tls::TlsConnector::builder();

    // These certs should be shared between the reqwest and tungstenite clients
    let tls_ca: Vec<tokio_native_tls::native_tls::Certificate> = Vec::new();

    tls_ca.iter().for_each(|ca| {
        connector.add_root_certificate(ca.clone());
    });

    let connector = connector.build().unwrap();
    let connector: tokio_native_tls::TlsConnector = connector.into();
    let addrs = ws_url.socket_addrs(|| None).unwrap();
    let stream = tokio::net::TcpStream::connect(&*addrs).await.unwrap();
    let stream = connector.connect(ws_url.as_str(), stream).await.unwrap();
    stream
}
