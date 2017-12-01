#![recursion_limit="128"] // because of quick_error

//! mio_httpc is an async http client that runs on top of mio only. 
//! 
//! No call will block, not even for DNS resolution as it is implemented internally to avoid blocking.
//!
//! mio_httpc requires you specify one of the TLS implementations using features: rustls, native, openssl.
//! Default is noop for everything.
//! 
//! mio_httpc does a minimal amount of allocation and in general works with buffers you provide and an internal pool
//! of buffers that get reused on new calls.
extern crate rand;
extern crate httparse;
extern crate tls_api;
extern crate mio;
extern crate byteorder;
extern crate libc;
extern crate fnv;
extern crate http;
extern crate itoa;
extern crate data_encoding;
#[macro_use(quick_error)]
extern crate quick_error;
#[cfg(test)]
#[macro_use]
extern crate matches;
#[cfg(target_os = "macos")]
extern crate core_foundation;
#[cfg(target_os = "macos")]
extern crate core_foundation_sys;

// Because of default implementation does nothing we suppress warnings of nothing going on.
// One of TLS implementation features must be picked.
// #[allow(dead_code,unused_variables)]
// mod con_table;
#[allow(dead_code,unused_variables)]
mod dns_cache;
#[allow(dead_code,unused_variables)]
mod dns;
#[allow(dead_code)]
mod dns_parser;
#[allow(dead_code,unused_variables)]
mod con;
#[allow(dead_code,unused_variables)]
mod httpc;
#[allow(dead_code,unused_variables)]
mod call;
#[allow(dead_code,unused_variables)]
mod api;
mod types;

pub use api::*;
pub use http::{Error as HttpError};
pub use http::header::*;
pub use http::method::*;
pub use http::request::*;
pub use http::response::*;
pub use http::status::*;
pub use http::uri::*;
pub use http::version::*;
// pub use http::Extensions;

pub type Result<T> = ::std::result::Result<T,Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            description(err.description())
            from()
        }
        Utf8(err: std::str::Utf8Error) {
            description(err.description())
            from()
        }
        FromUtf8(err: std::string::FromUtf8Error) {
            description(err.description())
            from()
        }
        Addr(err: std::net::AddrParseError) {
            description(err.description())
            from()
        }
        Tls(err: tls_api::Error) {
            description(err.description())
            from()
        }
        Httparse(err: httparse::Error) {
            description(err.description())
            from()
        }
        Http(err: http::Error) {
            description(err.description())
            from()
        }
        WebSocketFail(req: http::Response<Vec<u8>>) {
            display("WebSocket setup failed {:?}",req)
        }
        /// Request structure did not contain body and CallSimple was used for POST/PUT.
        MissingBody {
            display("Request structure did not contain body and CallSimple was used for POST/PUT.")
        }
        /// No call for mio::Token
        InvalidToken {
            display("No call for token")
        }
        /// Response over max_response limit
        ResponseTooBig {
            display("Response over max_response limit")
        }
        /// Connection closed.
        Closed {
            display("Connection closed")
        }
        /// No host found in request
        NoHost {
            display("No host found in request")
        }
        /// Invalid scheme
        InvalidScheme {
            display("Invalid scheme")
        }
        /// TLS handshake failed.
        TlsHandshake {
            display("Handshake failed")
        }
        /// All 0xFFFF slots for connections are full.
        NoSpace {
            display("Concurrent connection limit")
        }
        /// You must pick one of the features: native, rustls, openssl
        NoTls {
            display("You must pick one of the features: native, rustls, openssl")
        }
        /// Eror while parsing chunked stream
        ChunkedParse {
            display("Error parsing chunked transfer")
        }
        /// Eror while parsing chunked stream
        WebSocketParse {
            display("Error parsing WebSocket transfer")
        }
        /// Chunk was larger than configured CallBuilder::cunked_max_chunk.
        ChunkOverlimit(v:usize) {
            display("Chunk was larger than configured CallBuilder::cunked_max_chunk. {}",v)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        let mut v:Vec<u8> = vec![1,2,3,4,5];
    }
}
