use anyhow::Error;
use clap::{Parser};
use serde::{Deserialize, Serialize};
use spin_trigger::{cli::TriggerExecutorCommand, TriggerExecutor, TriggerAppEngine};

wit_bindgen_wasmtime::import!({paths: ["wit/spin-timer.wit"], async: *});

pub(crate) type RuntimeData = spin_timer::SpinTimerData;
pub(crate) type _Store = spin_core::Store<RuntimeData>;

type Command = TriggerExecutorCommand<TimerTrigger>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let t = Command::parse();
    t.run().await
}

struct TimerTrigger {
    engine: TriggerAppEngine<Self>,
    components: Vec<String>,
}

#[async_trait::async_trait]
impl TriggerExecutor for TimerTrigger {
    const TRIGGER_TYPE: &'static str = "timer";

    type RuntimeData = RuntimeData;

    type TriggerConfig = TimerTriggerConfig;

    type RunConfig = spin_trigger::cli::NoArgs;

    fn new(engine: spin_trigger::TriggerAppEngine<Self>) -> anyhow::Result<Self>  {
        let components = engine
            .trigger_configs()
            .map(|(_, config)| config.component.clone())
            .collect();

        Ok(Self { engine, components })
    }

    async fn run(self, _config: Self::RunConfig) -> anyhow::Result<()> {
        loop {
            let component_id = &self.components[0];
            self.handle_timer_event(component_id).await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}

impl TimerTrigger {
    async fn handle_timer_event(&self, component_id: &str) -> anyhow::Result<()> {
        let (instance, mut store) = self.engine.prepare_instance(component_id).await?;
        let engine = spin_timer::SpinTimer::new(&mut store, &instance, |data| data.as_mut())?;
        let ret = engine
            .handle_timer_request(&mut store)
            .await?;
        println!("Event handler returned {ret}");
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimerTriggerConfig {
    component: String,
    // don't ask
    queue_url: String,
}
