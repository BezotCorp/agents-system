mod agent;
mod agent_id;
mod agent_role;
mod brain_cognitive_failure;
mod brain_execution;
mod brain_genome;
mod brain_value;
mod brain_vm;
mod brain_vm_error;
mod experiment;
mod experiment_result;
mod order_by;
mod shared_secret;
mod start;
mod transmission;
mod unit;
mod unit_id;

use std::collections::BTreeMap;

use agent::Agent;
use agent_id::AgentId;
use agent_role::AgentRole;
use brain_execution::BrainExecution;
use brain_genome::BrainGenome;
use brain_value::BrainValue;
use brain_vm::BrainVm;
use experiment::Experiment;
use order_by::OrderBy;
use shared_secret::SharedSecret;
use start::Start;
use unit::Unit;

fn main() -> Result<(), String> {
    let transmitter = Agent::new(AgentId::new(), "Alice", AgentRole::Transmitter);

    let receiver = Agent::new(AgentId::new(), "Bob", AgentRole::Receiver);

    let interceptor = Agent::new(AgentId::new(), "Eve", AgentRole::Interceptor);

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

    println!();
    println!("Brain VM experiment");
    println!();

    let mut order_values = BTreeMap::new();
    order_values.insert("y".to_owned(), 1.0);

    let genome = BrainGenome::new(vec![
        Unit::start(Start::Marker, OrderBy::new("x + y", order_values)),
        Unit::convert_into_bytes(),
        Unit::output(),
    ]);

    let brain = BrainVm::from_genome(genome).map_err(|error| error.to_string())?;

    let execution = brain.execute(BrainValue::Text("Hello Bob".to_owned()), 100);

    match execution {
        BrainExecution::Output { value, steps } => {
            println!("Brain output: {value:?}");
            println!("Brain steps: {steps}");
        }

        BrainExecution::CognitiveFailure { failure, steps } => {
            println!("Mental bug: {failure:?}");
            println!("Brain steps: {steps}");
        }
    }

    Ok(())
}
