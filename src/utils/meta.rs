use serde::Serialize;

#[derive(Serialize)]
pub struct Meta {
    total_items: i32,
    page_number: i32,
    page_size: i32,
    total_pages: i32,
    has_previous_page: bool,
    has_next_page: bool,
    next_page_number: i32,
    previous_page_number: i32,
}

impl Meta {
    #[allow(dead_code)]
    pub fn new(
        total_items: i32,
        page_number: i32,
        page_size: i32,
        total_pages: i32,
        has_previous_page: bool,
        has_next_page: bool,
        next_page_number: i32,
        previous_page_number: i32,
    ) -> Self {
        Self {
            total_items,
            page_number,
            page_size,
            has_next_page,
            has_previous_page,
            total_pages,
            next_page_number,
            previous_page_number,
        }
    }
}
