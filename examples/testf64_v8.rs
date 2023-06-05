fn main() {
    let vector: Vec<f32> = vec![ 0.4, 0.5, 0.3];

    // Convert Vec<f32> to Vec<u8>
    let mut output: Vec<u8> = vector
        .iter()
        .flat_map(|val| val.to_be_bytes().to_vec())
        .collect();

        println!("{:?}", output);
    // Convert Vec<u8> back to Vec<f32>
    let converted_output: Vec<f32> = output
        .chunks_exact(4)
        .map(|bytes| {
            let mut array = [0u8; 4];
            array.copy_from_slice(bytes);
            f32::from_be_bytes(array)
        })
        .collect();

    println!("{:?}", converted_output);
}
