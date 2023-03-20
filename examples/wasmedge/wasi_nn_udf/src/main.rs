use libsql_bindgen::*;
use wasi_nn::NnErrno;
mod imagenet_classes;

fn main() {
    let mut args = std::env::args().into_iter();
    let _ = args.next();
    let input = args.next().unwrap();
    let output = args.next().unwrap();

    let image = std::fs::read(&input).unwrap();
    let tensor_data = image_to_pytorch_tensor(&image, 224, 224).unwrap();
    let tensor_ptr = tensor_data.as_ptr() as *const u8;
    let tensor_len = tensor_data.len() * std::mem::size_of::<f32>();
    let tensor_data = unsafe { std::slice::from_raw_parts(tensor_ptr, tensor_len) };
    std::fs::write(&output, tensor_data).unwrap();
}

#[derive(Debug, PartialEq)]
struct InferenceResult(usize, f32);

#[libsql_bindgen::libsql_bindgen]
fn classify(tensor: &mut [u8]) -> String {
    if tensor.len() != 602112 {
        return format!("err: tensor data length error");
    }
    match pytorch_classify(&tensor) {
        Ok(s) => s,
        Err(e) => format!("err: {:?}", e),
    }
}

fn image_to_pytorch_tensor(buffer: &[u8], height: u32, width: u32) -> image::ImageResult<Vec<f32>> {
    let img = image::load_from_memory(&buffer)?.to_rgb32f();
    let resized =
        image::imageops::resize(&img, height, width, ::image::imageops::FilterType::Triangle);

    let mut flat_img = resized.to_vec();
    let flat_img_len = flat_img.len() / 3;

    for (i, rgb) in resized.pixels().enumerate() {
        flat_img[i] = (rgb.0[0] - 0.485) / 0.229;
        flat_img[i + flat_img_len] = (rgb.0[1] - 0.456) / 0.224;
        flat_img[i + flat_img_len * 2] = (rgb.0[2] - 0.406) / 0.225;
    }

    Ok(flat_img)
}

fn sort_results(buffer: &[f32]) -> Vec<InferenceResult> {
    let mut results: Vec<InferenceResult> = buffer
        .iter()
        .enumerate()
        .map(|(c, p)| InferenceResult(c, *p))
        .collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

fn pytorch_classify(tensor_data: &[u8]) -> Result<String, NnErrno> {
    let weights = include_bytes!("../mobilenet.pt");
    unsafe {
        let graph = wasi_nn::load(
            &[weights],
            wasi_nn::GRAPH_ENCODING_PYTORCH,
            wasi_nn::EXECUTION_TARGET_CPU,
        )?;

        let context = wasi_nn::init_execution_context(graph)?;
        let tensor = wasi_nn::Tensor {
            dimensions: &[1, 3, 224, 224],
            type_: wasi_nn::TENSOR_TYPE_F32,
            data: &tensor_data,
        };

        wasi_nn::set_input(context, 0, tensor)?;
        wasi_nn::compute(context)?;
        let mut output_buffer = vec![0f32; 1000];
        wasi_nn::get_output(
            context,
            0,
            &mut output_buffer[..] as *mut [f32] as *mut u8,
            (output_buffer.len() * std::mem::size_of::<f32>()) as u32,
        )?;

        let results = sort_results(&output_buffer);
        Ok(imagenet_classes::IMAGENET_CLASSES[results[0].0].to_string())
    }
}
