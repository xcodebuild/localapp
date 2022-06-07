mod infer;
mod consts;
mod utils;
mod build;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let title = infer::infer_title(url.clone()).await;
    let icon_path = infer::infer_icon(url.clone()).await;
    build::build(title, url, icon_path);
}
