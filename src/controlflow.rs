/// Control flow of the Manager.
/// # Example
/// 
/// ```ignore
/// let mut manager = Manager::new(WindowBuilder::default());
/// 
/// manager.run(|events, control_flow, _| {
///     match events => {
///         Events::WindowEvent { id: _, event } match event {
///             WindowEvents::Close => *control_flow = ControlFlow::Exit,
///             _=> {}
///         }
///         _=> {}
///     }
/// });
/// ```
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum ControlFlow {
    /// Do not do anything
    #[default]
    Continue,
    /// Exit the program
    Exit,
    /// Exit the program and panic
    ExitWithCode(u32),
}