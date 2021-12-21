use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("createHash", create_hash)?;
    Ok(())
}

pub fn create_hash(mut cx: FunctionContext) -> JsResult<JsValue> {
    let input_buffer = cx.argument::<JsBuffer>(0)?;
    let input_bytes = cx.borrow(&input_buffer, |data| data.as_slice::<u8>());

    let mut hasher = blake3::Hasher::new();
    hasher.update(input_bytes);
    let mut reader = hasher.finalize_xof();
    reader_to_buffer(&mut cx, &mut reader, 32)
}

fn reader_to_buffer<'a, T: neon::object::This>(
    cx: &mut CallContext<'a, T>,
    output_reader: &mut blake3::OutputReader,
    length: u32,
) -> JsResult<'a, JsValue> {
    let mut output_buffer = cx.buffer(length)?;
    cx.borrow_mut(&mut output_buffer, |data| {
        output_reader.fill(data.as_mut_slice());
    });

    Ok(output_buffer.upcast())
}
