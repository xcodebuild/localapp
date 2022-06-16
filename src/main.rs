mod infer;
mod consts;
mod utils;
mod build;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    url: String,

    #[clap(short, long)]
    title: Option<String>,

    #[clap(short, long)]
    icon: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let title = if args.title.is_none() {
        infer::infer_title(url.clone()).await
    } else {
        args.title.unwrap()
    };

    let icon_path = if args.icon.is_none() {
        infer::infer_icon(url.clone()).await
    } else {
        args.icon.unwrap()
    };
    build::build(title, url, icon_path);
}
