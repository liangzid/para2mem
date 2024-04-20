use std::collections::HashMap;

fn para_to_memory(amount: f64, unit_para: &str, is_use_f32: bool, unit_memory: &str) -> f64 {
    let amount_unit_map: HashMap<&str, f64> =
        HashMap::from([("Billion", 1000_000_000 as f64),
		       ("Million", 1000_000 as f64),
		       ("Trillion", 1000_000_000_000.0 as f64),
		       ("Unit", 1 as f64),
	]);
    let unit_divide_map: HashMap<&str, f64> = HashMap::from([
        ("TB", 1024.0 * 1024.0 * 1024.0 * 1024.0),
        ("GB", 1024.0 * 1024.0 * 1024.0),
        ("MB", 1024.0 * 1024.0),
        ("KB", 1024.0),
        ("Byte", 1.0),
    ]);

    if let Some(para_amount) = amount_unit_map.get(unit_para) {
        // transfer to Byte.
        let byte_num = {
            if is_use_f32 {
                amount * 4.0 * para_amount
            } else {
                amount * 2.0 * para_amount
            }
        };

        if let Some(unit_memory) = unit_divide_map.get(unit_memory) {
            byte_num / unit_memory
        } else {
            -1.0
        }
    } else {
        -1.0
    }
}

fn memory_to_GPU(memory_gb: f64, gpu_type: &str) -> i64 {
    let gpu_cap_map: HashMap<&str, f64> = HashMap::from([
        ("4090", 24.0),
        ("3090", 24.0),
        ("4080", 16.0),
        ("V100", 32.0),
        ("A100", 80.0),
        ("H800", 80.0),
    ]);

    if let Some(cap) = gpu_cap_map.get(gpu_type) {
        (memory_gb / cap).ceil() as i64
    } else {
           -1 
    }
}

pub fn para_to_GPU_api(
    amount: f64,
    unit_para: &str,
    is_use_f32: bool,
    unit_memory: &str,
    gpu_type: &str,
) -> (f64, i64) {
    let memory = para_to_memory(amount, unit_para, is_use_f32, unit_memory);
    if memory + 1.0 < 0.005 {
        (-1.0, -1)
    } else {
        (memory, memory_to_GPU(memory, gpu_type))
    }
}

fn main() {
    let res1 = para_to_GPU_api(1.2, "B", true, "GB", "4090");
    println!("{:?}", res1);

    let res1 = para_to_GPU_api(6.9, "B", true, "GB", "4090");
    println!("{:?}", res1);

    let res1 = para_to_GPU_api(13.0, "B", true, "GB", "4090");
    println!("{:?}", res1);

    let res1 = para_to_GPU_api(13.0, "B", true, "GB", "A100");
    println!("{:?}", res1);
}
