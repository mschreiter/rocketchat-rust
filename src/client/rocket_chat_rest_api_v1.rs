use std::fmt;

use crate::utils::http_methods::HttpMethods;

#[derive(Debug, Clone, Copy)]
pub enum RocketChatRestApiV1 {
    // Adds all users to the specified channel.
    ChannelsAddAll,
    ChannelsAddModerator,
    ChannelsAddOwner,
    // Archives a channel.
    ChannelsArchive,
    // Cleans up a channel (removing messages).
    ChannelsCleanHistory,
    // Closes a channel.
    ChannelsClose,
    // Creates a new public channel.
    ChannelsCreate,
    // Deletes the public channel from the server.
    ChannelsDelete,
    ChannelsGetIntegrations,
    // Retrieves a public channel's information.
    ChannelsInfo,
    // Retrieves a list of all the public channels.
    ChannelsList,
    // Unarchives a channel.
    ChannelsUnarchive,
    // Invites a user to join a channel.
    ChannelsInvite,
    // Removes a user from the channel.
    ChannelsKick,
    // Removes the callee from the channel.
    ChannelsLeave,
    // Renames the channel.
    ChannelsRename,
    // Adds the channel back to the user's list of channels.
    ChannelsOpen,
    // Deletes a chat message.
    ChatDelete,
    // Sends a new chat message.
    ChatPostMessage,
    // Adds an owner to the group.
    GroupsAddOwner,
    // Retrieves information about a private group, but only if the user is part of it.
    GroupsInfo,
    // Retrieves a list of all the private groups the auth'd user has joined.
    GroupsList,
    // Creates a new private group.
    GroupsCreate,
    // Archives a group.
    GroupsArchive,
    // Unarchives a group.
    GroupsUnarchive,
    // Closes a group.
    GroupsClose,
    // Invites a user to join a group.
    GroupsInvite,
    // Removes a user from the group.
    GroupsKick,
    // Removes the callee from the group.
    GroupsLeave,
    // Adds the group back to the user's list of groups.
    GroupsOpen,
    // Renames the group.
    GroupsRename,
    // Retrieves a list of all the direct message rooms the auth'd user has.
    ImsList,
    // Gets the information about the server, including version and build commit.
    Info,
    // Gets all settings on the server.
    SettingsGetAll,
    // Get a single setting entry by id.
    SettingGetById,
    // Set a single setting by its id.
    SettingSetById,
    // Retrieves the user information from the server.
    UsersInfo,
    // Retrieves a list of all the users in the server.
    UsersList,
    // Creates a new user.
    UsersCreate,
    // Updates a user.
    UsersUpdate,
    // Deletes a user.
    UsersDelete,
    // Creates an auth token for an existing user.
    UsersCreateToken,
}

impl RocketChatRestApiV1 {
    pub fn method_name(&self) -> String {
        format!(
            "v1/{}",
            match self {
                Self::ChannelsAddAll => "channels.addAll",
                Self::ChannelsAddModerator => "channels.addModerator",
                Self::ChannelsAddOwner => "channels.addOwner",
                Self::ChannelsArchive => "channels.archive",
                Self::ChannelsCleanHistory => "channels.cleanHistory",
                Self::ChannelsClose => "channels.close",
                Self::ChannelsCreate => "channels.create",
                Self::ChannelsDelete => "channels.delete",
                Self::ChannelsGetIntegrations => "channels.getIntegrations",
                Self::ChannelsInfo => "channels.info",
                Self::ChannelsList => "channels.list",
                Self::ChannelsUnarchive => "channels.unarchive",
                Self::ChannelsInvite => "channels.invite",
                Self::ChannelsKick => "channels.kick",
                Self::ChannelsLeave => "channels.leave",
                Self::ChannelsRename => "channels.rename",
                Self::ChannelsOpen => "channels.open",
                Self::ChatDelete => "chat.delete",
                Self::ChatPostMessage => "chat.postMessage",
                Self::GroupsAddOwner => "groups.addOwner",
                Self::GroupsInfo => "groups.info",
                Self::GroupsList => "groups.list",
                Self::GroupsCreate => "groups.create",
                Self::GroupsArchive => "groups.archive",
                Self::GroupsUnarchive => "groups.unarchive",
                Self::GroupsClose => "groups.close",
                Self::GroupsInvite => "groups.invite",
                Self::GroupsKick => "groups.kick",
                Self::GroupsLeave => "groups.leave",
                Self::GroupsOpen => "groups.open",
                Self::GroupsRename => "groups.rename",
                Self::ImsList => "ims.list",
                Self::Info => "info",
                Self::SettingsGetAll => "settings",
                Self::SettingGetById => "settings/{0}",
                Self::SettingSetById => "settings/{0}",
                Self::UsersInfo => "users.info",
                Self::UsersList => "users.list",
                Self::UsersCreate => "users.create",
                Self::UsersUpdate => "users.update",
                Self::UsersDelete => "users.delete",
                Self::UsersCreateToken => "users.createToken",
            }
        )
    }

