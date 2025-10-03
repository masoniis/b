use std::fmt::Debug;
use std::hash::Hash;

// Trait to bundle all the necessary derives needed for state attributes
pub trait State: Send + Sync + 'static + Copy + Clone + Eq + Hash + Debug + Default {}
