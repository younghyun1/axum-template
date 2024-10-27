use utils::stopwatch::stopwatch::Stopwatch;

pub mod utils {
    pub mod stopwatch {
        pub mod stopwatch;
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let mut stopwatch = Stopwatch::new(Some("Initializing webserver..."));

    
    Ok(())
}
