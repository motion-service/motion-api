use mongodb::bson::Document;
use mongodb::Collection;
use crate::structs::page::Page;
use crate::structs::user::User;

#[derive(Clone, Debug)]
pub struct UserDb {
    pub user_collection: Collection<User>,
    pub user_doc_collection: Collection<Document>
}
