use crate::utils::process::Process;
use crate::v2ray::config::Config;
use crate::workdir;
use crate::workdir::servers::Servers;
use crate::workdir::settings::Settings;

pub struct Context<'a> {
    pub dir: &'a workdir::dir::Dir,
    pub settings: &'a mut Settings,
    pub servers: &'a mut Servers,
    pub config: &'a mut Config,
    pub process: &'a mut Process<'a>,
}
