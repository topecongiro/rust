//! An interpreter for MIR used in CTFE and by miri

mod cast;
mod const_eval;
mod eval_context;
mod place;
mod machine;
mod memory;
mod operator;
mod step;
mod terminator;
mod traits;

pub use self::eval_context::{EvalContext, Frame, ResourceLimits, StackPopCleanup, TyAndPacked,
                             ValTy};

pub use self::place::{Place, PlaceExtra};

pub use self::memory::{HasMemory, Memory, MemoryKind};

pub use self::const_eval::{const_eval_provider, eval_body, eval_body_as_integer,
                           CompileTimeEvaluator};

pub use self::machine::Machine;
