use crate::utils::RawFileInfo;
use native_dialog::{MessageDialog, MessageType};

#[derive(serde::Deserialize, Debug)]
pub struct ServerResponse(pub Vec<RawFileInfo>);

pub fn popup_message(title: &str, message: &str) -> bool {
    let result = MessageDialog::new()
        .set_title(title)
        .set_text(&message)
        .set_type(MessageType::Warning)
        .show_confirm();
    result.unwrap()
}
