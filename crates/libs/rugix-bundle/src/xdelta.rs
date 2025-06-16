use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use reportify::{bail, ResultExt};
use tracing::{trace, Level};

use crate::BundleResult;

#[tracing::instrument(level = Level::DEBUG, skip(patch, output))]
pub fn xdelta_decompress<R, W>(source: &Path, patch: &mut R, output: &mut W) -> BundleResult<()>
where
    R: Read + Send,
    W: Write + Send,
{
    let mut child = Command::new("xdelta3")
        .arg("-d")
        .arg("-c")
        .arg("-s")
        .arg(source)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .whatever("unable to spawn xdelta")?;
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let exit_status = std::thread::scope(|scope| {
        scope.spawn(move || {
            trace!("feeding patch to xdelta");
            let result = std::io::copy(patch, &mut stdin);
            trace!(?result, "done feeding patch to xdelta");
        });
        scope.spawn(move || std::io::copy(&mut stdout, output));
        child.wait()
    })
    .whatever("error running xdelta")?;
    if !exit_status.success() {
        bail!(
            "xdelta exited with non-zero return code: {:?}",
            exit_status.code()
        );
    }
    Ok(())
}

#[tracing::instrument(level = Level::DEBUG)]
pub fn xdelta_compress(old: &Path, new: &Path, patch: &Path) -> BundleResult<()> {
    let mut child = Command::new("xdelta3")
        .arg("-e")
        .arg("-s")
        .arg(old)
        .arg(new)
        .arg(patch)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .whatever("unable to spawn xdelta")?;
    let exit_status = child.wait().whatever("error running xdelta")?;
    if !exit_status.success() {
        bail!(
            "xdelta exited with non-zero return code: {:?}",
            exit_status.code()
        );
    }
    Ok(())
}
