use std::rc::Rc;

use hyper::rt::{Future, Stream};
use hyper::{self, client::HttpConnector};
use hyper_tls::HttpsConnector;
use serde_json;

const BASE_URL: &str = "https://api.iextrading.com/1.0";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IexResponse {
    symbol: String,
    pub delayed_price: f32,
    close: f32,
    ytd_change: f32,
}

impl IexResponse {
    pub fn get_symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn ytd_percentage(&self) -> f32 {
        self.ytd_change * 100.0
    }

    pub fn daily_percentage(&self) -> f32 {
        ((self.delayed_price - self.close) / self.close) * 100.0
    }
}

#[derive(Clone)]
pub struct IexClient {
    client: Rc<hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>>,
}

impl IexClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build(https);
        let client = Rc::new(client);
        IexClient { client }
    }

    pub fn fetch_stock(self, ticker: &str) -> impl Future<Item = IexResponse, Error = ()> {
        let uri = format!("{}/stock/{}/quote", BASE_URL, ticker);
        let uri = uri.parse::<hyper::Uri>().unwrap();
        self.client
            .get(uri)
            .and_then(|res| res.into_body().concat2())
            .and_then(|body| {
                let s = ::std::str::from_utf8(&body).expect("httpbin sends utf-8 JSON");
                let v: IexResponse = serde_json::from_str(s).unwrap();
                Ok(v)
            }).map_err(|err| println!("{:?}", err))
    }
}
