mod body;
mod color;
mod frame;
mod masked_color;
mod reader;
mod stream;
mod stream_set;
mod types;
pub use body::*;
pub use color::*;
pub use frame::*;
pub use masked_color::*;
pub use reader::*;
pub use stream::*;
pub use stream_set::*;
pub use types::*;

use crate::util::astra_status_to_result;
use model::Result;

pub fn update() -> Result<()> {
    unsafe { astra_status_to_result(sys::astra_update().into(), ()) }
}

pub fn init() -> Result<()> {
    unsafe { astra_status_to_result(sys::astra_initialize().into(), ()) }
}

pub fn terminate() -> Result<()> {
    unsafe { astra_status_to_result(sys::astra_terminate().into(), ()) }
}
