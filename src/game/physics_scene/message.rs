use super::body::BodyId;
use super::collision_data::CollisionData;

pub enum MessageBody {
    Collided(CollisionData),
    TriggerEntered,
}

pub struct Message {
    pub body: MessageBody,

    pub causer: BodyId,
    pub affected: BodyId,
}
