Pseudo code:

let my_tiff = GeoTiff:read("blah.tiff")
data: ndarray = my_tiff.data
metadata: MetaDataStruct = my_tiff.metadata

```
defaults = {
        'driver': 'GTiff',
        'interleave': 'band',
        'tiled': True,
        'blockxsize': 256,
        'blockysize': 256,
        'compress': 'lzw',
        'nodata': 0,
        'dtype': uint8
    }
```