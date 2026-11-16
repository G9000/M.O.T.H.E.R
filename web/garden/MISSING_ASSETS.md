# Missing assets

The released demo has ten sounds. Five are in this repo: the footsteps (CC0). The other five aren't, because they come from the **Sonniss GDC Game Audio Bundles**, whose licence lets anyone use the sounds in a game but not pass them on as sound files. This file lists each one so you can get it and put it back.

## What happens without them

The demo builds and plays. You hear your footsteps and nothing else: no wind in the grass, no leaves overhead, no birds and insects, no water at the tarn, no crickets at dusk. `npm run build` writes `dist/audio/manifest.json` listing only the recordings that are there, so the demo doesn't ask for the others and nothing fails.

## Where to get them

- **Source:** the Sonniss GDC Game Audio Bundles, free from Sonniss, one bundle a year: https://sonniss.com/gameaudiogdc/ (this demo uses the 2017, 2018 and 2019 bundles). Each bundle is a set of libraries, one folder per library, named `<Publisher> - <Library>`. You don't need whole bundles: the three libraries below are a few hundred megabytes together.
- **Licence:** https://sonniss.com/gdc-bundle-license/. Royalty-free, for commercial and non-commercial projects, no credit required (this demo credits the libraries in `public/CREDITS.md` anyway). You may use the sounds inside your game; you may not share, sell or give away the sound files themselves, on their own or in a collection. So keep them out of git, out of Patch's asset library and away from AI services. `.gitignore` already keeps `public/audio/*.ogg` out, except the footsteps.

## How to put them back

1. Download the three libraries listed below and put them in one folder, laid out `<year>/<library folder>/<file>`, for example `sonniss/2019/BlueZone - Lake Wave Sound Effects/Bluezone_BC0252_lake_wave_ambience_04.wav`. The exact folder and file names are in `tools/sounds.mjs`.
2. Install [ffmpeg](https://ffmpeg.org) with libvorbis.
3. Run `SONNISS_DIR=<that folder> node tools/sounds.mjs`. For each recording it takes 36 seconds from the point given below, brings it to one loudness, fades its last 3 seconds across its first 3 so it loops, and writes `public/audio/<id>.ogg` (33 seconds). A recording it can't find is skipped, and it says which.
4. `npm run build`. The manifest then lists them and the demo plays them.

The rebuilt files won't be byte-identical to the released demo's (encoders differ), but they're the same cuts at the same loudness.

## The files

The time is where in the recording the 36 seconds start.

| File | What it is | Where you hear it | Recording (bundle year, library, file) |
| --- | --- | --- | --- |
| `public/audio/summer.ogg` | A summer evening on a hill: birds, insects, quiet air | Everywhere, quieter towards dusk | 2018 bundle, Alexander Kopeikin: Quad ambience of Crimea in summer, `09 vineyard evening ambience, hill, very quiet, birds, crickets, take 2, ortf offset 5 m, front - ORTF.wav` (from 40 s) |
| `public/audio/leaves.ogg` | Wind in tree tops | Under and near a tree, louder in a gust | 2018 bundle, Alexander Kopeikin: Quad ambience of Crimea in summer, `28 lake, high wind in tree tops, birds, daytime - QUAD XY LR.wav` (from 60 s) |
| `public/audio/crickets.ogg` | Crickets in a meadow at night | From about 18:50, rising to sunset | 2018 bundle, Alexander Kopeikin: Quad ambience of Crimea in summer, `20 meadow night ambience, crickets, ortf offset by 3 m, front - ORTF.wav` (from 30 s) |
| `public/audio/grass.ogg` | A breeze in tall grass | Everywhere, rising and falling with the gusts | 2017 bundle, Hzandbits: Wind In Trees, `Wind,Ext,Breeze,Grass,Tall,Rustle,Buffeting.wav` (from 20 s) |
| `public/audio/lake.ogg` | Small waves at the bank of a lake | Within a few metres of the tarn | 2019 bundle, BlueZone: Lake Wave Sound Effects, `Bluezone_BC0252_lake_wave_ambience_04.wav` (from 8 s) |

How loud each one plays, and where, is in `src/valley/sounds.js` (`update`).

## Or use other recordings

Any five loops will do. Name them `summer.ogg`, `leaves.ogg`, `crickets.ogg`, `grass.ogg` and `lake.ogg` in `public/audio/` and build. For an open game, use recordings you may pass on (CC0 or CC BY), and then take their line out of `.gitignore`.
