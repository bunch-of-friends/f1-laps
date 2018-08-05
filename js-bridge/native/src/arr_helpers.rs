use neon::prelude::*;

pub fn set_num_item<'a>(
    cx: &mut FunctionContext<'a>,
    arr: &Handle<'a, JsArray>,
    key: u32,
    value: f64,
) -> NeonResult<bool> {
    let number = cx.number(value);
    arr.set(cx, key, number)
}

pub fn set_obj_item<'a>(
    cx: &mut FunctionContext<'a>,
    arr: &Handle<'a, JsArray>,
    key: u32,
    value: NeonResult<Handle<'a, JsObject>>,
) -> NeonResult<bool> {
    if let Ok(prop_obj) = value {
        arr.set(cx, key, prop_obj)
    } else {
        Ok(false)
    }
}