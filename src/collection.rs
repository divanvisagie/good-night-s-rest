use crate::requests::Request;

pub type Collection = Vec<Request>;

pub struct CollectionItem {
    pub name: String,
    pub collection: Collection,
}
