use crate::{
    agent::Agent, agent_role::AgentRole, experiment_result::ExperimentResult,
    shared_secret::SharedSecret, transmission::Transmission,
};

pub(crate) struct Experiment {
    transmitter: Agent,
    receiver: Agent,
    interceptor: Agent,
    shared_secret: SharedSecret,
}

impl Experiment {
    pub(crate) fn new(
        transmitter: Agent,
        receiver: Agent,
        interceptor: Agent,
        shared_secret: SharedSecret,
    ) -> Result<Self, String> {
        if transmitter.role() != AgentRole::Transmitter {
            return Err("transmitter agent has an invalid role".to_owned());
        }

        if receiver.role() != AgentRole::Receiver {
            return Err("receiver agent has an invalid role".to_owned());
        }

        if interceptor.role() != AgentRole::Interceptor {
            return Err("interceptor agent has an invalid role".to_owned());
        }

        Ok(Self {
            transmitter,
            receiver,
            interceptor,
            shared_secret,
        })
    }

    pub(crate) fn transmitter(&self) -> &Agent {
        &self.transmitter
    }

    pub(crate) fn receiver(&self) -> &Agent {
        &self.receiver
    }

    pub(crate) fn interceptor(&self) -> &Agent {
        &self.interceptor
    }

    pub(crate) fn run(&self, message: &[u8]) -> ExperimentResult {
        let transmission = self.transmit(message);

        let receiver_message = self.receive(transmission.protected_message());

        let interceptor_message = self.intercept(transmission.protected_message());

        ExperimentResult::new(receiver_message, interceptor_message, message)
    }

    fn transmit(&self, message: &[u8]) -> Transmission {
        Transmission::new(self.shared_secret.apply(message))
    }

    fn receive(&self, protected_message: &[u8]) -> Vec<u8> {
        self.shared_secret.apply(protected_message)
    }

    fn intercept(&self, protected_message: &[u8]) -> Vec<u8> {
        protected_message.to_vec()
    }
}
