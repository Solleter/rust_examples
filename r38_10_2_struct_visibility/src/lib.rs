mod my {
    pub struct OpenBox<T> {
        pub content: T,
    }

    #[allow(dead_code)]
    pub struct CloseBox<T> {
        content: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox {
                content: contents,
            }
        }
    }
}

pub fn execute() {
    let open_box = my::OpenBox { content: "public information" };
    println!("The open box contents: {}", open_box.content);

    let _close_box = my::CloseBox::new("classified information");
}
