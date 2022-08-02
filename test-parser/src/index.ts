import { filenameParse, ParsedShow } from '@ctrl/video-filename-parser';
import fs from 'fs/promises';
import path from 'path';

interface BaseConfig {
  media: Show[];
}

interface MediaBase {
  path?: string;
  title: string;
}

interface Show extends MediaBase {
  seasons: Season[];
}

interface Season {
  number: number;
  title: string;
  path?: string;
  episodes: Episode[];
}

interface Episode {
  number: number;
  title: string;
  path: string;
}

async function getFilesRecurse(dir: string): Promise<string[]> {
  const paths: string[] = [];
  const rootFiles = await fs.readdir(dir, { withFileTypes: true });
  for (const file of rootFiles) {
    if (file.isDirectory()) {
      paths.push(...(await getFilesRecurse(path.join(dir, file.name))));
    } else {
      paths.push(path.join(dir, file.name));
    }
  }
  return paths;
}

async function main() {
  const paths = await getFilesRecurse('../test-media');

  const config: BaseConfig = {
    media: [],
  };

  for (const path of paths) {
    const fileName = path.split('/').pop();
    if (!fileName) continue;

    const parsed = filenameParse(fileName, true) as ParsedShow;

    const seasonNumber = parsed.seasons?.length ? parsed.seasons[0] : 1;
    const episodeNumber = parsed.episodeNumbers?.length
      ? parsed.episodeNumbers[0]
      : 1;

    const episodeObj: Episode = {
      number: episodeNumber,
      title: `Episode ${episodeNumber}`,
      path: path,
    };

    const seasonObj: Season = {
      number: seasonNumber,
      title: `Season ${seasonNumber}`,
      episodes: [episodeObj],
    };

    const mediaObj: Show = {
      title: parsed.title,
      seasons: [seasonObj],
    };

    const mediaIndex = config.media.findIndex(
      (media) => media.title === parsed.title,
    );

    // Media does not exist
    if (mediaIndex === -1) {
      config.media.push(mediaObj);
      continue;
    }

    const seasonIndex = config.media[mediaIndex].seasons.findIndex(
      (season) => season.number === seasonNumber,
    );

    // Season does not exist
    if (seasonIndex === -1) {
      config.media[mediaIndex].seasons.push(seasonObj);
      continue;
    }

    config.media[mediaIndex].seasons[seasonIndex].episodes.push(episodeObj);
  }

  fs.writeFile('./output.json', JSON.stringify(config, null, 2));
}

main();
