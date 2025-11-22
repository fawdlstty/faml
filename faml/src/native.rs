use crate::FamlValue;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{LazyLock, Mutex};

pub trait FasCallable: Send + Sync {
    fn call(&self, args: Vec<FamlValue>) -> FamlValue;
    fn clone_(&self) -> Box<dyn FasCallable + 'static>;
    //fn to_fas_value(self, func_name: String) -> FamlValue;
    fn get_arg_count(&self) -> usize;
    //fn get_type(&self) -> AstType;
}

impl std::fmt::Debug for dyn FasCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FasCallable")
    }
}

pub trait FasToWrapper<T> {
    type Output: FasCallable + 'static;
    fn convert(self) -> Self::Output;
}

//

macro_rules! calc_arg_count {
	()=>{
		0
	};
	($id_eat:ident $($id:ident)*) => {
		1 + calc_arg_count!($($id)*)
	};
}

macro_rules! FasWrapper {
    ($sname:ident) => {
        pub struct $sname<T: Fn() -> R, R>(T, PhantomData<R>);

        impl<T, R> FasCallable for $sname<T, R>
        where
            T: Fn() -> R + Clone + Send + Sync + 'static,
            R: Into<FamlValue>  + Send + Sync + 'static,
        {
            fn call(&self, _args: Vec<FamlValue>) -> FamlValue {
                let r = (self.0)();
                r.into()
            }

            fn clone_(&self) -> Box<dyn FasCallable + 'static> {
                let copy = Self(self.0.clone(), PhantomData);
                Box::new(copy)
            }

            // fn to_fas_value(self, _func_name: String) -> FamlValue {
            //     // let native_func = AstNativeFunc {
            //     //     ret_type: R::get_ast_type(),
            //     //     name: func_name,
            //     //     arg_types: vec![],
            //     //     func_impl: Box::new(self),
            //     // };
            //     FamlValue::Func(Box::new(AstFuncExpr::new(AstFunc::NativeFunc(
            //         Box::new(self),
            //     ))))
            // }

            fn get_arg_count(&self) -> usize {
                0
            }

            // fn get_type(&self) -> AstType {
            //     AstType::Func((Box::new(R::get_ast_type()), vec![]))
            // }
        }

        impl<T, R> FasToWrapper<R> for T
        where
            T: Fn() -> R + Clone + Send + Sync + 'static,
            R: Into<FamlValue>  + Send + Sync + 'static,
        {
            type Output = $sname<T,  R>;

            fn convert(self) -> Self::Output {
                $sname(self, PhantomData)
            }
        }
    };
    ($sname:ident $($name1:ident) *) => {
        pub struct $sname<T: Fn($($name1),*) -> R, $($name1),*, R>(T, PhantomData<($($name1),*, R)>);

        impl<T, $($name1), * ,R> FasCallable for $sname<T, $($name1),*, R>
        where
            T: Fn($($name1), *) -> R + Clone + Send + Sync + 'static,
            $($name1: From<FamlValue> + Send + Sync + 'static),*,
            R: Into<FamlValue> + Send + Sync + 'static,
        {
			#[allow(non_snake_case)]
            fn call(&self, args: Vec<FamlValue>) -> FamlValue {
				let r = {
					let mut index = 0;
					$(let $name1 = $name1::from(args[index].clone());index+=1;)*
					let _ = index;
					(self.0)($($name1),*)
				};
                r.into()
            }

            fn clone_(&self) -> Box<dyn FasCallable + 'static> {
                let copy = Self(self.0.clone(), PhantomData);
                Box::new(copy)
            }

            // fn to_fas_value(self, _func_name: String) -> FamlValue {
            //     // let native_func = AstNativeFunc {
            //     //     ret_type: R::get_ast_type(),
            //     //     name: func_name,
            //     //     arg_types: vec![$($name1::get_ast_type(),)*],
            //     //     func_impl: Box::new(self),
            //     // };
            //     FamlValue::Func(Box::new(AstFuncExpr::new(AstFunc::NativeFunc(
            //         Box::new(self),
            //     ))))
            // }

            fn get_arg_count(&self) -> usize {
				calc_arg_count!($($name1)*)
            }

            // fn get_type(&self) -> AstType {
            //     AstType::Func((Box::new(R::get_ast_type()), vec![
            //         $($name1::get_ast_type()),*
            //     ]))
            // }
        }

        impl<T, $($name1,)* R> FasToWrapper<($($name1,)* R)> for T
        where
            T: Fn($($name1,)*) -> R + Clone + Send + Sync + 'static,
            $($name1: From<FamlValue> + Send + Sync + 'static,)*
            R: Into<FamlValue> + Send + Sync + 'static,
        {
            type Output = $sname<T, $($name1,)* R>;

            fn convert(self) -> Self::Output {
                $sname(self, PhantomData)
            }
        }

        // impl<T, $($name1), * ,R> Clone for $sname<T, $($name1),*, R>
        // where
        //     T: Fn($($name1,)*) -> R + Clone + Send + Sync + 'static,
        //     $($name1: From<FamlValue> + Send + Sync + 'static,)*
        //     R: Into<FamlValue> + Send + Sync + 'static,
        // {
        //     fn clone(&self) -> Self {
        //         Self(self.0.clone(), PhantomData)
        //     }
        // }
    };
}

FasWrapper!(FuncWrapper0);
FasWrapper!(FuncWrapper1 T0);
FasWrapper!(FuncWrapper2 T0 T1);
FasWrapper!(FuncWrapper3 T0 T1 T2);
FasWrapper!(FuncWrapper4 T0 T1 T2 T3);
FasWrapper!(FuncWrapper5 T0 T1 T2 T3 T4);
FasWrapper!(FuncWrapper6 T0 T1 T2 T3 T4 T5);
FasWrapper!(FuncWrapper7 T0 T1 T2 T3 T4 T5 T6 );
FasWrapper!(FuncWrapper8 T0 T1 T2 T3 T4 T5 T6 T7);
FasWrapper!(FuncWrapper9 T0 T1 T2 T3 T4 T5 T6 T7 T8);

impl Clone for Box<dyn FasCallable + 'static> {
    fn clone(&self) -> Self {
        self.clone_()
    }
}

pub struct Native {}

impl Native {
    pub fn add_func<T: FasToWrapper<U> + 'static, U>(name: impl Into<String>, func: T) {
        let func: Box<dyn FasCallable + 'static> = Box::new(func.convert());
        if let Ok(mut funcs) = NATIVE_FUNCS.lock() {
            funcs.insert(name.into(), func);
        }
    }

    pub fn get_func(name: &str) -> Option<Box<dyn FasCallable + 'static>> {
        if let Ok(funcs) = NATIVE_FUNCS.lock() {
            funcs.get(name).cloned()
        } else {
            None
        }
    }
}

pub static NATIVE_FUNCS: LazyLock<Mutex<HashMap<String, Box<dyn FasCallable + 'static>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
