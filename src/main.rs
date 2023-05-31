use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;
use tiff::decoder::{ifd, Decoder, DecodingResult};
use tiff::tags::Tag;
use tiff::TiffResult;

// https://github.com/geospatial-jeff/aiocogeo/tree/master
pub struct BasicTags {
    bitsPerSample: i32, // This is written as an array per sample, but libtiff only takes one value, and so do we.
                        // colorMap: Seq[(Short, Short, Short)] = Seq(),
                        // imageLength: Int = 0,
                        // imageWidth: Int = 0,
                        // compression: Int = 1,
                        // photometricInterp: Int = -1,
                        // resolutionUnit: Option[Int] = None,
                        // rowsPerStrip: Long = -1, // If it's undefined GDAL interprets the entire TIFF as a single strip
                        // samplesPerPixel: Int = 1,
                        // stripByteCounts: Option[Array[Long]] = None,
                        // stripOffsets: Option[Array[Long]] = None,
                        // xResolution: Option[(Long, Long)] = None,
                        // yResolution: Option[(Long, Long)] = None
}

pub struct TiffTags {
    // basicTags: BasicTags = BasicTags(),
    // metadataTags: MetadataTags = MetadataTags(),
    // nonBasicTags: NonBasicTags = NonBasicTags(),
    // geoTiffTags: GeoTiffTags = GeoTiffTags(),
    // documentationTags: DocumentationTags = DocumentationTags(),
    // tileTags: TileTags = TileTags(),
    // cmykTags: CmykTags = CmykTags(),
    // dataSampleFormatTags: DataSampleFormatTags = DataSampleFormatTags(),
    // colimetryTags: ColimetryTags = ColimetryTags(),
    // jpegTags: JpegTags = JpegTags(),
    // yCbCrTags: YCbCrTags = YCbCrTags(),
    // nonStandardizedTags: NonStandardizedTags = NonStandardizedTags(),
    // tiffType: TiffType = Tiff,
    // overviews: List[TiffTags] = Nil
}

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

    dbg!(decoder.get_tag(Tag::GeoKeyDirectoryTag));
    dbg!(decoder.get_tag(Tag::GeoDoubleParamsTag));
    dbg!(decoder.get_tag(Tag::GeoAsciiParamsTag));

    // TODO: Try to read tag for: GeogGeodeticDatumGeoKey = 2050
    // tiff library does not have enum for GeogGeodeticDatumGeoKey so I can't use decoder.get_tag
    // dbg!(decoder.get_tag(Tag::Unknown(2050)).expect("good things"));
    // dbg!(decoder.find_geo_tag(2050).expect("good things"));
}

pub trait GeoTiffExt {
    fn find_geo_tag(&mut self, tag: u32) -> TiffResult<Option<ifd::Value>>;
}

impl<R> GeoTiffExt for Decoder<R>
where
    R: Read + Seek,
{
    fn find_geo_tag(&mut self, tag: u32) -> TiffResult<Option<ifd::Value>> {
        // this won't work because Decoder.image() is a private method

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
