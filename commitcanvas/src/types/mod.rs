mod geometry;

pub use geometry::Point;

pub type Guid = i32;

pub fn to_identifier(guid: Guid) -> String {
    format!("cc_id_{}", guid)
}

pub fn from_identifier(identifier: &str) -> Result<Guid, std::num::ParseIntError> {
    identifier["cc_id_".len()..].parse()
}
