//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use futures_util::try_join;
use neon::prelude::*;
use neon::types::JsPromise;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use signal_neon_futures::*;

struct NameStore {
    js_channel: Channel,
    store_object: Arc<Root<JsObject>>,
}

impl NameStore {
    fn new<'a>(cx: &mut FunctionContext<'a>, store: Handle<'a, JsObject>) -> Self {
        Self {
            js_channel: cx.channel(),
            store_object: Arc::new(store.root(cx)),
        }
    }

    async fn get_name(&self) -> Result<String, String> {
        let store_object_shared = self.store_object.clone();
        JsFuture::get_promise(&self.js_channel, move |cx| {
            let store_object = store_object_shared.to_inner(cx);
            let result = call_method(cx, store_object, "getName", std::iter::empty())?
                .downcast_or_throw(cx)?;
            store_object_shared.finalize(cx);
            Ok(result)
        })
        .then(|cx, result| match result {
            Ok(value) => match value.downcast::<JsString, _>(cx) {
                Ok(s) => Ok(s.value(cx)),
                Err(_) => Err("name must be a string".into()),
            },
            Err(error) => Err(error
                .to_string(cx)
                .expect("can convert to string")
                .value(cx)),
        })
        .await
    }
}

impl Finalize for NameStore {
    fn finalize<'a, C: Context<'a>>(self, cx: &mut C) {
        self.store_object.finalize(cx)
    }
}

async fn double_name_from_store_impl(store: &mut NameStore) -> Result<String, String> {
    Ok(format!(
        "{0} {1}",
        store.get_name().await?,
        store.get_name().await?
    ))
}

// function doubleNameFromStore(store: { getName: () => Promise<string> }): Promise<string>
pub fn double_name_from_store(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let js_store = cx.argument(0)?;
    let mut store = NameStore::new(&mut cx, js_store);

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    cx.start_future(async move {
        let future = AssertUnwindSafe(double_name_from_store_impl(&mut store));
        let result = future.await;
        channel.settle_with(deferred, move |cx| {
            store.finalize(cx);
            match result {
                Ok(doubled) => Ok(cx.string(doubled)),
                Err(message) => cx.throw_error(format!("rejected: {}", message)),
            }
        });
    });
    Ok(promise)
}

async fn double_name_from_store_using_join_impl(store: &mut NameStore) -> Result<String, String> {
    let names = try_join!(store.get_name(), store.get_name())?;
    Ok(format!("{0} {1}", names.0, names.1))
}

// function doubleNameFromStoreUsingJoin(store: { getName: () => Promise<string> }): Promise<string>
pub fn double_name_from_store_using_join(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let js_store = cx.argument(0)?;
    let mut store = NameStore::new(&mut cx, js_store);

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    cx.start_future(async move {
        let future = AssertUnwindSafe(double_name_from_store_using_join_impl(&mut store));
        let result = future.await;
        channel.settle_with(deferred, move |cx| {
            store.finalize(cx);
            match result {
                Ok(doubled) => Ok(cx.string(doubled)),
                Err(message) => cx.throw_error(format!("rejected: {}", message)),
            }
        });
    });
    Ok(promise)
}
