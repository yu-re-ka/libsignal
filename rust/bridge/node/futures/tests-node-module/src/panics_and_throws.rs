//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use neon::prelude::*;
use neon::types::JsPromise;
use signal_neon_futures::*;

#[allow(unreachable_code, unused_variables)]
pub fn panic_pre_await(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let promise = cx.argument::<JsObject>(0)?;

    let future = JsFuture::from_promise(&mut cx, promise, move |cx, result| {
        cx.try_catch(|cx| {
            let value = result.or_else(|e| cx.throw(e))?;
            Ok(value.downcast_or_throw::<JsNumber, _>(cx)?.value(cx))
        })
        .map_err(|e| PersistentException::new(cx, e))
    })?;

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    cx.start_future(async move {
        panic!("check for this");
        let result = future.await;
        channel.settle_with(deferred, move |cx| match result {
            Ok(value) => Ok(cx.undefined()),
            Err(e) => {
                let exception = e.into_inner(cx);
                cx.throw(exception)
            }
        });
    });
    Ok(promise)
}

#[allow(unreachable_code)]
pub fn panic_during_callback(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let promise = cx.argument::<JsObject>(0)?;

    let future: JsFuture<()> = JsFuture::from_promise(&mut cx, promise, move |_cx, _result| {
        panic!("check for this");
    })?;

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    cx.start_future(async move {
        let _ = future.await;
        channel.settle_with(deferred, move |cx| Ok(cx.undefined()));
    });
    Ok(promise)
}

#[allow(unreachable_code)]
pub fn panic_post_await(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let promise = cx.argument::<JsObject>(0)?;

    let future = JsFuture::from_promise(&mut cx, promise, move |cx, result| {
        cx.try_catch(|cx| {
            let value = result.or_else(|e| cx.throw(e))?;
            Ok(value.downcast_or_throw::<JsNumber, _>(cx)?.value(cx))
        })
        .map_err(|e| PersistentException::new(cx, e))
    })?;

    let (_deferred, promise) = cx.promise();
    cx.start_future(async move {
        let _ = future.await;
        panic!("check for this");
    });
    Ok(promise)
}

#[allow(unreachable_code, unused_variables)]
pub fn panic_during_settle(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let promise = cx.argument::<JsObject>(0)?;

    let future = JsFuture::from_promise(&mut cx, promise, move |cx, result| {
        cx.try_catch(|cx| {
            let value = result.or_else(|e| cx.throw(e))?;
            Ok(value.downcast_or_throw::<JsNumber, _>(cx)?.value(cx))
        })
        .map_err(|e| PersistentException::new(cx, e))
    })?;

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    cx.start_future(async move {
        let result = future.await;
        channel.settle_with::<JsUndefined, _>(deferred, move |cx| panic!("check for this"));
    });
    Ok(promise)
}

pub fn throw_during_settle(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let promise = cx.argument::<JsObject>(0)?;

    let future = JsFuture::from_promise(&mut cx, promise, move |cx, result| {
        cx.try_catch(|cx| {
            let value = result.or_else(|e| cx.throw(e))?;
            Ok(value.downcast_or_throw::<JsNumber, _>(cx)?.value(cx))
        })
        .map_err(|e| PersistentException::new(cx, e))
    })?;

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    cx.start_future(async move {
        let _ = future.await;
        channel.settle_with(deferred, move |cx| {
            cx.throw_error("check for this")?;
            Ok(cx.undefined())
        });
    });
    Ok(promise)
}
