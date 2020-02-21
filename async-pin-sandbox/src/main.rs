use futures::prelude::*;
use futures::future::ready;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct NotSend(Rc<()>);

#[derive(Default)]
struct NotSync(RefCell<()>);

#[tokio::main]
async fn main() {
    require_send(async_send());
    // require_send(async_not_send());
    require_send(async_send_expended());
    // require_send(async_not_send_expended());
    require_send(async_send2());
    // require_send(async_not_send2());
    require_send(impl_future_send());
    require_send(impl_future_send2());
    // require_send(impl_future_not_send());
    // require_send(impl_future_not_send2());

    let mut x = 0;
    // require_static(async_fn_with_ref(&x));
    // require_static(async_fn_with_ref_expanded(&x));
    // require_static(async_fn_with_ref_not_static(&x));
    require_static(impl_future_with_ref_static(&x));
    // require_static(async_fn_with_mut_ref(&mut x, 1));
    // require_static(async_fn_with_mut_ref_expanded(&mut x, 1));

    require_sync(async_sync());
    // require_sync(async_not_sync());
    require_sync(impl_future_sync());
    // require_sync(impl_future_not_sync());
}

fn require_send  <T: Send   >(_: T) {}
fn require_static<T: 'static>(_: T) {}
fn require_sync  <T: Sync   >(_: T) {}
async fn async_send             () ->                    ()         {              let a = NotSend::default(); let _ = a;                                 }
async fn async_not_send         () ->                    ()         {              let a = NotSend::default(); let _ = a;   ready(()).await;              }
fn       async_send_expended    () -> impl Future<Output=()>        { async move { let a = NotSend::default(); let _ = a;                               } }
fn       async_not_send_expended() -> impl Future<Output=()>        { async move { let a = NotSend::default(); let _ = a;   ready(()).await;            } }
async fn async_send2            () ->                    ()         {            { let a = NotSend::default();          }   ready(()).await;              }
async fn async_not_send2        () ->                    ()         {              let a = NotSend::default();   drop(a);   ready(()).await;              }
fn       impl_future_send       () -> impl Future<Output=()>        {              let a = NotSend::default(); async move { ready(()).await;            } }
fn       impl_future_send2      () -> impl Future<Output=()>        {              let a = NotSend::default(); async      { ready(()).await;            } }
fn       impl_future_not_send   () -> impl Future<Output=()>        {              let a = NotSend::default(); async move { ready(()).await; let _ = a; } }
fn       impl_future_not_send2  () -> impl Future<Output=()>        { async move { let a = NotSend::default();              ready(()).await;            } }

async fn async_fn_with_ref             <'a, T: Copy>(x: &'a     T      ) ->                      ()            {              ready(()).await; let _ = *x;   }
fn       async_fn_with_ref_expanded    <'a, T: Copy>(x: &'a     T      ) -> impl Future<Output = ()> + 'a      { async move { ready(()).await; let _ = *x; } }
async fn async_fn_with_ref_not_static  <'a, T: Copy>(x: &'a     T      ) ->                      ()            {              ready(()).await;               }
fn       impl_future_with_ref_static   <'a, T: Copy>(x: &'a     T      ) -> impl Future<Output = ()> + 'static { async move { ready(()).await;             } }
async fn async_fn_with_mut_ref         <'a, T: Copy>(x: &'a mut T, y: T) ->                      ()            {              ready(()).await;     *x = y;   }
fn       async_fn_with_mut_ref_expanded<'a, T: Copy>(x: &'a mut T, y: T) -> impl Future<Output = ()> + 'a      { async move { ready(()).await;     *x = y; } }

async fn async_sync          () ->                    ()  {              let a = NotSync::default();                    }
async fn async_not_sync      () ->                    ()  {              let a = NotSync::default(); ready(()).await;   }
fn       impl_future_sync    () -> impl Future<Output=()> { async move { let a = NotSync::default();                  } }
fn       impl_future_not_sync() -> impl Future<Output=()> { async move { let a = NotSync::default(); ready(()).await; } }