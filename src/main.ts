import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { loadConfig } from './config';
import { Store } from './store';
import { createServer } from './server';
async function main() {
  const args = process.argv.slice(2);
  if (args.length !== 2 || args[0] !== '--config' || !args[1]) throw new Error('Usage: bun src/main.ts --config <absolute client.private.json>');
  const config = loadConfig(args[1]);
  const store = Store.open(config.databasePath);
  try {
    store.authorize(config, 'read');
    const server = createServer(store, config);
    server.server.onclose = () => store.close();
    const shutdown = async () => { await server.close(); store.close(); process.exit(0); };
    process.once('SIGINT', shutdown); process.once('SIGTERM', shutdown);
    await server.connect(new StdioServerTransport());
  } catch (e) { store.close(); throw e; }
}
main().catch(() => { console.error('M.O.T.H.E.R could not start. Check the client config, grant and initialized database.'); process.exitCode = 1; });
