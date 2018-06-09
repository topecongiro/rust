//! An interpreter for MIR used in CTFE and by miri

mod cast;
mod const_eval;
mod eval_context;
mod machine;
mod memory;
mod operator;
mod place;
mod step;
mod terminator;
mod traits;

pub use self::eval_context::{EvalContext, Frame, StackPopCleanup, TyAndPacked, ValTy};

pub use self::place::{Place, PlaceExtra};

pub use self::memory::{HasMemory, Memory, MemoryKind};

pub use self::const_eval::{
    const_eval_provider, const_val_field, const_value_to_allocation_provider, const_variant_index,
    eval_promoted, mk_borrowck_eval_cx, value_to_const_value, CompileTimeEvaluator,
};

pub use self::machine::Machine;

pub use self::memory::{read_target_uint, write_target_int, write_target_uint};

use rustc::mir::interpret::{EvalErrorKind, EvalResult};
use rustc::ty::{ParamEnv, Ty, TyCtxt};

pub fn sign_extend<'a, 'tcx>(
    tcx: TyCtxt<'a, 'tcx, 'tcx>,
    value: u128,
    ty: Ty<'tcx>,
) -> EvalResult<'tcx, u128> {
    let param_env = ParamEnv::empty();
    let layout = tcx
        .layout_of(param_env.and(ty))
        .map_err(|layout| EvalErrorKind::Layout(layout))?;
    let size = layout.size.bits();
    assert!(layout.abi.is_signed());
    // sign extend
    let shift = 128 - size;
    // shift the unsigned value to the left
    // and back to the right as signed (essentially fills with FF on the left)
    Ok((((value << shift) as i128) >> shift) as u128)
}

pub fn truncate<'a, 'tcx>(
    tcx: TyCtxt<'a, 'tcx, 'tcx>,
    value: u128,
    ty: Ty<'tcx>,
) -> EvalResult<'tcx, u128> {
    let param_env = ParamEnv::empty();
    let layout = tcx
        .layout_of(param_env.and(ty))
        .map_err(|layout| EvalErrorKind::Layout(layout))?;
    let size = layout.size.bits();
    let shift = 128 - size;
    // truncate (shift left to drop out leftover values, shift right to fill with zeroes)
    Ok((value << shift) >> shift)
}
