// This method will connect client request to out of the world
pub(crate) async fn get_out(req: fpm::http::Request<'_>) {
    let headermap = req.headers();
    // TODO: It should be part of fpm::Request
    let path = &req.req.uri().to_string()[1..];

    let mut proxy_request = reqwest::Request::new(
        req.req.method().clone(),
        reqwest::Url::parse("http://127.0.0.1").unwrap(),
    );

    // Problem no way to pass headers in reqwest with ClientBuilder with execute method
    // TODO: Connection Pool, It by default holds the connection pool internally
    // let client = reqwest::ClientBuilder::new()
    //     .http2_adaptive_window(true)
    //     .tcp_keepalive(std::time::Duration::new(150, 0))
    //     .tcp_nodelay(true)
    //     .connect_timeout(std::time::Duration::new(150, 0))
    //     .connection_verbose(true)
    //     .build()
    //     .unwrap();
    //
    // match client.execute(proxy_request).await {
    //     Ok(reponse) => fpm::http::ResponseBuilder::from(),
    //     Err(e) =>
    // }
}