    pub fn http_method(&self) -> HttpMethods {
        match self {
            Self::ChannelsAddAll
            | Self::ChannelsAddModerator
            | Self::ChannelsAddOwner
            | Self::ChannelsArchive
            | Self::ChannelsCleanHistory
            | Self::ChannelsClose
            | Self::ChannelsCreate
            | Self::ChannelsDelete
            | Self::ChannelsUnarchive
            | Self::ChannelsInvite
            | Self::ChannelsKick
            | Self::ChannelsLeave
            | Self::ChannelsRename
            | Self::ChannelsOpen
            | Self::ChatDelete
            | Self::ChatPostMessage
            | Self::GroupsAddOwner
            | Self::GroupsCreate
            | Self::GroupsArchive
            | Self::GroupsUnarchive
            | Self::GroupsClose
            | Self::GroupsInvite
            | Self::GroupsKick
            | Self::GroupsLeave
            | Self::GroupsOpen
            | Self::GroupsRename
            | Self::SettingSetById
            | Self::UsersCreate
            | Self::UsersUpdate
            | Self::UsersDelete
            | Self::UsersCreateToken => HttpMethods::Post,
            Self::ChannelsGetIntegrations
            | Self::ChannelsInfo
            | Self::ChannelsList
            | Self::GroupsInfo
            | Self::GroupsList
            | Self::ImsList
            | Self::Info
            | Self::SettingsGetAll
            | Self::SettingGetById
            | Self::UsersInfo
            | Self::UsersList => HttpMethods::Get,
        }
    }

    pub fn requires_auth(&self) -> bool {
        match self {
            Self::ChannelsAddAll
            | Self::ChannelsAddModerator
            | Self::ChannelsAddOwner
            | Self::ChannelsArchive
            | Self::ChannelsCleanHistory
            | Self::ChannelsClose
            | Self::ChannelsCreate
            | Self::ChannelsDelete
            | Self::ChannelsGetIntegrations
            | Self::ChannelsInfo
            | Self::ChannelsList
            | Self::ChannelsUnarchive
            | Self::ChannelsInvite
            | Self::ChannelsKick
            | Self::ChannelsLeave
            | Self::ChannelsRename
            | Self::ChannelsOpen
            | Self::ChatDelete
            | Self::ChatPostMessage
            | Self::GroupsAddOwner
            | Self::GroupsInfo
            | Self::GroupsList
            | Self::GroupsCreate
            | Self::GroupsArchive
            | Self::GroupsUnarchive
            | Self::GroupsClose
            | Self::GroupsInvite
            | Self::GroupsKick
            | Self::GroupsLeave
            | Self::GroupsOpen
            | Self::GroupsRename
            | Self::ImsList
            | Self::SettingsGetAll
            | Self::SettingGetById
            | Self::SettingSetById
            | Self::UsersInfo
            | Self::UsersList
            | Self::UsersCreate
            | Self::UsersUpdate
            | Self::UsersDelete
            | Self::UsersCreateToken => true,
            Self::Info => false,
        }
    }
}

impl fmt::Display for RocketChatRestApiV1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.method_name())
    }
}
