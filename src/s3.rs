use std::collections::HashSet;
use std::rc::Rc;

use hyper::rt::{Future, Stream};
use hyper::{self, client::HttpConnector};
use hyper_tls::HttpsConnector;
use serde_json;

const BASE_URL: &str = "https://s3.amazonaws.com/challenge.getcrossbeam.com/public";

#[derive(Deserialize, Debug)]
pub struct XBeamResponse {
    companies: Vec<Company>,
}

impl XBeamResponse {

    pub fn domain_set(&self) -> HashSet<&str> {
        let mut set = HashSet::new();
        for company in &self.companies {
            set.insert(company.domain.trim());
        }
        set
    }
}

#[derive(Deserialize, Debug)]
pub struct Company {
    name: String,
    domain: String,
}

#[derive(Clone)]
pub struct S3Client {
    client: Rc<hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>>,
}

impl S3Client {
    pub fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build(https);
        let client = Rc::new(client);
        S3Client { client }
    }

    pub fn fetch_file(self, file_name: &str) -> impl Future<Item = XBeamResponse, Error = ()> {
        let uri = format!("{}/{}.json", BASE_URL, file_name);
        let uri = uri.parse::<hyper::Uri>().unwrap();
        self.client
            .get(uri)
            .and_then(|res| { res.into_body().concat2() })
            .and_then(|body| {
                let s = ::std::str::from_utf8(&body).unwrap();
                let v: XBeamResponse = serde_json::from_str(s).unwrap();
                Ok(v)
            }).map_err(|err| println!("{:?}", err))
    }
}
