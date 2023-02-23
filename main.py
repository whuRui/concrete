# coding=utf-8
import time

import concrete
import numpy
from tifffile import imread, imwrite, TiffFile

if __name__ == "__main__":
    start = time.time()
    input_file = ''
    ceramsite_ratio = 0.75
    color_map = None
    with TiffFile(input_file + '.tif') as tif:
        for tag in tif.pages[0].tags:
            if tag.name == 'ColorMap':
                color_map = tag.value
    img = imread(input_file + '.tif')

    num, total, converted, new_img = concrete.make_some_ceramsite(img, 1 / ceramsite_ratio)
    print("骨料个数:", num, "骨料总体积:", total,
          "转换陶粒体积:", converted,
          "转换率:", "{}%".format(round((converted / total * 100), 3)))

    new_img = numpy.array(new_img, dtype='uint8')
    imwrite(input_file + '.outs.tif', new_img, photometric=3, colormap=color_map)
    print('calcu duration:', time.time() - start)
