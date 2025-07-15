import { Hono } from 'hono';

const app = new Hono();

app.get('/', ({ redirect }) =>
    redirect('https://bsky.app/profile/did:plc:jai46evw5qma2hfcrq7mxyjc'),
);

export default app;
