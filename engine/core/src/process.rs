use crate::{error::EngineError, launch::LaunchPlan};

pub trait ProcessManager { fn validate_launch(&self, plan: &LaunchPlan) -> Result<(), EngineError>; }

pub struct DefaultProcessManager;
impl ProcessManager for DefaultProcessManager { fn validate_launch(&self, plan: &LaunchPlan) -> Result<(), EngineError> { plan.validate() } }
