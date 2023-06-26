#!/bin/bash
if command -v magick >/dev/null 2>&1 ; then
	magick display image.ppm
else
	echo "magick/imagemagick needs to be installed"
fi
