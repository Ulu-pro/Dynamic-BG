use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reply {
    pub photos: Vec<Photo>
}

#[derive(Deserialize)]
pub struct Photo {
    pub src: PhotoSource
}

#[derive(Deserialize)]
pub struct PhotoSource {
    pub original: String
}