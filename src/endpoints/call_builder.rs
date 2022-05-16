use crate::endpoints::{horizon::Record, Server};
use crate::utils::{Direction, Endpoint};

pub trait CallBuilder<'a, T> {
    fn new(s: &'a Server) -> Self;
    fn cursor(&mut self, cursor: &str) -> &mut Self;
    fn order(&mut self, dir: Direction) -> &mut Self;
    fn limit(&mut self, limit_number: u8) -> &mut Self;
    fn call(&self) -> Result<Record<T>, &str>;
    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self;
}
