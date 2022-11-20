use crate::settings::Settings;
use crate::utils::process::Process;
use crate::v2ray::config::Config;

pub struct Context<'a, 'b> {
    pub settings: &'a mut Settings,
    pub config: &'a mut Config,
    pub process: &'a mut Process<'b>,
}
