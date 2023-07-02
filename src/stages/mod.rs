pub mod update_check;
pub mod welcome;
pub mod exit_confirmation;

#[derive(Default, Copy, Clone)]
pub enum InstallerStage {
    #[default]
    UpdateCheck,
    Welcome,
    PathSelect,
    Options,
    Progress,
    Complete,
    ExitConfirmation,
    Error
}