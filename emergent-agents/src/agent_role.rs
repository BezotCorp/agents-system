#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AgentRole {
    Transmitter,
    Receiver,
    Interceptor,
}
