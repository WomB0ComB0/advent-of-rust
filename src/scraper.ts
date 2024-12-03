import * as cheerio from 'cheerio';
import { mkdir, writeFile, rm } from 'fs/promises';
import path from 'path';
import puppeteer from 'puppeteer';
import type { Browser, Page } from 'puppeteer';

/**
 * Represents a single Rust challenge with its description and starter code
 * @interface Challenge
 * @property {string} description - The full challenge description text, including problem statement, constraints and examples
 * @property {string} code - The initial Rust code/skeleton that serves as a starting point for solving the challenge
 */
interface Challenge {
  description: string;
  code: string;
}

/**
 * Custom error class for scraper-specific errors to differentiate from general errors
 * @class ScraperError
 * @extends {Error}
 * @property {string} name - Always set to 'ScraperError' to identify error type
 * @property {string} message - Detailed error message explaining what went wrong
 */
class ScraperError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'ScraperError';
  }
}

/**
 * Extracts challenge content from HTML using Cheerio
 * @async
 * @param {string} html - Raw HTML content from the challenge page to be parsed
 * @returns {Promise<Challenge>} Parsed challenge content containing description and starter code
 * @throws {ScraperError} If required content sections (description or code blocks) cannot be found in the HTML
 * @description
 * Uses Cheerio to parse the HTML and extract:
 * 1. Challenge description from the #radix-:r5:-content-description element's text-lg class elements
 * 2. Rust code blocks from pre elements with data-language="rust"
 * The description text is cleaned and formatted with proper paragraph breaks.
 * Only the first valid Rust code block is used as the starter code.
 */
async function extractChallenge(html: string): Promise<Challenge> {
  const $ = cheerio.load(html);
  
  const description = $('.text-lg')
    .map((_, el) => $(el).text().trim())
    .get()
    .join('\n\n')
    .trim();

  if (!description) {
    throw new ScraperError('No description found');
  }
  
  const codeBlocks = $('.view-lines .view-line')
    .map((_, el) => $(el).text().trim())
    .get()
    .join('\n')
    .replace(/\s+\n/g, '\n')
    .replace(/\n{3,}/g, '\n\n')
    .trim();

  if (!codeBlocks) {
    throw new ScraperError('No code blocks found');
  }
  
  return {
    description,
    code: codeBlocks
  };
}

/**
 * Fetches challenge content using Puppeteer by navigating to the challenge page
 * @async
 * @param {number} year - The challenge year (e.g. 2023)
 * @param {number} day - The challenge day number (1-25)
 * @returns {Promise<Challenge>} The fetched challenge content with description and code
 * @throws {Error} If page navigation fails, required elements cannot be found, or content extraction fails
 * @description
 * 1. Launches a Puppeteer browser instance in non-headless mode
 * 2. Navigates to the challenge URL with appropriate timeouts and wait conditions
 * 3. Waits for critical page elements to be visible
 * 4. Extracts page content and parses it using extractChallenge()
 * 5. Handles errors by taking a screenshot and cleaning up resources
 * 6. Always ensures browser is closed properly
 */
async function fetchChallengeWithPuppeteer(year: number, day: number): Promise<Challenge> {
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
    const challenge = await extractChallenge(html);
    await page.close();
    return challenge;
  } catch (error) {
    console.error('Error during page fetch:', error);
    await page.screenshot({ path: 'error-screenshot.png' });
    throw error;
  } finally {
    await browser.close();
  }
}

/**
 * Saves challenge content to the filesystem in a structured format
 * @async
 * @param {number} year - The challenge year (e.g. 2023)
 * @param {number} day - The challenge day number (1-25)
 * @param {Challenge} content - The challenge content to save, containing description and code
 * @throws {Error} If any file system operations fail (directory creation, file writing, etc)
 * @description
 * Creates a directory structure and files for the challenge:
 * - Base directory: <year>/<day>/
 * - README.md: Contains formatted challenge description and code
 * - src/main.rs: Contains just the starter code
 * - Cargo.toml: Basic Rust project configuration
 * 
 * Directory structure:
 * ```
 * <year>/
 *   <day>/
 *     README.md
 *     Cargo.toml
 *     src/
 *       main.rs
 * ```
 * 
 * Process:
 * 1. Removes existing directory if present
 * 2. Creates new directory structure
 * 3. Writes all files concurrently with appropriate permissions
 * 4. Handles any errors during the process
 */
async function saveChallenge(year: number, day: number, content: Challenge) {
  try {
    const baseDir = path.join(process.cwd(), year.toString(), day.toString());
    const srcDir = path.join(baseDir, 'src');
    
    // Delete if exists
    try { 
      await rm(baseDir, { recursive: true, force: true });
    } catch {} finally { 
      console.log(`Deleted directory ${baseDir}`);
    }

    // Create both directories
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

    // Now write files after directories are created
    await Promise.all([
      writeFile(path.join(baseDir, 'README.md'), questionContent, { mode: 0o644 }),
      writeFile(path.join(srcDir, 'main.rs'), content.code, { mode: 0o644 }),
      writeFile(path.join(baseDir, 'Cargo.toml'), 
        `[package]\nname = "aor-${year}-day-${day}"\nversion = "0.1.0"\nedition = "2024"\n`, 
        { mode: 0o644 }
      )
    ]);
    
    console.log(`Challenge for day ${day} saved successfully`);
  } catch (error) {
    console.error('Challenge save failed:', error);
    throw error;
  }
}

/**
 * Main execution function that orchestrates the entire scraping process
 * @async
 * @description 
 * Entry point for the scraper that:
 * 1. Parses and validates command line arguments
 * 2. Fetches challenge content using Puppeteer
 * 3. Saves the challenge content to the filesystem
 * 4. Handles any errors during the process
 * 
 * Usage: bun run scraper.ts <year> <day>
 * Example: bun run scraper.ts 2023 1
 * 
 * @throws {ScraperError} If command line arguments are invalid or missing
 * @throws {Error} If any part of the scraping or saving process fails
 */
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

export { fetchChallengeWithPuppeteer, extractChallenge, saveChallenge };

console.log('Starting Rust challenge scraper...');
main().catch(console.error);