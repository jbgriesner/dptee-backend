#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate net2;

mod http;

use http::TCPServer;
use sgx_types::*;
use std::slice;

#[no_mangle]
pub extern "C" fn start_server(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };

    println!("{}", &"[+] TCP server started inside Enclave");

    let server = TCPServer::new("127.0.0.1", 50080);
    server.run();

    sgx_status_t::SGX_SUCCESS
}
