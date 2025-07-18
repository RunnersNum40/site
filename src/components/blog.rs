use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use include_dir::{include_dir, Dir, File};

const PROJECT_DIR: Dir = include_dir!("content");

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Title { "Ted Pinkerton: Blog" },
        div {
            id: "blog",
            "Blog"
            for entry in PROJECT_DIR.find("**/*.md").expect("Error finding markdown files") {
                BlogPost {
                    file: entry.as_file().expect("Error converting entry to file").clone(),
                }
            }
        }
    }
}

#[component]
fn BlogPost(file: File<'static>) -> Element {
    rsx! {
        div {
            class: "blog-post",
            Markdown {
                src: file.contents_utf8().expect("Error reading file"),
            }
        }
    }
}
