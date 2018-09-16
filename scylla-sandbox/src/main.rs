extern crate cdrs;

use cdrs::cluster::Cluster;
use cdrs::load_balancing::RoundRobin;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::query::QueryExecutor;
fn main() {
    let cluster = Cluster::new(vec!["127.0.1.1:9042"], NoneAuthenticator);
    let mut session = cluster.connect(RoundRobin::new())
        .expect("No compression connection error");
    
}
