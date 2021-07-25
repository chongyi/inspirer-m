#[async_trait]
pub trait WriteProcessChannel<T> {
    /// 写入数据
    async fn write(&self, data: T);
}