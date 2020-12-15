use std::time::SystemTime;

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
    as u64
}

fn randomize_string(input: &'static str) -> String {
    format!("{0}{1}", input, current_time())
}

#[cfg(test)]
mod controller_test;

#[cfg(test)]
mod dao_test;