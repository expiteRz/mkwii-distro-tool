use mkwii_distro_tool_config::Config;

// Define screen states
#[derive(Debug, Default)]
pub enum Workspace {
    #[default]
    None,
    Loading, // Show while parsing Config.pul
    Config(Config),
}
