use std::collections::HashMap;

use super::body::BodyId;
use super::collision_data::CollisionData;

#[derive(Clone)]
pub enum MessageBody {
    Collided(CollisionData),
    TriggerEntered,
}

#[derive(Clone)]
pub struct Message {
    pub body: MessageBody,

    pub causer: BodyId,
    pub affected: BodyId,
}

pub struct MessageHierarchy {
    pub messages: HashMap<BodyId, Vec<Message>>,
    pub nested: Vec<MessageHierarchy>,
}
