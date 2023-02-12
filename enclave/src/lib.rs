#![crate_name = "tcp_server_enclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate net2;
extern crate sgx_rand;

mod http;
mod laplace;

use http::TCPServer;
use sgx_types::*;
use std::slice;
use std::time::{Duration, SystemTime};
use std::untrusted::time::SystemTimeEx;

#[no_mangle]
pub extern "C" fn start_server(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let server = TCPServer::new("127.0.0.1", 50080);
    server.run(SystemTime::now());

    sgx_status_t::SGX_SUCCESS
}
