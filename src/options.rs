use clap::Parser;

#[derive(Parser, Debug)]
#[command(long_about = None)]
pub struct Args {
    /// Number of tests
    #[arg(short, long, value_name = "NUMBER")]
    pub tests: Option<u8>,

    /// Enable/Disable download test
    #[arg(short, value_name = "URL", long)]
    pub download: Option<String>,

    /// Size of data to upload
    #[arg(short, long, default_value_t = 10)]
    pub size: usize,

    /// Check the avaibility of the connection
    #[arg(short, long)]
    pub check: bool,
}
