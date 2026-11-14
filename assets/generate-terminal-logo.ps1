# Regenerate the embedded 64x64 RGBA thumbnail from the approved logo (Windows).
# The application embeds these bytes; it needs no image decoder or runtime file.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$source = [System.Drawing.Bitmap]::new((Join-Path $PSScriptRoot 'mother-logo.png'))
$thumbnail = [System.Drawing.Bitmap]::new(64, 64)
$graphics = [System.Drawing.Graphics]::FromImage($thumbnail)
try {
    $graphics.CompositingMode = [System.Drawing.Drawing2D.CompositingMode]::SourceCopy
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $graphics.DrawImage($source, [System.Drawing.Rectangle]::new(0, 0, 64, 64))
    $pixels = [byte[]]::new(64 * 64 * 4)
    for ($y = 0; $y -lt 64; $y++) {
        for ($x = 0; $x -lt 64; $x++) {
            $color = $thumbnail.GetPixel($x, $y)
            $offset = ($y * 64 + $x) * 4
            $pixels[$offset] = $color.R
            $pixels[$offset + 1] = $color.G
            $pixels[$offset + 2] = $color.B
            $pixels[$offset + 3] = $color.A
        }
    }
    [System.IO.File]::WriteAllBytes((Join-Path $PSScriptRoot 'mother-logo.rgba'), $pixels)
} finally {
    $graphics.Dispose()
    $thumbnail.Dispose()
    $source.Dispose()
}
