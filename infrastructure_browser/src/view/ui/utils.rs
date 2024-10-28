use commitcanvas::model::Guid;

pub fn to_identifier(guid: Guid) -> String {
    format!("cc_id_{}", guid)
}

#[allow(dead_code)]
pub fn from_identifier(identifier: &str) -> Result<Guid, std::num::ParseIntError> {
    identifier["cc_id_".len()..].parse()
}
