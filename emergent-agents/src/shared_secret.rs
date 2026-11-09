pub(crate) struct SharedSecret {
    bytes: Vec<u8>,
}

impl SharedSecret {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub(crate) fn apply(&self, input: &[u8]) -> Vec<u8> {
        input
            .iter()
            .enumerate()
            .map(|(index, byte)| byte ^ self.bytes[index % self.bytes.len()])
            .collect()
    }
}
