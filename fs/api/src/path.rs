use crate::api::API;
use crate::directory::Directory;
use crate::file::File;

#[derive(Debug)]
pub enum FSPath {
    API(API),
    File(File),
    Directory(Directory),
}
