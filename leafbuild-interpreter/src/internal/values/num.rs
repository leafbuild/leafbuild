/// Creates `impl Value` for numeric types
///
/// $name = name of the number newtype
///
/// $value_type = name of the [`ValueType`] enum value
#[macro_export]
macro_rules! impl_value_num {
    ($name:ident, $value_type:ident) => {
        impl<'a> Value<'a> for $name {
            fn get_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn get_property(
                &self,
                this_span: Span,
                dot_span: Span,
                property_name: &str,
                property_name_span: Span,
            ) -> Result<&'a dyn Value<'a>, GetPropertyError> {
                Err(GetPropertyError::NoSuchProperty {
                    root_type: ValueType::$value_type,
                    root_span: this_span,
                    dot_span,
                    name: property_name.to_string(),
                    name_span: property_name_span,
                })
            }

            fn get_property_mut(
                &mut self,
                this_span: Span,
                dot_span: Span,
                property_name: &str,
                property_name_span: Span,
            ) -> Result<&'a mut dyn Value<'a>, GetPropertyError> {
                Err(GetPropertyError::NoSuchProperty {
                    root_type: ValueType::$value_type,
                    root_span: this_span,
                    dot_span,
                    name: property_name.to_string(),
                    name_span: property_name_span,
                })
            }

            fn get_indexed(
                &self,
                this_span: Span,
                left_brace: Span,
                index_value: &dyn Value,
                right_brace: Span,
            ) -> Result<&'a dyn Value<'a>, GetIndexedError> {
                Err(GetIndexedError::TypeCannotBeIndexed {
                    root_type: ValueType::$value_type,
                    root_span: this_span,

                    lbrace_span: left_brace,
                    index_type: index_value.get_type(),
                    rbrace_span: right_brace,
                })
            }

            fn get_indexed_mut(
                &mut self,
                this_span: Span,
                left_brace: Span,
                index_value: &dyn Value,
                right_brace: Span,
            ) -> Result<&'a mut dyn Value<'a>, GetIndexedError> {
                Err(GetIndexedError::TypeCannotBeIndexed {
                    root_type: ValueType::$value_type,
                    root_span: this_span,

                    lbrace_span: left_brace,
                    index_type: index_value.get_type(),
                    rbrace_span: right_brace,
                })
            }

            fn invoke_method(
                &self,
                this_span: Span,
                dot_span: Span,
                method_name: &str,
                method_name_span: Span,
            ) -> Result<Box<dyn Value<'static>>, InvokeMethodError> {
                match method_name {
                    "print" => {
                        println!("{}", self.0);
                        Ok(Box::new(Self(0)))
                    }
                    _ => Err(InvokeMethodError::NoSuchMethod {
                        root_type: ValueType::$value_type,
                        root_span: this_span,
                        dot_span,
                        name: method_name.to_string(),
                        name_span: method_name_span,
                    }),
                }
            }
        }
    };
}
