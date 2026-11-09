use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct AgentId(Uuid);

impl AgentId {
    pub(crate) fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub(crate) const fn value(self) -> Uuid {
        self.0
    }
}
