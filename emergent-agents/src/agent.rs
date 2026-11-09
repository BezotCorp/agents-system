use crate::{agent_id::AgentId, agent_role::AgentRole};

pub(crate) struct Agent {
    id: AgentId,
    name: String,
    role: AgentRole,
}

impl Agent {
    pub(crate) fn new(id: AgentId, name: impl Into<String>, role: AgentRole) -> Self {
        Self {
            id,
            name: name.into(),
            role,
        }
    }

    pub(crate) const fn id(&self) -> AgentId {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn role(&self) -> AgentRole {
        self.role
    }
}
