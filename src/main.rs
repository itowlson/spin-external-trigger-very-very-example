use std::collections::HashMap;

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
    component_timings: HashMap<String, tokio::time::Duration>,
}

#[async_trait::async_trait]
impl TriggerExecutor for TimerTrigger {
    const TRIGGER_TYPE: &'static str = "timer";

    type RuntimeData = RuntimeData;

    type TriggerConfig = TimerTriggerConfig;

    type RunConfig = spin_trigger::cli::NoArgs;

    fn new(engine: spin_trigger::TriggerAppEngine<Self>) -> anyhow::Result<Self>  {
        let component_timings = engine
            .trigger_configs()
            .map(|(_, config)| (config.component.clone(), secs_text_to_dur(&config.interval_secs)))
            .collect();

        Ok(Self { engine, component_timings })
    }

    async fn run(self, _config: Self::RunConfig) -> anyhow::Result<()> {
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            std::process::exit(0);
        });
        tokio_scoped::scope(|scope|
            for (c, d) in &self.component_timings {
                scope.spawn(async {
                    loop {
                        self.handle_timer_event(c).await.unwrap();
                        tokio::time::sleep(*d).await;
                    }
                });
            }
        );
        Ok(())
    }
}

fn secs_text_to_dur(interval_secs: &str) -> tokio::time::Duration {
    let seconds: u64 = interval_secs.parse().unwrap();
    tokio::time::Duration::from_secs(seconds)
}

impl TimerTrigger {
    async fn handle_timer_event(&self, component_id: &str) -> anyhow::Result<()> {
        let (instance, mut store) = self.engine.prepare_instance(component_id).await?;
        let engine = spin_timer::SpinTimer::new(&mut store, &instance, |data| data.as_mut())?;
        engine
            .handle_timer_request(&mut store)
            .await?;
        // println!("Event handler returned {ret}");
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimerTriggerConfig {
    component: String,
    interval_secs: String,
}
