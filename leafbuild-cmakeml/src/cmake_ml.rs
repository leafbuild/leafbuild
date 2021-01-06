use leafbuild_core::lf_buildsys::LfBuildsys;
use leafbuild_ml::{
    BuildsysBoundaryDetails, BuildsysChanges, MiddleLayer, RecognizeResult, Result,
};
use std::path::Path;

leafbuild_ml::middle_layer! {CMakeMiddleLayer, CMAKE_ML}

struct CMakeMiddleLayer;

impl MiddleLayer for CMakeMiddleLayer {
    fn recognize(&self, path: &Path) -> RecognizeResult {
        unimplemented!()
    }

    fn handle<'buildsys>(
        &self,
        buildsys: &'buildsys LfBuildsys<'buildsys>,
        boundary_details: BuildsysBoundaryDetails,
    ) -> Result<BuildsysChanges> {
        unimplemented!()
    }
}
