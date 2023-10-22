use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PagedResponse<T> {
    data: Vec<T>,
    total_pages: usize,
    current_page: usize,
}

impl<T> PagedResponse<T> {
    // Define any methods you need here
}
