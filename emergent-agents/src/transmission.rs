pub(crate) struct Transmission {
    protected_message: Vec<u8>,
}

impl Transmission {
    pub(crate) fn new(protected_message: Vec<u8>) -> Self {
        Self { protected_message }
    }

    pub(crate) fn protected_message(&self) -> &[u8] {
        &self.protected_message
    }
}
