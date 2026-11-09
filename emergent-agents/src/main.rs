mod agent;
mod agent_id;
mod agent_role;
mod brain_genome;
mod brain_runtime;
mod brain_vm;
mod brain_vm_error;
mod experiment;
mod experiment_result;
mod shared_secret;
mod transmission;
mod unit;
mod unit_definition;
mod unit_id;

use agent::Agent;
use agent_id::AgentId;
use agent_role::AgentRole;
use experiment::Experiment;
use shared_secret::SharedSecret;

fn main() -> Result<(), String> {
    let transmitter = Agent::new(AgentId::new(0), "Alice", AgentRole::Transmitter);

    let receiver = Agent::new(AgentId::new(1), "Bob", AgentRole::Receiver);

    let interceptor = Agent::new(AgentId::new(2), "Eve", AgentRole::Interceptor);

    let experiment = Experiment::new(
        transmitter,
        receiver,
        interceptor,
        SharedSecret::new(vec![17, 93, 201, 42]),
    )?;

    let original_message = b"Hello Bob";

    println!("Adversarial communication experiment");
    println!();

    println!(
        "Agent {} - {} - {:?}",
        experiment.transmitter().id().value(),
        experiment.transmitter().name(),
        experiment.transmitter().role(),
    );

    println!(
        "Agent {} - {} - {:?}",
        experiment.receiver().id().value(),
        experiment.receiver().name(),
        experiment.receiver().role(),
    );

    println!(
        "Agent {} - {} - {:?}",
        experiment.interceptor().id().value(),
        experiment.interceptor().name(),
        experiment.interceptor().role(),
    );

    println!();
    println!(
        "Original message: {:?}",
        String::from_utf8_lossy(original_message)
    );

    let result = experiment.run(original_message);

    println!(
        "Receiver result: {:?}",
        String::from_utf8_lossy(result.receiver_message())
    );

    println!(
        "Interceptor result: {:?}",
        String::from_utf8_lossy(result.interceptor_message())
    );

    println!();
    println!("Receiver success: {}", result.receiver_success());

    println!("Interceptor success: {}", result.interceptor_success());

    Ok(())
}
