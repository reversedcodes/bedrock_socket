pub const ID_CONNECTED_PING: u8 = 0x00;
pub const ID_UNCONNECTED_PING: u8 = 0x01;
pub const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: u8 = 0x02;
pub const ID_CONNECTED_PONG: u8 = 0x03;
pub const ID_DETECT_LOST_CONNECTIONS: u8 = 0x04;

pub const ID_OPEN_CONNECTION_REQUEST_1: u8 = 0x05;
pub const ID_OPEN_CONNECTION_REPLY_1: u8 = 0x06;
pub const ID_OPEN_CONNECTION_REQUEST_2: u8 = 0x07;
pub const ID_OPEN_CONNECTION_REPLY_2: u8 = 0x08;

pub const ID_CONNECTION_REQUEST: u8 = 0x09;
pub const ID_REMOTE_SYSTEM_REQUIRES_pub_KEY: u8 = 0x0a;
pub const ID_OUR_SYSTEM_REQUIRES_SECURITY: u8 = 0x0b;
pub const ID_pub_KEY_MISMATCH: u8 = 0x0c;
pub const ID_OUT_OF_BAND_INTERNAL: u8 = 0x0d;
pub const ID_SND_RECEIPT_ACKED: u8 = 0x0e;
pub const ID_SND_RECEIPT_LOSS: u8 = 0x0f;
pub const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 0x10;
pub const ID_CONNECTION_ATTEMPT_FAILED: u8 = 0x11;
pub const ID_ALREADY_CONNECTED: u8 = 0x12;
pub const ID_NEW_INCOMING_CONNECTION: u8 = 0x13;
pub const ID_NO_FREE_INCOMING_CONNECTIONS: u8 = 0x14;
pub const ID_DISCONNECTION_NOTIFICATION: u8 = 0x15;
pub const ID_CONNECTION_LOST: u8 = 0x16;
pub const ID_CONNECTION_BANNED: u8 = 0x17;
pub const ID_INVALID_PASSWORD: u8 = 0x18;
pub const ID_INCOMPATIBLE_PROTOCOL_VERSION: u8 = 0x19;
pub const ID_IP_RECENTLY_CONNECTED: u8 = 0x1a;
pub const ID_TIMESTAMP: u8 = 0x1b;
pub const ID_UNCONNECTED_PONG: u8 = 0x1c;
pub const ID_ADVERTISE_SYSTEM: u8 = 0x1d;
pub const ID_DOWNLOAD_PROGRESS: u8 = 0x1e;

pub const ID_REMOTE_DISCONNECTION_NOTIFICATION: u8 = 0x1f;
pub const ID_REMOTE_CONNECTION_LOST: u8 = 0x20;
pub const ID_REMOTE_NEW_INCOMING_CONNECTION: u8 = 0x21;

pub const ID_FILE_LIST_TRANSFER_HEADER: u8 = 0x22;
pub const ID_FILE_LIST_TRANSFER_FILE: u8 = 0x23;
pub const ID_FILE_LIST_REFERENCE_PUSH_ACK: u8 = 0x24;

pub const ID_DDT_DOWNLOAD_REQUEST: u8 = 0x25;

pub const ID_TRANSPORT_STRING: u8 = 0x26;

pub const ID_REPLICA_MANAGER_CONSTRUCTION: u8 = 0x27;
pub const ID_REPLICA_MANAGER_SCOPE_CHANGE: u8 = 0x28;
pub const ID_REPLICA_MANAGER_SERIALIZE: u8 = 0x29;
pub const ID_REPLICA_MANAGER_DOWNLOAD_STARTED: u8 = 0x2a;
pub const ID_REPLICA_MANAGER_DOWNLOAD_COMPLETE: u8 = 0x2b;

pub const ID_RAKVOICE_OPEN_CHANNEL_REQUEST: u8 = 0x2c;
pub const ID_RAKVOICE_OPEN_CHANNEL_REPLY: u8 = 0x2d;
pub const ID_RAKVOICE_CLOSE_CHANNEL: u8 = 0x2e;
pub const ID_RAKVOICE_DATA: u8 = 0x2f;

