pub mod changed_text;
pub use changed_text::update_visible_text_system;

pub mod handle_visibility_changed;
pub use handle_visibility_changed::handle_text_visibility_change_system;

pub mod toggle_debug_diagnostics;
pub use toggle_debug_diagnostics::toggle_debug_diagnostics_system;

pub mod update_debug_diagnostics;
pub use update_debug_diagnostics::update_debug_diagnostics_system;
