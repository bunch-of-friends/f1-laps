use neon::prelude::*;

pub fn set_bool_prop<'a>(
    cx: &mut FunctionContext<'a>,
    object: &Handle<'a, JsObject>,
    key: &str,
    value: bool,
) -> NeonResult<bool> {
    let boolean = cx.boolean(value);
    object.set(cx, key, boolean)
}

pub fn set_num_prop<'a>(
    cx: &mut FunctionContext<'a>,
    object: &Handle<'a, JsObject>,
    key: &str,
    value: f64,
) -> NeonResult<bool> {
    let number = cx.number(value);
    object.set(cx, key, number)
}

pub fn set_str_prop<'a>(
    cx: &mut FunctionContext<'a>,
    object: &Handle<'a, JsObject>,
    key: &str,
    value: &str,
) -> NeonResult<bool> {
    let number = cx.string(value);
    object.set(cx, key, number)
}

pub fn set_obj_prop<'a>(
    cx: &mut FunctionContext<'a>,
    object: &Handle<'a, JsObject>,
    key: &str,
    value: NeonResult<Handle<'a, JsObject>>,
) -> NeonResult<bool> {
    if let Ok(prop_obj) = value {
        object.set(cx, key, prop_obj)
    } else {
        Ok(false)
    }
}
