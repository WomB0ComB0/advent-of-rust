import { $ } from 'bun';

async function main() {
  const year = Number(Bun.argv[2]) || 2023;
  
  if (year < 2023 || year > new Date().getFullYear()) {
    console.error(`Year ${year} is out of range (2023-${new Date().getFullYear()})`);
    return;
  }

  for (const day of Array.from({ length: 25 }, (_, i) => i + 1)) {
    try {
      console.log(`Attempting day ${day} of year ${year}`);
      const result = await $`bun run src/scraper.ts ${year} ${day}`.quiet();
      
      if (!result.exitCode || result.exitCode !== 0) {
        console.error(`Failed on day ${day} with exit code ${result.exitCode}`);
        const errorMessage = result.stderr.toString().split('\n')[0];
        console.error('Error:', errorMessage);
        break;
      }
    } catch (error) {
      if (error instanceof Error) {
        console.error(`Failed to execute scraper for day ${day}:`, error.message);
      } else {
        console.error(`Failed to execute scraper for day ${day}:`, error);
      }
      break;
    }
  }
}

main().catch(console.error);
