use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u32,
}

pub fn default_page() -> u32 {
    1
}

pub fn default_page_size() -> u32 {
    20
}

pub fn clamp_page_size(page_size: u32) -> u32 {
    page_size.clamp(1, 50)
}

pub fn paginate_offset(page: u32, page_size: u32) -> (u32, u32, i64) {
    let page = page.max(1);
    let page_size = clamp_page_size(page_size);
    let offset = ((page - 1) as i64) * (page_size as i64);
    (page, page_size, offset)
}
