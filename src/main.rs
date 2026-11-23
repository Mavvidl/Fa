
use fafs::config::FaFsConfig;
use fafs::fractal::hilbert::HilbertCurve;
use fafs::fractal::Point2D;

use fafs::storage::block_device::BlockDevice;
use fafs::storage::memory::MemoryDevice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("================================");
    println!("       FaFs - Fractal FS");
    println!("================================");

    let config = FaFsConfig::default();
    config.validate()?;

    println!("Block size: {} bytes", config.block_size);
    println!("Fractal depth: {}", config.fractal_depth);

    let curve = HilbertCurve::new(config.fractal_depth)?;

    let point = Point2D { x: 2, y: 3 };
    let distance = curve.encode(point)?;
    let decoded = curve.decode(distance)?;

    println!("\nHilbert curve:");
    println!("Original point: {:?}", point);
    println!("Encoded address: {}", distance);
    println!("Decoded point: {:?}", decoded);

    let device = MemoryDevice::new(
        config.block_size,
        1024,
    )?;

    let data = vec![42u8; config.block_size as usize];

    device.write_block(0, &data)?;

    let mut buffer = vec![0u8; config.block_size as usize];

    device.read_block(0, &mut buffer)?;

    println!("\nMemory storage:");
    println!("Blocks: {}", device.block_count());
    println!("Read successful: {}", buffer == data);

    println!("\nFaFs initialized successfully.");

    Ok(())
}

