pub static PROJECT_TY_BLUEPRINT: Lazy<TyBlueprint> = Lazy::new(|| TyBlueprint {
    properties: vec![TyProperty {
        name: String::from("id"),
        ty: Ty::U32,
        is_mut: false,
    }],
    methods: vec![],
    indexable_by: vec![],
});
