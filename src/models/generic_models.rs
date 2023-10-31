use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub total_pages: i64,
    pub current_page: i64,
}

impl<T> PagedResponse<T> {
    // Define any methods you need here
}
