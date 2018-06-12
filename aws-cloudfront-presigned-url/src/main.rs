extern crate openssl;
extern crate base64;
extern crate getopts;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.reqopt("", "policy", "set custom policy json file", "policy.json");
    opts.reqopt("", "private_key", "set rsa private key file", "id_rsa");
    opts.reqopt("", "url", "set cloudfront url", "https://foo.clouodfront.net/foo/bat/baz.jpg");
    opts.reqopt("", "key_pair_id", "Key-Pair-Id", "APKA9ONS7QCOWEXAMPLE");
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") {
        let help = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&help));
        return;
    }
    let url = matches.opt_str("url").unwrap();
    let key_pair_id = matches.opt_str("key_pair_id").unwrap();
    let mut private_key: Vec<u8> = vec![];
    {
        let path = matches.opt_str("private_key").unwrap();
        let mut f = std::fs::File::open(path).unwrap();
        f.read_to_end(&mut private_key).unwrap();
    }
    let policy: Vec<u8> = {
        let mut policy_buf: Vec<u8> = vec![];
        let path = matches.opt_str("policy").unwrap();
        let mut f = std::fs::File::open(path).unwrap();
        f.read_to_end(&mut policy_buf).unwrap();
        let policy_str = std::str::from_utf8(&policy_buf).unwrap();
        let policy_str = policy_str.replace("\n", "").replace("\r", "");
        policy_str.into_bytes()
    };
    let keypair = openssl::pkey::PKey::private_key_from_pem(&private_key.into_boxed_slice()).unwrap();
    let mut signer = openssl::sign::Signer::new(openssl::hash::MessageDigest::sha1(), &keypair).unwrap();
    signer.update(&policy.clone().into_boxed_slice()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    let b64_policy = base64::encode(&policy.into_boxed_slice()).replace("+", "-").replace("=", "_").replace("/", "~");
    let b64_signature = base64::encode(&signature).replace("+", "-").replace("=", "_").replace("/", "~");
    // println!("url: {}", url);
    // println!("policy: {}", b64_policy);
    // println!("signature: {}", b64_signature);
    // println!("key_pair_id: {}", key_pair_id);
    println!(
        "{url}?Policy={policy}&Signature={signature}&Key-Pair-Id={key_pair_id}",
        url=url,
        policy=b64_policy,
        signature=b64_signature,
        key_pair_id=key_pair_id,
    );
}
