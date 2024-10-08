#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ERoutes {
    // Auth
    LOGIN,
    LOGIN_WITH_TOKEN,
    // Access Key
    CREATE_ACCESS_KEY,
    GET_ACCESS_KEY_LIST,
    DELETE_ACCESS_KEY,
    // Create Account
    CREATE_ACCOUNT,
    // Collection
    CREATE_COLLECTION,
    GET_COLLECTION_LIST,
    GET_COLLECTION_BY_ID,
    UPDATE_COLLECTION_BY_ID,
    DELETE_COLLECTION_BY_ID,
    // Schema
    CREATE_SCHEMA,
    GET_SCHEMA_LIST,
    UPDATE_SCHEMA,
    GET_SCHEMA_BY_ID,
    DELETE_SCHEMA_BY_ID,
    GET_SCHEMA_BY_HANDLE,
    // Storage
    UPLOAD_IMAGE,
    GET_IMAGE_LIST,
    DELETE_IMAGE,
    // Object
    CREATE_OBJECT,
    PARSE_OBJECTS,
    GET_OBJECT_LIST,
    UPDATE_OBJECT_BY_ID,
    GET_OBJECT_BY_ID,
    DELETE_OBJECT_BY_ID,
    UPDATE_OR_CREATE_OBJECT_BY_HANDLE,
    // User
    CREATE_USER,
    GET_USER_LIST,
    GET_USER_BY_ID,
    UPDATE_USER,
    DELETE_USER,
    // User Meta
    UPDATE_USER_META,
    DELETE_USER_META_ITEM,
    // User Auth
    USER_LOGIN,
    USER_LOGIN_WITH_TOKEN,
    USER_RESET_PASSWORD_BY_CURRENT_PASSWORD,
    USER_RESET_EMAIL,
    USER_SEND_RECOVER_EMAIL,
    USER_RESET_PASSWORD,
    // Member
    CREATE_MEMBER,
    GET_MEMBER_LIST,
    GET_MEMBER_BY_ID,
    UPDATE_MEMBER_BY_ID,
    DELETE_MEMBER,
    // Shortcut
    CREATE_SHORTCUT,
    GET_SHORTCUT_LIST,
    DELETE_SHORTCUT,
    // Email
    SEND_EMAIL,
}