pub const ID_AUTOPATCHER_GET_CHANGELIST_SINCE_DATE: u8 = 0x30;
pub const ID_AUTOPATCHER_CREATION_LIST: u8 = 0x31;
pub const ID_AUTOPATCHER_DELETION_LIST: u8 = 0x32;
pub const ID_AUTOPATCHER_GET_PATCH: u8 = 0x33;
pub const ID_AUTOPATCHER_PATCH_LIST: u8 = 0x34;
pub const ID_AUTOPATCHER_REPOSITORY_FATAL_ERROR: u8 = 0x35;
pub const ID_AUTOPATCHER_CANNOT_DOWNLOAD_ORIGINAL_UNMODIFIED_FILES: u8 = 0x36;
pub const ID_AUTOPATCHER_FINISHED_INTERNAL: u8 = 0x37;
pub const ID_AUTOPATCHER_FINISHED: u8 = 0x38;
pub const ID_AUTOPATCHER_RESTART_APPLICATION: u8 = 0x39;

pub const ID_NAT_PUNCHTHROUGH_REQUEST: u8 = 0x3a;
pub const ID_NAT_CONNECT_AT_TIME: u8 = 0x3b;
pub const ID_NAT_GET_MOST_RECENT_PORT: u8 = 0x3c;
pub const ID_NAT_CLIENT_READY: u8 = 0x3d;
pub const ID_NAT_TARGET_NOT_CONNECTED: u8 = 0x3e;
pub const ID_NAT_TARGET_UNRESPONSIVE: u8 = 0x3f;
pub const ID_NAT_CONNECTION_TO_TARGET_LOST: u8 = 0x40;
pub const ID_NAT_ALREADY_IN_PROGRESS: u8 = 0x41;
pub const ID_NAT_PUNCHTHROUGH_FAILED: u8 = 0x42;
pub const ID_NAT_PUNCHTHROUGH_SUCCEEDED: u8 = 0x43;

pub const ID_READY_EVENT_SET: u8 = 0x44;
pub const ID_READY_EVENT_UNSET: u8 = 0x45;
pub const ID_READY_EVENT_ALL_SET: u8 = 0x46;
pub const ID_READY_EVENT_QUERY: u8 = 0x47;

pub const ID_LOBBY_GENERAL: u8 = 0x48;

pub const ID_RPC_REMOTE_ERROR: u8 = 0x49;
pub const ID_RPC_PLUGIN: u8 = 0x4a;

pub const ID_FILE_LIST_REFERENCE_PUSH: u8 = 0x4b;

pub const ID_READY_EVENT_FORCE_ALL_SET: u8 = 0x4c;

pub const ID_ROOMS_EXECUTE_FUNC: u8 = 0x4d;
pub const ID_ROOMS_LOGON_STATUS: u8 = 0x4e;
pub const ID_ROOMS_HANDLE_CHANGE: u8 = 0x4f;

pub const ID_LOBBY2_SEND_MESSAGE: u8 = 0x50;
pub const ID_LOBBY2_SERVER_ERROR: u8 = 0x51;

pub const ID_FCM2_NEW_HOST: u8 = 0x52;
pub const ID_FCM2_REQUEST_FCMGUID: u8 = 0x53;
pub const ID_FCM2_RESPOND_CONNECTION_COUNT: u8 = 0x54;
pub const ID_FCM2_INFORM_FCMGUID: u8 = 0x55;
pub const ID_FCM2_UPDATE_MIN_TOTAL_CONNECTION_COUNT: u8 = 0x56;
pub const ID_FCM2_VERIFIED_JOIN_START: u8 = 0x57;
pub const ID_FCM2_VERIFIED_JOIN_CAPABLE: u8 = 0x58;
pub const ID_FCM2_VERIFIED_JOIN_FAILED: u8 = 0x59;
pub const ID_FCM2_VERIFIED_JOIN_ACCEPTED: u8 = 0x5a;
pub const ID_FCM2_VERIFIED_JOIN_REJECTED: u8 = 0x5b;

