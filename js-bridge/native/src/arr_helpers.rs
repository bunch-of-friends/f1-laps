use neon::prelude::*;

pub fn set_num_prop<'a>(
    cx: &mut FunctionContext<'a>,
    arr: &Handle<'a, JsArray>,
    key: u32,
    value: f64,
) -> NeonResult<bool> {
    let number = cx.number(value);
    arr.set(cx, key, number)
}

pub fn set_obj_prop<'a>(
    cx: &mut FunctionContext<'a>,
    object: &Handle<'a, JsArray>,
    key: u32,
    value: NeonResult<Handle<'a, JsObject>>,
) -> NeonResult<bool> {
    if let Ok(prop_obj) = value {
        object.set(cx, key, prop_obj)
    } else {
        Ok(false)
    }
}