use tract_ndarray::Array2;
use tract_onnx::prelude::*;

fn main() {
    let model = tract_onnx::onnx()
        .model_for_path("assets/g1-control-direction.onnx")
        .unwrap()
        .with_input_fact(0, f32::fact([1, 121]).into())
        .unwrap()
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap();

    let zeros: Vec<f32> = (0..121).map(|_| 0.0).collect();
    let input_array = Array2::from_shape_vec((1, 121), zeros).unwrap();
    let input: Tensor = input_array.into();
    let result = model.run(tvec!(input.into())).unwrap();
    println!(
        "Result of neural net invocation: {:?}",
        result[0].to_array_view::<f32>().unwrap()
    );
}
