/// 从 domain type 提取 ID
pub trait LandscapeDBStore<Id> {
    fn get_id(&self) -> Id;
}
