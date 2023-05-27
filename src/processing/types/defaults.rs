#[macro_export]
macro_rules! default_type_wrapper_struct_and_impl {
    ($wrapper_name: ident, $type_name: ident) => {
        pub struct $wrapper_name {}

        impl crate::processing::types::UninstantiatedType for $wrapper_name {
            fn instantiate(&self) -> Box<dyn crate::processing::types::Type> {
                Box::new($type_name::new())
            }

            fn get_type_symbol(&self) -> crate::processing::symbols::TypeSymbol {
                TypeSymbol::Boolean
            }
        }
    };
}

#[macro_export]
macro_rules! default_type_struct {
    ($type_name: ident) => {
        #[allow(dead_code)]
        pub struct $type_name {
            operators: Vec<Box<dyn crate::processing::types::Operation<$type_name>>>,
            address: Option<crate::address::Address>
        }
    };
}

#[macro_export]
macro_rules! default_type_initialiser {
    ($type_name: ident $(, $operator: ident)*) => {
        impl $type_name {
            pub fn new() -> Self {
                Self {
                    operators: vec![
                        $(Box::new($operator{})
                    )*
                    ],
                    address: None
                }
            }
        }
    };
}

#[macro_export]
macro_rules! default_get_type_symbol_impl {
    ($type_name: ident, $type_symbol: expr) => {
        fn get_type_symbol(&self) -> crate::processing::symbols::TypeSymbol {
            $type_symbol
        }
    };
}

#[macro_export]
macro_rules! default_type_operate_impl {
    ($type_name: ident) => {
        fn operate(&self, rhs: Box<dyn crate::processing::types::Type>) -> Result<(), String> {
            for operator in self.operators.iter() {
                if operator.get_result_type(Some(rhs.get_type_symbol())).is_some() {
                    return operator.operate(self, rhs);
                }
            }
    
            Err("Operations not found!".to_string())
        }
    };
}