pub const ID_UDP_PROXY_GENERAL: u8 = 0x5c;

pub const ID_SQLite3_EXEC: u8 = 0x5d;
pub const ID_SQLite3_UNKNOWN_DB: u8 = 0x5e;
pub const ID_SQLLITE_LOGGER: u8 = 0x5f;

pub const ID_NAT_TYPE_DETECTION_REQUEST: u8 = 0x60;
pub const ID_NAT_TYPE_DETECTION_RESULT: u8 = 0x61;

pub const ID_ROUTER_2_INTERNAL: u8 = 0x62;
pub const ID_ROUTER_2_FORWARDING_NO_PATH: u8 = 0x63;
pub const ID_ROUTER_2_FORWARDING_ESTABLISHED: u8 = 0x64;
pub const ID_ROUTER_2_REROUTED: u8 = 0x65;

pub const ID_TEAM_BALANCER_INTERNAL: u8 = 0x66;
pub const ID_TEAM_BALANCER_REQUESTED_TEAM_FULL: u8 = 0x67;
pub const ID_TEAM_BALANCER_REQUESTED_TEAM_LOCKED: u8 = 0x68;
pub const ID_TEAM_BALANCER_TEAM_REQUESTED_CANCELLED: u8 = 0x69;
pub const ID_TEAM_BALANCER_TEAM_ASSIGNED: u8 = 0x6a;

pub const ID_LIGHTSPEED_INTEGRATION: u8 = 0x6b;

pub const ID_XBOX_LOBBY: u8 = 0x6c;

pub const ID_TWO_WAY_AUTHENTICATION_INCOMING_CHALLENGE_SUCCESS: u8 = 0x6d;
pub const ID_TWO_WAY_AUTHENTICATION_OUTGOING_CHALLENGE_SUCCESS: u8 = 0x6e;
pub const ID_TWO_WAY_AUTHENTICATION_INCOMING_CHALLENGE_FAILURE: u8 = 0x6f;
pub const ID_TWO_WAY_AUTHENTICATION_OUTGOING_CHALLENGE_FAILURE: u8 = 0x70;
pub const ID_TWO_WAY_AUTHENTICATION_OUTGOING_CHALLENGE_TIMEOUT: u8 = 0x71;
pub const ID_TWO_WAY_AUTHENTICATION_NEGOTIATION: u8 = 0x72;

pub const ID_CLOUD_POST_REQUEST: u8 = 0x73;
pub const ID_CLOUD_RELEASE_REQUEST: u8 = 0x74;
pub const ID_CLOUD_GET_REQUEST: u8 = 0x75;
pub const ID_CLOUD_GET_RESPONSE: u8 = 0x76;
pub const ID_CLOUD_UNSUBSCRIBE_REQUEST: u8 = 0x77;
pub const ID_CLOUD_SERVER_TO_SERVER_COMMAND: u8 = 0x78;
pub const ID_CLOUD_SUBSCRIPTION_NOTIFICATION: u8 = 0x79;

pub const ID_LIB_VOICE: u8 = 0x7a;

pub const ID_RELAY_PLUGIN: u8 = 0x7b;
pub const ID_NAT_REQUEST_BOUND_ADDRESSES: u8 = 0x7c;
pub const ID_NAT_RESPOND_BOUND_ADDRESSES: u8 = 0x7d;
pub const ID_FCM2_UPDATE_USER_CONTEXT: u8 = 0x7e;
pub const ID_RESERVED_3: u8 = 0x7f;
pub const ID_RESERVED_4: u8 = 0x80;
pub const ID_RESERVED_5: u8 = 0x81;
pub const ID_RESERVED_6: u8 = 0x82;
pub const ID_RESERVED_7: u8 = 0x83;
pub const ID_RESERVED_8: u8 = 0x84;
pub const ID_RESERVED_9: u8 = 0x85;

pub const ID_USER_PACKET_ENUM: u8 = 0x86;