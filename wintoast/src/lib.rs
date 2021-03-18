mod bindings {
    ::windows::include_bindings!();
}

use std::error::Error;

use bindings::{
    windows::data::xml::dom::XmlDocument, windows::ui::notifications::ToastNotification,
    windows::ui::notifications::ToastNotificationManager, windows::HString,
};

pub enum Duration {
    Long,
    Short,
}

pub enum Sound {
    Default,
    None,
}

pub struct Toast<'a> {
    title: &'a str,
    body: &'a str,
    duration: Duration,
    sound: Sound,
}

impl<'a> Toast<'a> {
    pub fn new(title: &'a str, body: &'a str, duration: Duration, sound: Sound) -> Toast<'a> {
        Toast {
            title,
            body,
            duration,
            sound,
        }
    }
}

pub fn show<'a>(toast: &Toast) -> Result<(), Box<dyn Error>> {
    let doc = XmlDocument::new()?;
    let duratation = match toast.duration {
        Duration::Short => "short",
        Duration::Long => "long",
    };
    let sound = match toast.sound {
        Sound::None => "<audio silent=\"true\" />",
        Sound::Default => "",
    };
    doc.load_xml(HString::from(format!(
        "<toast duration=\"{}\">
                <visual>
                    <binding template=\"ToastGeneric\">
                        <text>{}</text>
                        <text>{}</text>
                    </binding>
                </visual>
                {}
            </toast>",
        duratation, toast.title, toast.body, sound
    )))?;
    let toast = ToastNotification::create_toast_notification(doc)?;
    let toast_notifier = ToastNotificationManager::create_toast_notifier_with_id(HString::from(
        "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe",
    ))?;
    toast_notifier.show(&toast)?;
    Ok(())
}
