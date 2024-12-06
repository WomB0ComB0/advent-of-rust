import * as cheerio from 'cheerio';
import { mkdir, writeFile, rm, readdir, stat } from 'fs/promises';
import path from 'path';
import puppeteer from 'puppeteer';
import { $ } from 'bun'

interface Challenge {
  description: string;
  code: string;
}

class ScraperError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'ScraperError';
  }
}

async function updateWorkspaceMembers(year: number) {
  const baseDir = path.join(process.cwd(), year.toString());
  const rootCargoTomlPath = path.join(process.cwd(), 'Cargo.toml');

  try {
    const directories = await readdir(baseDir);
    const validMembers: string[] = [];

    for (const dir of directories) {
      const dirPath = path.join(baseDir, dir);
      const cargoTomlPath = path.join(dirPath, 'Cargo.toml');

      try {
        const stats = await stat(cargoTomlPath);
        if (stats.isFile()) {
          validMembers.push(`"${year}/${dir}"`);
        }
      } catch {}
      finally {
        console.log(`Checked ${dirPath}`);
      }
    }

    const cargoTomlContent = [
      '[workspace]',
      'members = [',
      validMembers.join(',\n  '),
      ']',
    ].join('\n');

    await writeFile(rootCargoTomlPath, cargoTomlContent, { encoding: 'utf8' });
    console.log('Updated Cargo.toml with current project members.');
  } catch (error) {
    console.error('Failed to update workspace members:', error);
  }
}

async function extractChallenges(html: string): Promise<Challenge[]> {
  const $ = cheerio.load(html);
  const challenges: Challenge[] = [];

  console.log('HTML content length:', html.length);

  const descriptionParts: string[] = [];
  $('.text-lg.text-gray-400').find('p, h1, h2, ul').each((_, el) => {
    const $el = $(el);
    if ($el.is('p, h1, h2')) {
      const text = $el.text().trim();
      if (text) descriptionParts.push(text);
    } else if ($el.is('ul')) {
      const listItems = $el.find('li').map((_, li) => `- ${$(li).text().trim()}`).get();
      if (listItems.length) descriptionParts.push(listItems.join('\n'));
    }
  });

  const description = descriptionParts.join('\n\n');

  const code = $('.view-line').map((_, line) => {
    const spans = $(line).find('span span');
    const lineText = spans.map((_, span) => {
      const text = $(span).text();
      return text;
    }).get();
    const joinedLine = lineText.join('').trim();
    return joinedLine;
  }).get();
  const filteredCode = code.filter(line => line);
  const finalCode = filteredCode.join('\n');

  if (description && finalCode) {
    challenges.push({ description, code: finalCode });
  }

  return challenges;
}

async function fetchChallengeWithPuppeteer(year: number, day: number): Promise<Challenge | null> {
  const browser = await puppeteer.launch({
    headless: false,
    defaultViewport: null
  });
  const page = await browser.newPage();

  try {
    const url = `https://www.rustfinity.com/practice/rust/challenges/aor-${year}-${day}/description`;
    console.log(`Navigating to: ${url}`);

    await page.goto(url, {
      waitUntil: ['networkidle0', 'domcontentloaded'],
      timeout: 30000
    });

    await page.waitForSelector('#radix-\\:r5\\:-content-description', {
      timeout: 15000,
      visible: true
    });

    await new Promise(resolve => setTimeout(resolve, 2000));

    const html = await page.content();
    const challenges = await extractChallenges(html);

    await page.close();

    if (challenges.length === 0) {
      console.error('No challenges found');
      return null;
    }

    return challenges[0];
  } catch (error) {
    console.error('Error during page fetch:', error);
    throw error;
  } finally {
    await browser.close();
  }
}

async function saveChallenge(year: number, day: number, content: Challenge | null) {
  if (!content) {
    console.error('No content to save');
    return;
  }

  try {
    const baseDir = path.join(process.cwd(), year.toString(), day.toString());
    const srcDir = path.join(baseDir, 'src');

    try {
      await rm(baseDir, { recursive: true, force: true });
    } catch { } finally {
      console.log(`Deleted directory ${baseDir}`);
    }

    await mkdir(baseDir, { recursive: true, mode: 0o755 });
    await mkdir(srcDir, { recursive: true, mode: 0o755 });

    const questionContent = [
      `# Advent of Rust ${year} - Day ${day}`,
      '',
      content.description,
      '',
      '## Initial Code',
      '```rust',
      content.code,
      '```',
      '',
    ].join('\n');

    // Remove zero-width spaces and other unwanted characters
    const cleanedCode = content.code
      .replace(/\u200B/g, "")
      .replace(/\u200C/g, "")
      .replace(/\u200D/g, "")
      .replace(/\uFEFF/g, "")
      .replace(/\u00A0/g, " ");


    await writeFile(path.join(srcDir, 'main.rs'), cleanedCode, { encoding: 'utf8', mode: 0o644 });

    await Promise.all([
      writeFile(path.join(baseDir, 'README.md'), questionContent, { encoding: 'utf8', mode: 0o644 }),
      writeFile(path.join(baseDir, 'Cargo.toml'),
        `[package]\nname = "aor-${year}-day-${day}"\nversion = "0.1.0"\nedition = "2021"\n`,
        { encoding: 'utf8', mode: 0o644 }
      )
    ]);

    try {
      await $`cargo fmt --manifest-path ${path.join(baseDir, 'Cargo.toml')}`
    } catch (error) {
      console.log(`You likely need to install the formatter 'rustup component add rustfmt'`);
      console.error(`Error formatting code in ${baseDir}: ${error}`);
    } finally {
      console.log(`Formatted code in ${baseDir}`);
    }

    try {
      await $`cargo clean --manifest-path ${path.join(baseDir, 'Cargo.toml')}`;
      await $`cargo build --manifest-path ${path.join(baseDir, 'Cargo.toml')}`;
    } catch (error) {
      console.error(`Error building project in ${baseDir}: ${error}`);
    } finally {
      console.log(`Built project in ${baseDir}`);
    }

    try {
      await updateWorkspaceMembers(year);
    } catch (error) {
      console.error('Failed to update workspace members:', error);
    } finally {
      console.log(`Challenge for day ${day} saved successfully`);
    }
  } catch (error) {
    console.error('Challenge save failed:', error);
    throw error;
  }
}

async function main() {
  try {
    const args = Bun.argv.slice(2);

    if (args.length !== 2) {
      throw new ScraperError('Usage: bun run scraper.ts <year> <day>');
    }

    const year = Number(args[0]);
    const day = Number(args[1]);

    if (isNaN(year) || isNaN(day) || day < 1 || day > 25) {
      throw new ScraperError('Year must be a number, day must be between 1 and 25');
    }

    const challenge = await fetchChallengeWithPuppeteer(year, day);
    await saveChallenge(year, day, challenge);

    console.log(`Successfully processed challenge for ${year}/day${day}`);
  } catch (error) {
    console.error('Scraper execution failed:', error);
    process.exit(1);
  }
}

export { fetchChallengeWithPuppeteer, extractChallenges, saveChallenge };

console.log('Starting Rust challenge scraper...');
main().catch(console.error);