use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;
use tiff::decoder::{ifd, Decoder, DecodingResult};
use tiff::tags::Tag;
use tiff::TiffResult;

fn main() {
    const TEST_IMAGE: &str = "data/sp27.tif";
    let path = PathBuf::from(TEST_IMAGE);
    let img_file = File::open(path).expect("Cannot find test image!");
    let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
    let img_res: DecodingResult = decoder.read_image().unwrap();

    match img_res {
        DecodingResult::U8(res) => {
            dbg!(res.len());
            println!("Tiff is bytes, first 4: {:?} ...", &res[0..4]);
        }
        _ => panic!("Tiff is NOT byte!"),
    }

    dbg!(decoder
        .get_tag(Tag::GeoKeyDirectoryTag)
        .expect("no GeoKeyDirectoryTag"));

    // TODO: Try to read tag for: GeogGeodeticDatumGeoKey = 2050
    // tiff library does not have enum for GeogGeodeticDatumGeoKey so I can't use decoder.get_tag
    dbg!(decoder.get_tag(Tag::Unknown(2050)).expect("good things"));
    dbg!(decoder.find_geo_tag(2050).expect("good things"));
}

pub trait GeoTiffExt {
    fn find_geo_tag(&mut self, tag: u32) -> TiffResult<Option<ifd::Value>>;
}

impl<R> GeoTiffExt for Decoder<R>
where
    R: Read + Seek,
{
    fn find_geo_tag(&mut self, tag: u32) -> TiffResult<Option<ifd::Value>> {
        //// this won't work because Decoder.image() is a private method

        // let entry = match self.image().ifd.as_ref().unwrap().get(&tag) {
        //     None => return Ok(None),
        //     Some(entry) => entry.clone(),
        // };

        // Ok(Some(entry.val(
        //     &self.limits,
        //     self.bigtiff,
        //     &mut self.reader,
        // )?))

        Ok(None)
    }
}
