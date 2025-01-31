use super::{Entity, Moment, Action};
use uuid::Uuid;

pub enum Effect {
    ApplyAura {
        source: Entity,
        target: Entity,
        aura: u32,
        duration: Moment
    },
    RemoveAura {
        source: Entity,
        target: Entity,
        aura: u32
    },
    Damage {
        source: Entity,
        target: Entity,
        action: u32
    },
    ModifyResource {
        target: Entity,
        resource: String,
        amount: i32
    },
    BeginCast {
        source: Entity,
        target: Entity,
        action: Action,
        duration: Moment
    },
    FinishCast {
        source: Entity,
        target: Entity,
        action: Action
    },
    BeginAnimationLock {
        action: Action,
        target: Entity,
        start: Moment,
        duration: Moment
    },
    BeginIdle {
        target: Entity,
        start: Moment
    }
}