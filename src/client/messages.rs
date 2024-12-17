pub mod error;
pub mod get_model_config;
pub mod get_pilot;
pub mod get_power;
pub mod get_system_config;
pub mod set_pilot;

pub trait SetResponse {
    fn success(&self) -> bool;
}
