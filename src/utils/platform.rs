use jwalk::DirEntry;
#[allow(unused_imports)]
use std::fs;
use std::io;
use std::path::Path;

#[cfg(target_family = "unix")]
fn get_block_size() -> u64 {
    // All os specific implementations of MetatdataExt seem to define a block as 512 bytes
    // https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html#tymethod.st_blocks
    512
}

#[cfg(target_family = "unix")]
pub fn get_metadata(d: &DirEntry, use_apparent_size: bool) -> Option<(u64, u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    d.metadata.as_ref().unwrap().as_ref().ok().map(|md| {
        if use_apparent_size {
            (md.len(), md.ino(), md.dev())
        } else {
            (md.blocks() * get_block_size(), md.ino(), md.dev())
        }
    })
}

#[cfg(target_family = "windows")]
pub fn get_metadata(d: &DirEntry, _use_apparent_size: bool) -> Option<(u64, u64, u64)> {
    use winapi_util::file::information;
    use winapi_util::Handle;

    let h = Handle::from_path_any(d.path()).ok()?;
    let info = information(&h).ok()?;

    Some((
        info.file_size(),
        info.file_index(),
        info.volume_serial_number(),
    ))
}

#[cfg(target_family = "unix")]
pub fn get_filesystem<P: AsRef<Path>>(file_path: P) -> Result<u64, io::Error> {
    use std::os::unix::fs::MetadataExt;
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.dev())
}
#[cfg(target_family = "windows")]
pub fn get_filesystem<P: AsRef<Path>>(file_path: P) -> Result<u64, io::Error> {
    use winapi_util::file::information;
    use winapi_util::Handle;

    let h = Handle::from_path_any(file_path)?;
    let info = information(&h)?;
    Ok(info.volume_serial_number())
}
