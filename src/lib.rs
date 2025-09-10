use std::fs::File;
use std::io;
use std::path::Path;

use io::BufRead;
use io::Read;

use io::Seek;

use io::BufWriter;
use io::Write;

use zip::ZipArchive;

use zip::read::ZipFile;

pub fn zfile2wtr<R, W>(mut zfile: ZipFile<R>, wtr: &mut W) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    io::copy(&mut zfile, wtr)?;
    Ok(())
}

pub fn zip2wtr<R, W>(zipfile: R, wtr: &mut W) -> Result<(), io::Error>
where
    R: Read + Seek,
    W: Write,
{
    let mut za = ZipArchive::new(zipfile)?;
    let cnt: usize = za.len();
    for i in 0..cnt {
        let zf = za.by_index(i)?;
        zfile2wtr(zf, wtr)?;
    }
    Ok(())
}

pub fn zipfile2wtr<P, W>(zipfilename: P, wtr: &mut W) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    W: Write,
{
    let zf = File::open(zipfilename)?;
    zip2wtr(zf, wtr)
}

pub fn zipfiles2wtr<I, W>(filenames: I, wtr: &mut W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
    W: Write,
{
    for rname in filenames {
        let zname: String = rname?;
        zipfile2wtr(zname, wtr)?;
    }
    Ok(())
}

pub fn stdin2filenames() -> impl Iterator<Item = Result<String, io::Error>> {
    io::stdin().lock().lines()
}

pub fn stdin2filenames2zips2stdout() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    {
        let mut bw = BufWriter::new(&mut out);
        for rname in stdin2filenames() {
            let zname: String = rname?;
            zipfile2wtr(zname, &mut bw)?;
        }
        bw.flush()?;
    }
    out.flush()?;
    Ok(())
}
