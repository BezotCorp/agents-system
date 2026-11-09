pub(crate) struct ExperimentResult {
    receiver_message: Vec<u8>,
    interceptor_message: Vec<u8>,
    receiver_success: bool,
    interceptor_success: bool,
}

impl ExperimentResult {
    pub(crate) fn new(
        receiver_message: Vec<u8>,
        interceptor_message: Vec<u8>,
        expected_message: &[u8],
    ) -> Self {
        let receiver_success = receiver_message == expected_message;
        let interceptor_success = interceptor_message == expected_message;

        Self {
            receiver_message,
            interceptor_message,
            receiver_success,
            interceptor_success,
        }
    }

    pub(crate) fn receiver_message(&self) -> &[u8] {
        &self.receiver_message
    }

    pub(crate) fn interceptor_message(&self) -> &[u8] {
        &self.interceptor_message
    }

    pub(crate) const fn receiver_success(&self) -> bool {
        self.receiver_success
    }

    pub(crate) const fn interceptor_success(&self) -> bool {
        self.interceptor_success
    }
}
