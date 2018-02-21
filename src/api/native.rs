// extern crate tls_api_native_tls;
use tls_api;
use http::Request;
use types::CallBuilderImpl;
use mio::{Event, Poll};
use tls_api::TlsConnector;
use Result;

#[derive(Debug)]
pub struct CallBuilder {
    cb: Option<CallBuilderImpl>,
}

impl CallBuilder {
    pub fn new(req: Request<Vec<u8>>) -> CallBuilder {
        CallBuilder {
            cb: Some(CallBuilderImpl::new(req)),
        }
    }
    pub fn call(&mut self, httpc: &mut Httpc, poll: &Poll) -> ::Result<::Call> {
        let cb = self.cb.take().unwrap();
        httpc.call::<tls_api::native::TlsConnector>(cb, poll)
    }
    pub fn websocket(&mut self, httpc: &mut Httpc, poll: &Poll) -> ::Result<::WebSocket> {
        let mut cb = self.cb.take().unwrap();
        cb.websocket();
        let cid = httpc.call::<tls_api::native::TlsConnector>(cb, poll)?;
        Ok(::WebSocket::new(cid, httpc.h.get_buf()))
    }
    pub fn max_response(&mut self, m: usize) -> &mut Self {
        self.cb.as_mut().unwrap().max_response(m);
        self
    }
    pub fn dns_retry_ms(&mut self, n: u64) -> &mut Self {
        self.cb.as_mut().unwrap().dns_retry_ms(n);
        self
    }
    pub fn chunked_parse(&mut self, b: bool) -> &mut Self {
        self.cb.as_mut().unwrap().chunked_parse(b);
        self
    }
    pub fn chunked_max_chunk(&mut self, v: usize) -> &mut Self {
        self.cb.as_mut().unwrap().chunked_max_chunk(v);
        self
    }
    pub fn timeout_ms(&mut self, d: u64) -> &mut Self {
        self.cb.as_mut().unwrap().timeout_ms(d);
        self
    }
    pub fn digest_auth(&mut self, v: bool) -> &mut Self {
        self.cb.as_mut().unwrap().digest_auth(v);
        self
    }
    pub fn gzip(&mut self, b: bool) -> &mut Self {
        self.cb.as_mut().unwrap().gzip(b);
        self
    }
    pub fn max_redirects(&mut self, v: u8) -> &mut Self {
        self.cb.as_mut().unwrap().max_redirects(v);
        self
    }
    pub fn insecure_do_not_verify_domain(&mut self) -> &mut Self {
        self.cb.as_mut().unwrap().insecure();
        self
    }
}

pub struct Httpc {
    h: ::httpc::HttpcImpl,
}

impl Httpc {
    pub fn new(con_offset: usize, cfg: Option<::HttpcCfg>) -> Httpc {
        Httpc {
            h: ::httpc::HttpcImpl::new(con_offset, cfg),
        }
    }
    pub(crate) fn call<C: TlsConnector>(
        &mut self,
        b: CallBuilderImpl,
        poll: &Poll,
    ) -> Result<::Call> {
        self.h.call::<C>(b, poll)
    }
    pub(crate) fn peek_body(&mut self, id: &::Call, off: &mut usize) -> &[u8] {
        self.h.peek_body(id, off)
    }
    pub(crate) fn try_truncate(&mut self, id: &::Call, off: &mut usize) {
        self.h.try_truncate(id, off);
    }
    pub fn open_connections(&self) -> usize {
        self.h.open_connections()
    }
    pub fn reuse(&mut self, buf: Vec<u8>) {
        self.h.reuse(buf);
    }
    pub fn call_close(&mut self, id: ::Call) {
        self.h.call_close(id);
    }
    pub fn timeout(&mut self) -> Vec<::CallRef> {
        self.h.timeout()
    }
    pub fn timeout_extend<C: TlsConnector>(&mut self, out: &mut Vec<::CallRef>) {
        self.h.timeout_extend(out)
    }
    pub fn event(&mut self, ev: &Event) -> Option<::CallRef> {
        self.h.event::<tls_api::native::TlsConnector>(ev)
    }
    pub fn call_send(&mut self, poll: &Poll, id: &mut ::Call, buf: Option<&[u8]>) -> ::SendState {
        self.h
            .call_send::<tls_api::native::TlsConnector>(poll, id, buf)
    }
    pub fn call_recv(
        &mut self,
        poll: &Poll,
        id: &mut ::Call,
        buf: Option<&mut Vec<u8>>,
    ) -> ::RecvState {
        self.h
            .call_recv::<tls_api::native::TlsConnector>(poll, id, buf)
    }
}
