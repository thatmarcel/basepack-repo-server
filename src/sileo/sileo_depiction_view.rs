use erased_serde::{Serialize, serialize_trait_object};
use dyn_clone::{clone_trait_object, DynClone};

pub trait SileoDepictionView: Serialize + DynClone {}

serialize_trait_object!(SileoDepictionView);
clone_trait_object!(SileoDepictionView);