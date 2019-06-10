use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct PageInfoType {
    #[allow(dead_code)]
    totalResults: u8,

    #[allow(dead_code)]
    resultsPerPage: u8
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct StatisticsType {
    #[allow(dead_code)]
    viewCount: String,

    #[allow(dead_code)]
    commentCount: String,

    pub subscriberCount: String,

    #[allow(dead_code)]
    hiddenSubscriberCount: bool,

    #[allow(dead_code)]
    videoCount: String
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct  ItemType {
    #[allow(dead_code)]
    kind: String,

    #[allow(dead_code)]
    etag: String,

    pub id: String,
    pub statistics: StatisticsType
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct YoutubeResponseType {
    #[allow(dead_code)]
    kind: String,

    #[allow(dead_code)]
    etag: String,

    #[allow(dead_code)]
    nextPageToken: String,

    #[allow(dead_code)]
    pageInfo: PageInfoType,

    pub items: Vec<ItemType>
}