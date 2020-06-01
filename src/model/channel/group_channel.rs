use chrono::{DateTime, FixedOffset};
use crate::model::prelude::*;
use std::borrow::Cow;
use std::fmt::Write;

#[cfg(feature = "http")]
use crate::http::Http;

/// A group channel - potentially including other [`User`]s - separate from a
/// [`Guild`].
///
/// [`Guild`]: ../guild/struct.Guild.html
/// [`User`]: ../user/struct.User.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupChannel {
    /// The Id of the group channel.
    pub id: ChannelId,
    /// The optional icon of the group channel.
    pub icon: Option<String>,
    /// The Id of the last message sent.
    pub last_message_id: Option<MessageId>,
    /// Timestamp of the latest pinned message.
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
    /// The name of the group channel.
    pub name: Option<String>,
    /// The Id of the group owner.
    pub owner_id: UserId,
    /// A list of the group's recipients.
    pub recipients: Vec<User>,
    #[serde(skip)]
    pub(crate) _nonexhaustive: (),
}

#[cfg(feature = "model")]
impl GroupChannel {
    /// Deletes the channel. This does not delete the contents of the channel,
    /// and is equivalent to closing a private channel on the client, which can
    /// be re-opened.
    #[inline]
    pub async fn delete(&self, http: impl AsRef<Http>) -> Result<Channel> {
        self.id.delete(&http).await
    }

    /// Generates a name for the group.
    ///
    /// If there are no recipients in the group, the name will be "Empty Group".
    /// Otherwise, the name is generated in a Comma Separated Value list, such
    /// as "person 1, person 2, person 3".
    pub fn name(&self) -> Cow<'_, str> {
        match self.name {
            Some(ref name) => Cow::Borrowed(name),
            None => {
                let mut name = match self.recipients.first() {
                    Some(recipient) => recipient.name.clone(),
                    None => return Cow::Borrowed("Empty Group"),
                };

                for recipient in self.recipients.iter().skip(1) {
                    let _ = write!(name, ", {}", &recipient.name);
                }

                Cow::Owned(name)
            },
        }
    }
}
