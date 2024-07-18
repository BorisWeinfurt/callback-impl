use std::future::Future;
use futures::future::BoxFuture;

type AsyncCallback<Args, Output> = Box<dyn Callable<Args, Output = BoxFuture<'static, Output>> + Send + Sync>;

trait Callable<Args> {
    type Output;

    // Calls the function with the given arguments and returns the output
    fn call(&self, args: Args) -> Self::Output;
}

// Macro to implement Callable trait for different numbers of parameters
macro_rules! impl_callable {
    ($($param:ident),*) => {
        impl<Func, Fut, $($param,)* Output> Callable<($($param,)*)> for Func
        where
        Func: Fn($($param),*) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Output> + Send + 'static,
            $($param: Send + 'static,)*
        {
            type Output = BoxFuture<'static, Output>;

            fn call(&self, args: ($($param,)*)) -> Self::Output {
                let ($($param,)*) = args;
                Box::pin((self)($($param,)*))
            }
        }
    };
}

macro_rules! generate_impl_callable {
    ($($t:ident),*) => {
        impl_callable!();
        generate_impl_callable!(@recurse $($t),*);
    };

    (@recurse $head:ident $(, $tail:ident)*) => {
        impl_callable!($head $(, $tail)*);
        generate_impl_callable!(@recurse $($tail),*);
    };

    (@recurse) => {};
}

// Callbacks can (currently) have up to 10 parameters
generate_impl_callable!(A,B,C,D,E,F,G,H,I,J);


pub struct Callback<Args, Output> {
    callback: AsyncCallback<Args, Output>,
}

impl<Args, Output> Callback<Args, Output> {
    pub fn new<Func>(callback: Func) -> Self
    where
    Func: Callable<Args, Output = BoxFuture<'static, Output>> + Send + Sync + 'static,
    {
        let callback: AsyncCallback<Args, Output> = Box::new(callback);
        Self { callback }
    }

    // Calls the wrapped callback with the given arguments
    pub async fn call(&self, args: Args) -> Output {
        self.callback.call(args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn async_no_arg() -> i32 {
        println!("Called async_no_arg");
        return 10;
    }

    async fn async_one_arg(x: i32) -> String {
        println!("Called async_one_arg with {}", x);
        return String::from("return string");
    }

    async fn async_two_args(x: i32, y: String) {
        println!("Called async_two_args with {}, {}", x, y);
    }

    async fn async_three_args(x: i32, y: String, z: f64) {
        println!("Called async_three_args with {}, {}, {}", x, y, z);
    }

    #[tokio::test]
    async fn test_callback_no_args() {
        let callback = Callback::new(async_no_arg);
        let res = callback.call(()).await;
        println!("{}", res);
    }

    #[tokio::test]
    async fn test_callback_one_arg() {
        let callback = Callback::new(async_one_arg);
        let res = callback.call((42, )).await;
        println!("{}", res);
    }

    #[tokio::test]
    async fn test_callback_two_args() {
        let callback = Callback::new(async_two_args);
        callback.call((42, "hello".to_string())).await;
    }

    #[tokio::test]
    async fn test_callback_three_args() {
        let callback = Callback::new(async_three_args);
        callback.call((42, "hello".to_string(), 3.14)).await;
    }
}
