macro_rules! define_ast {
    ($base_name:ident, $(($class_name:ident, $($field_name:ident: $field_type:ty),*)),*) => {

        pub trait Visitor<T> {
        $(
            paste::item! {
            fn [< visit_ $class_name:lower _ $base_name:lower >](&mut self, expr: &$class_name) -> T;
            }
        )*
        }

        pub enum $base_name {
        $(
            $class_name($class_name),
        )*
        }

        $(
        pub struct $class_name {
            $(pub $field_name: $field_type),*
        }

        impl $class_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }
        )*

        impl $base_name {
            pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
                paste::item! {
                match self {
                    $(
                        $base_name::$class_name(ref [< $base_name:snake >]) => {
                            visitor.[< visit_ $class_name:snake _ $base_name:snake >]([< $base_name:snake >])
                        },
                    )*
                }
                }
            }
        }

    };
}

use super::token::{Object, Token};

define_ast!(
    Expr,
    (
        Binary,
        left: Box<Expr>, operator: Token, right: Box<Expr>
    ),
    (Grouping, expression: Box<Expr>),
    (Literal, value: Object),
    (Unary, operator: Token, right: Box<Expr>)
);
