use crate::utils::process::Process;
use crate::v2ray::config::Config;
use crate::workdir;
use crate::workdir::servers::Servers;
use crate::workdir::settings::Settings;

pub struct Context<'a, 'b> {
    pub dir: &'a workdir::dir::Dir,
    pub settings: &'a mut Settings,
    pub servers: &'a mut Servers,
    pub config: &'a mut Config,
    pub v2ray_process: &'a mut Process<'b>,
    pub worker_process: &'a mut Process<'b>,
}
