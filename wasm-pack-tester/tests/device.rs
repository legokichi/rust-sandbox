#![cfg(target_arch = "wasm32")]

use device_api::mock_impl::client::Client;
use tower_service::Service;
use device_api::model::device_register;
use device_api::model::device_unregister;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn device(){
    let mut client = Client{};
    let o = client.call(device_register::Request{

    }).await.unwrap();
    let o = client.call(device_unregister::Request{
        
    }).await.unwrap();
}
