/* automatically generated by rust-bindgen 0.56.0 */
#![allow(non_camel_case_types)]
use libc::FILE;

#[doc = "< No flag."]
pub const CQDB_NONE: ::std::os::raw::c_uint = 0;
#[doc = "< A reverse lookup array is omitted."]
pub const CQDB_ONEWAY: ::std::os::raw::c_uint = 1;
#[doc = "< An error has occurred."]
pub const CQDB_ERROR_OCCURRED: ::std::os::raw::c_uint = 65536;
#[doc = " CQDB flags."]
#[doc = "< Success."]
pub const CQDB_SUCCESS: ::std::os::raw::c_int = 0;
#[doc = "< Unspecified error."]
pub const CQDB_ERROR: ::std::os::raw::c_int = -1024;
#[doc = "< String not found."]
pub const CQDB_ERROR_NOTFOUND: ::std::os::raw::c_int = -1023;
#[doc = "< Insufficient memory."]
pub const CQDB_ERROR_OUTOFMEMORY: ::std::os::raw::c_int = -1022;
#[doc = "< Error in fwrite() operations."]
pub const CQDB_ERROR_FILEWRITE: ::std::os::raw::c_int = -1021;
#[doc = "< Error in ftell() operations."]
pub const CQDB_ERROR_FILETELL: ::std::os::raw::c_int = -1020;
#[doc = "< Error in fseek() operations."]
pub const CQDB_ERROR_FILESEEK: ::std::os::raw::c_int = -1019;
#[doc = "< Invalid parameters."]
pub const CQDB_ERROR_INVALIDID: ::std::os::raw::c_int = -1018;
#[doc = " CQDB status codes."]
#[doc = " \\addtogroup cqdb_writer CQDB Writer API"]
#[doc = " @{"]
#[doc = ""]
#[doc = "    The CQDB Writer API constructs a CQDB chunk on a seekable stream. The"]
#[doc = "    seekable stream must be created by the fopen() function with writable and"]
#[doc = "    binary flags (\"wb\"). The CQDB Writer API can build a CQDB chunk at any"]
#[doc = "    position on the stream; one can thus write some data, append a CQDB chunk,"]
#[doc = "    and continue writing other data on the stream."]
#[doc = ""]
#[doc = "    By default, the function cqdb_writer() constructs a database with forward"]
#[doc = "    (string to integer identifier) and backward (integer identifier to string)"]
#[doc = "    lookups. The data for reverse lookup is omitted with ::CQDB_ONEWAY flag"]
#[doc = "    specified."]
#[doc = ""]
#[doc = "    It is recommended to keep the maximum number of identifiers as smallest as"]
#[doc = "    possible because reverse lookup is maintained by a array with the size of"]
#[doc = "    sizeof(int) * (maximum number of identifiers + 1). For example, putting a"]
#[doc = "    set of integer identifers (0, 1, 1000) creates a reverse lookup array with"]
#[doc = "    1001 elements only to waste the disk space for 998 (= 1001-3) elements in"]
#[doc = "    the array."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_cqdb_writer {
    _unused: [u8; 0],
}
pub type cqdb_writer_t = tag_cqdb_writer;
extern "C" {
    #[doc = " Create a new CQDB writer on a seekable stream."]
    #[doc = ""]
    #[doc = "    This function initializes a database on the seekable stream and returns"]
    #[doc = "    the pointer to a ::cqdb_writer_t instance to write the database."]
    #[doc = "    The stream must have the writable and binary flags. The database creation"]
    #[doc = "    flag must be zero except when the reverse lookup array is unnecessary;"]
    #[doc = "    specifying ::CQDB_ONEWAY flag will save the storage space for the reverse"]
    #[doc = "    lookup array. Once calling this function, one should avoid accessing the"]
    #[doc = "    seekable stream directly until calling cqdb_writer_close()."]
    #[doc = ""]
    #[doc = "    @param    fp                The pointer to the writable and seekable stream."]
    #[doc = "    @param    flag            Database creation flag."]
    #[doc = "    @retval    cqdb_writer_t*    The pointer to the new ::cqdb_writer_t instance if"]
    #[doc = "                            successful; otherwise \\c NULL."]
    pub fn cqdb_writer(fp: *mut FILE, flag: ::std::os::raw::c_int) -> *mut cqdb_writer_t;
}
extern "C" {
    #[doc = " Put a string/identifier association to the database."]
    #[doc = ""]
    #[doc = "    This function append a string/identifier association into the database."]
    #[doc = "    Make sure that the string and/or identifier have never been inserted to"]
    #[doc = "    the database and that the identifier is a non-negative value."]
    #[doc = ""]
    #[doc = "    @param    dbw            The pointer to the ::cqdb_writer_t instance."]
    #[doc = "    @param    str            The pointer to the string."]
    #[doc = "    @param    id            The identifier."]
    #[doc = "    @retval    int            Zero if successful, or a status code otherwise."]
    pub fn cqdb_writer_put(
        dbw: *mut cqdb_writer_t,
        str_: *const ::std::os::raw::c_char,
        id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Close a CQDB writer."]
    #[doc = ""]
    #[doc = "    This function finalizes the database on the stream. If successful, the"]
    #[doc = "    data remaining on the memory is flushed to the stream; the stream position"]
    #[doc = "    is moved to the end of the chunk. If an unexpected error occurs, this"]
    #[doc = "    function tries to rewind the stream position to the original position when"]
    #[doc = "    the function cqdb_writer() was called."]
    #[doc = ""]
    #[doc = "    @param    dbw            The pointer to the ::cqdb_writer_t instance."]
    #[doc = "    @retval    int            Zero if successful, or a status code otherwise."]
    pub fn cqdb_writer_close(dbw: *mut cqdb_writer_t) -> ::std::os::raw::c_int;
}
#[doc = " \\addtogroup cqdb_reader CQDB Reader API"]
#[doc = " @{"]
#[doc = ""]
#[doc = "    The CQDB reader API provides a read access to the database whose memory"]
#[doc = "    image is loaded on a memory block. The memory-passing interface has"]
#[doc = "    several advantages. Firstly, one can choose an efficient way for their"]
#[doc = "    application to load a database image to a memory block, e.g., to read"]
#[doc = "    the whole image from a file, to use the Memory Mapped File (mmap) API,"]
#[doc = "    etc."]
#[doc = "    Secondaly, one can design the file format freely only if the memory"]
#[doc = "    block for a database is extracted from the file."]
#[doc = ""]
#[doc = "    The most fundamental operation on the CQDB reader API is forward lookup"]
#[doc = "    through the use of cqdb_to_id() function, which retrieves integer"]
#[doc = "    identifiers from strings. Reverse lookup (retrieving strings from integer"]
#[doc = "    identifiers) with cqdb_to_string() function is not supported if the"]
#[doc = "    database has been created with ::CQDB_ONEWAY flag."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_cqdb {
    _unused: [u8; 0],
}
pub type cqdb_t = tag_cqdb;
extern "C" {
    #[doc = " Open a new CQDB reader on a memory block."]
    #[doc = ""]
    #[doc = "    This function initializes a database on a memory block and returns the"]
    #[doc = "    pointer to a ::cqdb_t instance to access the database."]
    #[doc = ""]
    #[doc = "    @param    buffer        The pointer to the memory block."]
    #[doc = "    @param    size        The size of the memory block."]
    #[doc = "    @retval    cqdb_t*        The pointer to the ::cqdb_t instance."]
    pub fn cqdb_reader(buffer: *const ::std::os::raw::c_void, size: usize) -> *mut cqdb_t;
}
extern "C" {
    #[doc = " Delete the CQDB reader."]
    #[doc = ""]
    #[doc = "    This function frees the work area allocated by cqdb_reader() function."]
    #[doc = ""]
    #[doc = "    @param    db            The pointer to the ::cqdb_t instance."]
    pub fn cqdb_delete(db: *mut cqdb_t);
}
extern "C" {
    #[doc = " Retrieve the identifier associated with a string."]
    #[doc = ""]
    #[doc = "    This function returns the identifier associated with a string."]
    #[doc = ""]
    #[doc = "    @param    db            The pointer to the ::cqdb_t instance."]
    #[doc = "    @param    str            The pointer to a string."]
    #[doc = "    @retval    int            The non-negative identifier if successful, negative"]
    #[doc = "                        status code otherwise."]
    pub fn cqdb_to_id(
        db: *mut cqdb_t,
        str_: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Retrieve the string associated with an identifier."]
    #[doc = ""]
    #[doc = "    This function returns the string associated with an identifier."]
    #[doc = ""]
    #[doc = "    @param    db            The pointer to the cqdb_t instance."]
    #[doc = "    @param    id            The id."]
    #[doc = "    @retval    const char*    The pointer to the string associated with the"]
    #[doc = "                        identifier if successful; otherwise \\c NULL."]
    pub fn cqdb_to_string(
        db: *mut cqdb_t,
        id: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Get the number of associations in the database."]
    #[doc = ""]
    #[doc = "    This function returns the number of associations in the database."]
    #[doc = ""]
    #[doc = "    @param    db            The pointer to the ::cqdb_t instance."]
    #[doc = "    @retval    int            The number of string/identifier associations."]
    pub fn cqdb_num(db: *mut cqdb_t) -> ::std::os::raw::c_int;
}
