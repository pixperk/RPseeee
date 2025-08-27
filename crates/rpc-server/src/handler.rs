#[async_trait::async_trait]
pub trait Handler : Send + Sync{
    fn handle(&self, payload : &[u8]) -> Vec<u8>;
}
