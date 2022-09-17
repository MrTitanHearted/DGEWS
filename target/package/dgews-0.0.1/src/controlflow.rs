#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum ControlFlow {
    #[default]
    Continue,
    Exit,
    ExitWithCode(u32),
}