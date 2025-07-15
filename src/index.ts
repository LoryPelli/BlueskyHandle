import { Hono } from 'hono';

const app = new Hono();
const did = 'did:plc:jai46evw5qma2hfcrq7mxyjc';

app.get('/.well-known/atproto-did', ({ text }) => text(did));
app.get('*', ({ redirect }) => redirect(`https://bsky.app/profile/${did}`));

export default app;
