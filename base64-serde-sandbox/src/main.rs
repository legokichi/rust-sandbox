use base64::STANDARD;
use base64_serde::base64_serde_type;
use serde_json; // 1.0.37
use serde::*;
use serde_derive::*;
use std::collections::BTreeMap;
base64_serde_type!(Base64Standard, STANDARD);
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum HeaderValue {
    Single(String),
    Multiple(Vec<String>)
}
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[serde(untagged)]
pub enum SafeHttpRequestReturn {
    #[serde(rename_all = "camelCase")]
    Ok {
        status_code: u16,
        headers: BTreeMap<String, HeaderValue>,
        #[serde(with = "Base64Standard")]
        body: Vec<u8>,
    },
    #[serde(rename_all = "camelCase")]
    Err {
        /// ex. "connect ECONNREFUSED 169.254.169.254:80"
        error_message: String,
        /// ex. "Error"
        error_type: String,
        /// ```json
        /// [
        ///   "Object._errnoException (util.js:1022:11)",
        ///   "_exceptionWithHostPort (util.js:1044:20)",
        ///   "TCPConnectWrap.afterConnect [as oncomplete] (net.js:1198:14)"
        /// ]
        /// ```
        stack_trace: Vec<String>,
    },
}
fn main() {
    let json = r###"{"statusCode":200,"headers":{"date":"Mon, 04 Mar 2019 07:07:58 GMT","content-type":"text/html","transfer-encoding":"chunked","connection":"close","set-cookie":["__cfduid=db9d3db1d2d69685af2b712ffa6d1023d1551683278; expires=Tue, 03-Mar-20 07:07:58 GMT; path=/; domain=.pastebin.com; HttpOnly"],"vary":"Accept-Encoding","x-xss-protection":"1; mode=block","expect-ct":"max-age=604800, report-uri=\"https://report-uri.cloudflare.com/cdn-cgi/beacon/expect-ct\"","server":"cloudflare","cf-ray":"4b220429dbf0a59c-NRT"},"body":"QmFkIEFQSSByZXF1ZXN0LCBhcGlfcGFzdGVfY29kZSB3YXMgZW1wdHk="}"###;
    
    let a = serde_json::from_str::<SafeHttpRequestReturn>(json).unwrap();
    println!("{:?}", a);
    
}