use crate::structs::content::Content;

pub trait IdentifyContent{
    async fn get_block_contents(&self, uuid: String) -> Vec<Content>;
    async fn update_block_contents(&self, uuid: String) -> Vec<Content>;
    async fn remove_block_contents(&self, uuid: String) -> Vec<Content>;
}