pub mod update_check;

#[derive(Default)]
pub enum InstallerStage {
    #[default]
    UpdateCheck,
    Welcome,
    PathSelect,
    Options,
    Progress,
    Complete,
    Error
}