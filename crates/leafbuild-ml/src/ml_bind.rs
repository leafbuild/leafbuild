use crate::MiddleLayer;

// See docs of [`linkme`] to know how this works

/// The list of functions that give references to static middle layers
#[distributed_slice]
pub static MIDDLE_LAYERS: [fn() -> &'static dyn MiddleLayer] = [..];

/// Declare a middle layer static structure and add it to `MIDDLE_LAYERS`
#[macro_export]
macro_rules! middle_layer {
    ($name:ident, $ml_static_name:ident) => {
        static $ml_static_name: $name = $name;

        fn ret_ml() -> &'static dyn $crate::MiddleLayer {
            &$ml_static_name
        }

        #[$crate::distributed_slice($crate::MIDDLE_LAYERS)]
        static RET_ML: fn() -> &'static dyn $crate::MiddleLayer = ret_ml;
    };
}
