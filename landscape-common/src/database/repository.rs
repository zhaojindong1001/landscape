/// 从 domain type 提取 ID 和 update_at 时间戳
pub trait LandscapeDBStore<Id> {
    fn get_id(&self) -> Id;
    fn get_update_at(&self) -> f64;
    fn set_update_at(&mut self, ts: f64);
}
