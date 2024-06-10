use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the default host
    let host = cpal::default_host();

    // Get the default input device
    let device = host
        .default_input_device()
        .expect("Failed to find a default input device");

    // Get the input stream configuration
    let supported_config = device
        .default_input_config()
        .expect("Failed to get default input format");

    let config = supported_config.config();

    println!("Input format: {:?}", config);

    // Define the input stream and callback
    let err_fn = |err| eprintln!("an error occurred on the input audio stream: {}", err);

    // Initialize the input stream
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Print audio data
            for &sample in data {
                print!("{}, ", sample);
            }
        },
        err_fn,
        None,
    )?;

    // Start the stream
    stream.play()?;

    println!("Recording... Press Ctrl+C to stop.");

    // Keep the program running to continue recording
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
