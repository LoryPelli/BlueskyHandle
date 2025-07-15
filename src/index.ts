import { Hono } from 'hono';

const app = new Hono();
const did = 'did:plc:jai46evw5qma2hfcrq7mxyjc';

app.get('/', ({ redirect }) => redirect(`https://bsky.app/profile/${did}`));
app.get('/.well-known/atproto-did', ({ text }) => text(did));

export default app;
