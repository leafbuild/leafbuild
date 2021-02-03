pub struct Module {}

#[derive(Debug, Clone)]
pub struct ModuleTy;

static MODULE_TY_BLUEPRINT: Lazy<TyBlueprint> = Lazy::new(|| TyBlueprint {
    properties: vec![TyProperty {
        name: String::from("id"),
        ty: Ty::U32,
        is_mut: false,
    }],
    indexable_by: vec![],
    methods: vec![],
});
