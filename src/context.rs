use crate::settings::Settings;
use crate::v2ray::config::Config;
use crate::v2ray::process::Process;

pub struct Context<'a> {
    pub settings: &'a mut Settings,
    pub config: &'a mut Config,
    pub process: &'a mut Process,
}
