use std::env;

pub fn is_gte_max_queue_size(num: u32) -> bool {
    num.ge(&get_max_queue_size_var())
}

fn get_max_queue_size_var() -> u32 {
    // If there are memory constraints for the bot (e.g. cloud hosting only
    // allows certain amount of memory), we can restrict the size of the
    // queue (the main culprit for high memory use) with an environment
    // variable.
    //
    // Look if there's one set, if not use the default.
    const DEFAULT_QUEUE_SIZE: u32 = 15;
    let max_queue_size: u32 = match env::var("MAX_QUEUE_SIZE") {
        Ok(size) => size.parse::<u32>().unwrap(),
        Err(_) => DEFAULT_QUEUE_SIZE,
    };

    max_queue_size
}
